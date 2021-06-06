#[cfg(test)]
mod util;

#[cfg(test)]
mod tests {
    use super::util::*;

    #[tokio::test]
    async fn wallet_deposit_address() {
        let api = authenticated();

        let result = api.deposit_address("USDT").await;
        assert!(result.is_ok(), "could not get deposit addresses for USDT");

        let single_chain = api.deposit_address("DAI").await;
        assert!(single_chain.is_ok(), "could not get deposit addresses for DAI");
    }
}
