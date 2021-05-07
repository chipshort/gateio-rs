#[cfg(test)]
mod tests {
    use http_client::hyper::HyperClient;
    // use gateio::spot::market::list_currencies;
    use tokio_compat_02::FutureExt;
    use std::sync::Arc;
    use serde_json::from_str;

    fn unauthenticated() -> gateio::GateIO<HyperClient> {
        let client = http_client::hyper::HyperClient::new();
        gateio::GateIO::new(Arc::new(client), None)
    }

    fn authenticated() -> gateio::GateIO<HyperClient> {
        let client = http_client::hyper::HyperClient::new();
        gateio::GateIO::new(Arc::new(client), Some(from_str(include_str!("../api_key.json")).unwrap()))
    }

    #[tokio::test]
    async fn market_currencies() {
        let api = unauthenticated();

        let result = api.list_currencies().compat().await;
        
        assert!(result.is_ok(), "could not get currencies");
        assert!(result.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn spot_accounts() {
        let api = authenticated();

        let result = api.spot_accounts(None).compat().await;
        assert!(result.is_ok(), "could not get spot accounts");
        let result = api.spot_accounts(Some("BTC")).compat().await;
        assert!(result.is_ok(), "could not get spot accounts");
        assert!(result.unwrap().len() == 1);
    }

    #[tokio::test]
    async fn market_candlesticks() {
        let api = unauthenticated();

        let result = api.candlesticks("BTC_USDT", None, None, None, None).compat().await;

        assert!(result.is_ok(), "could not get candlesticks for BTC_USDT");
        assert!(result.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn market_orderbook() {
        let api = unauthenticated();

        let result = api.order_book("BTC_USDT", None, None, None).compat().await;

        assert!(result.is_ok(), "could not get candlesticks for BTC_USDT");
    }
}
