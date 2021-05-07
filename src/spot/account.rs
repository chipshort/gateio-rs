use crate::{Error, GateIO, model::*, util::*};
use http_client::http_types::Url;
use http_client::{HttpClient, Request};
use crate::set_query;

impl<C: HttpClient> GateIO<C> {
    pub async fn spot_accounts(&self, currency: Option<&str>) -> Result<Vec<SpotAccount>, Error> {
        let mut url = Url::parse(&format!("{}/spot/accounts", API_URL)).unwrap();
        set_query!(url, currency);
        
        send_signed(self, Request::get(url)).await
    }

    pub async fn place_order(&self, order: OrderRequest) -> Result<Order, Error> {
        let mut req = Request::post(Url::parse(&format!("{}/spot/orders", API_URL)).unwrap());
        req.set_body(serde_json::to_string(&order).unwrap());

        send_signed(self, req).await
    }
}
