use crate::{model::*, set_query, util::*, Error, GateIO};
use reqwest::Url;

impl GateIO {
    pub async fn deposit_address(&self, currency: &str) -> Result<DepositAddress, Error> {
        let mut url = Url::parse(&format!("{}/wallet/deposit_address", API_URL)).unwrap();
        set_query!(url, currency);
        let req = self.client.get(url).build()?;
        
        send_signed(self, req).await
    }
}