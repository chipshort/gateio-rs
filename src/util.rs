use std::{str::FromStr, time::SystemTime};
use crate::{Error, GateIO, error::GateIOError, model::{Interval, OrderSide}};
use hmac::{Hmac, Mac, NewMac};
use http_client::{Body, HttpClient, Request};
use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::{Digest, Sha512};

pub(crate) trait ToStrMarker {}
impl ToStrMarker for i32 {}
impl ToStrMarker for i64 {}
impl ToStrMarker for u32 {}
impl ToStrMarker for u64 {}
impl ToStrMarker for f32 {}
impl ToStrMarker for f64 {}
impl ToStrMarker for String {}
impl ToStrMarker for &str {}
impl ToStrMarker for bool {}
impl ToStrMarker for Interval {}
impl ToStrMarker for OrderSide {}

pub(crate) trait ToOptionString {
    fn to_opt(&self) -> Option<String>;
}
impl<S: ToString + ToStrMarker> ToOptionString for S {
    fn to_opt(&self) -> Option<String> {
        Some(self.to_string())
    }
}
impl<S: ToString + ToStrMarker> ToOptionString for Option<S> {
    fn to_opt(&self) -> Option<String> {
        self.as_ref().map(|s| s.to_string())
    }
}

#[macro_export]
macro_rules! set_query {
    // macth like arm for macro
    ($url:expr, $($key:ident),*)=>{
        {
            let mut query: Vec<String> = vec![];
            $(
                let entry: Option<String> = $crate::util::ToOptionString::to_opt(&$key);
                if let Some(value) = entry {
                    query.push(format!("{}={}", stringify!($key), value));
                }
            )*
            if query.len() > 0 {
                $url.set_query(Some(&query.join("&")));
            }
        }
    }
}

pub fn parse_from_str<T: FromStr>(s: &Value) -> Result<T, Error> {
    s.as_str().ok_or(Error::ParseError)?.parse().map_err(|_| Error::ParseError)
}

pub async fn send_signed<C: HttpClient, T: DeserializeOwned>(
    gateio: &GateIO<C>,
    mut req: Request,
) -> Result<T, Error> {
    sign_request(gateio.key.as_ref().ok_or(Error::AuthRequired)?, &mut req).await?;
    send_request(gateio, req).await
}

pub async fn send_request<C: HttpClient, T: DeserializeOwned>(
    gateio: &GateIO<C>,
    mut req: Request,
) -> Result<T, Error> {
    req.insert_header("Content-Type", "application/json");
    req.insert_header("Accept", "application/json");

    let mut response = gateio.client.send(req).await?;

    if response.status().is_success() {
        // println!("{:?}", response.body_string().await);
        Ok(response.body_json().await?)
    } else {
        Err(Error::GateIO(response.body_json::<GateIOError>().await?))
    }
}

pub async fn sign_request(
    key: &crate::ApiKey,
    req: &mut Request,
) -> Result<(), Error> {
    let method = req.method().to_string().to_uppercase();
    let url = req.url().path().to_string();
    let query_str = urldecode::decode(req.url().query().unwrap_or("").to_string());
    let body = req.take_body();
    let request_payload = body.into_string().await?;
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();
    // let timestamp = 1619910800u64;

    let signature_string = format!(
        "{}\n{}\n{}\n{}\n{}",
        method,
        url,
        query_str,
        hex::encode(Sha512::digest(request_payload.as_bytes())),
        timestamp
    );

    // println!("{}", request_payload);
    // println!("{}", signature_string);

    let mut hmac =
        Hmac::<Sha512>::new_from_slice(key.secret_key.as_bytes()).map_err(|_| Error::InvalidKey)?;
    hmac.update(signature_string.as_bytes());
    let signature = hex::encode(hmac.finalize().into_bytes());

    // println!("{}", signature);

    req.insert_header("KEY", &key.api_key);
    req.insert_header("SIGN", signature);
    req.insert_header("Timestamp", timestamp.to_string());

    req.set_body(Body::from_string(request_payload));

    Ok(())
    // Err(Error::InvalidKey)
}
