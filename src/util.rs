use crate::{
    model::{Interval, OrderSide},
    Error, GateIO,
};
use hmac::{Hmac, Mac, NewMac};
use reqwest::{header::HeaderValue, Request};
use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::{Digest, Sha512};
use std::{str::FromStr, time::SystemTime};

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
    s.as_str()
        .ok_or(Error::ParseError)?
        .parse()
        .map_err(|_| Error::ParseError)
}

pub async fn send_signed<T: DeserializeOwned>(gateio: &GateIO, req: Request) -> Result<T, Error> {
    let req = sign_request(gateio.key.as_ref().ok_or(Error::AuthRequired)?, req).await?;
    send_request(gateio, req).await
}

pub async fn send_request<T: DeserializeOwned>(
    gateio: &GateIO,
    mut req: Request,
) -> Result<T, Error> {
    req.headers_mut()
        .insert("Content-Type", HeaderValue::from_static("application/json"));
    req.headers_mut()
        .insert("Accept", HeaderValue::from_static("application/json"));

    let response = gateio.client.execute(req).await?;

    if response.status().is_success() {
        // println!("{:?}", response.body_string().await);
        Ok(response.json().await?)
    } else {
        Err(Error::GateIO(response.json().await?))
    }
}

pub async fn sign_request(key: &crate::ApiKey, mut req: Request) -> Result<Request, Error> {
    let method = req.method().to_string().to_uppercase();
    let url = req.url().path().to_string();
    let query_str = urldecode::decode(req.url().query().unwrap_or("").to_string());
    let body = req
        .body()
        .map(|b| b.as_bytes())
        .flatten()
        .unwrap_or("".as_bytes());
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
        hex::encode(Sha512::digest(body)),
        timestamp
    );

    // println!("{}", request_payload);
    // println!("{}", signature_string);

    let mut hmac =
        Hmac::<Sha512>::new_from_slice(key.secret_key.as_bytes()).map_err(|_| Error::InvalidKey)?;
    hmac.update(signature_string.as_bytes());
    let signature = hex::encode(hmac.finalize().into_bytes());

    // println!("{}", signature);

    req.headers_mut().insert("KEY", HeaderValue::from_str(&key.api_key)?);
    req.headers_mut().insert("SIGN", HeaderValue::from_str(&signature)?);
    req.headers_mut().insert("Timestamp", timestamp.into());

    Ok(req)
    // Err(Error::InvalidKey)
}
