use crate::{Error, GateIO, model::*, util::*};
use crate::set_query;
use reqwest::Url;

impl GateIO {
    pub async fn spot_accounts(&self, currency: Option<&str>) -> Result<Vec<SpotAccount>, Error> {
        let mut url = Url::parse(&format!("{}/spot/accounts", API_URL)).unwrap();
        set_query!(url, currency);
        let req = self.client.get(url).build()?;
        
        send_signed(self, req).await
    }

    pub async fn place_order(&self, order: OrderRequest) -> Result<Order, Error> {
        let req = self.client.post(format!("{}/spot/orders", API_URL)).json(&order);

        send_signed(self, req.build()?).await
    }
}
