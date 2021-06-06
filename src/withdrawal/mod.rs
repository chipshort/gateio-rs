use crate::{model::*, set_query, util::*, Error, GateIO};
use reqwest::Url;

impl GateIO {
    pub async fn withdrawals(&self, currency: &str, amount: f64, address: Option<String>, memo: Option<String>, chain: Option<Chain>) -> Result<DepositAddress, Error> {
        //TODO: test
        let mut url = Url::parse(&format!("{}/withdrawals", API_URL)).unwrap();
        set_query!(url, currency, amount, address, memo, chain);
        let req = self.client.post(url).build()?;
        
        send_signed(self, req).await
    }
}