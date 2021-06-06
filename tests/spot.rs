#[cfg(test)]
mod util;

#[cfg(test)]
mod tests {
    use super::util::*;

    #[tokio::test]
    async fn spot_accounts() {
        let api = authenticated();

        let result = api.spot_accounts(None).await;
        assert!(result.is_ok(), "could not get spot accounts");
        let result = api.spot_accounts(Some("BTC")).await;
        assert!(result.is_ok(), "could not get spot accounts");
        assert!(result.unwrap().len() == 1);
    }

    #[tokio::test]
    async fn market_currencies() {
        let api = unauthenticated();

        let result = api.list_currencies().await;
        
        assert!(result.is_ok(), "could not get currencies");
        assert!(result.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn market_candlesticks() {
        let api = unauthenticated();

        let result = api.candlesticks("BTC_USDT", None, None, None, None).await;

        assert!(result.is_ok(), "could not get candlesticks for BTC_USDT");
        assert!(result.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn market_orderbook() {
        let api = unauthenticated();

        let result = api.order_book("BTC_USDT", None, None, None).await;

        assert!(result.is_ok(), "could not get candlesticks for BTC_USDT");
    }
}
