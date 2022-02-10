use rust_decimal::Decimal;
use gateio::model::{OrderRequest, OrderSide};
use gateio::GateIO;
use serde_json::from_str;

#[tokio::main]
async fn main() -> Result<(), gateio::Error> {
    let unauthenticated = GateIO::new(None);

    currencies(&unauthenticated).await;
    candlesticks(&unauthenticated).await;
    orderbook(&unauthenticated).await;

    let authenticated = GateIO::new(Some(from_str(include_str!("../api_key.json")).unwrap()));
    accounts(&authenticated).await;
    // place_order(&authenticated).await;

    Ok(())
}

async fn currencies(unauthenticated: &GateIO) -> () {
    let currencies = unauthenticated.list_currencies().await;
    println!("{:#?}", currencies);
}

async fn candlesticks(unauthenticated: &GateIO) -> () {
    let candlesticks = unauthenticated
        .candlesticks(
            "BTC_USDT",
            None,
            None,
            None,
            Some(gateio::model::Interval::OneMin),
        )
        .await;
    println!("{:#?}", candlesticks);
}

async fn orderbook(unauthenticated: &GateIO) -> () {
    let orderbook = unauthenticated
        .order_book("BTC_USDT", None, None, None)
        .await;
    println!("{:#?}", orderbook);
}

async fn accounts(authenticated: &GateIO) -> () {
    let accounts = authenticated.spot_accounts(None).await;
    println!("{:#?}", accounts);
}

async fn place_order(authenticated: &GateIO) -> () {
    let order = authenticated
        .place_order(
            OrderRequest::builder()
                .currency_pair("BTC_USDT".into())
                .side(OrderSide::Buy)
                .amount(1.into())
                .price(1.into())
                .build(),
        )
        .await;
    println!("{:#?}", order);
}
