use std::sync::Arc;

use gateio::model::OrderRequest;
use gateio::GateIO;
use serde_json::from_str;
use tokio_compat_02::FutureExt;

#[tokio::main]
async fn main() -> Result<(), gateio::Error> {
    let client = Arc::new(http_client::hyper::HyperClient::new());
    let unauthenticated = GateIO::new(client.clone(), None);

    // let currencies = unauthenticated.list_currencies().compat().await?;
    // println!("{:?}", currencies);

    // let candlesticks = unauthenticated
    //     .candlesticks("BTC_USDT", None, None, None, Some(gateio::model::Interval::OneMin))
    //     .compat()
    //     .await;
    // println!("{:?}", candlesticks);

    // let orderbook = unauthenticated
    //     .order_book("BTC_USDT", None, None, None)
    //     .compat()
    //     .await;
    // println!("{:?}", orderbook);

    // let authenticated = GateIO::new(
    //     client,
    //     Some(from_str(include_str!("../api_key.json")).unwrap()),
    // );

    // let accounts = authenticated.spot_accounts(None).compat().await?;
    // println!("{:?}", accounts);

    // let order = authenticated
    //     .place_order(
    //         OrderRequest::builder()
    //             .currency_pair("BTC_USDT".into())
    //             .side("buy".into())
    //             .amount(1f64)
    //             .price(1f64)
    //             .build(),
    //     )
    //     .compat()
    //     .await;
    // println!("{:?}", order);

    Ok(())
}
