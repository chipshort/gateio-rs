use crate::{model::*, set_query, util::*, Error, GateIO};
use reqwest::Url;
use serde_json::Value;

impl GateIO {
    pub async fn list_currencies(&self) -> Result<Vec<Currency>, Error> {
        let req = self.client.get(format!("{}/spot/currencies", API_URL));
        send_request(self, req.build()?).await
    }

    pub async fn candlesticks(
        &self,
        currency_pair: &str,
        limit: Option<i32>,
        from: Option<i64>,
        to: Option<i64>,
        interval: Option<Interval>,
    ) -> Result<Vec<Candlestick>, Error> {
        let mut url = Url::parse(&format!("{}/spot/candlesticks", API_URL)).unwrap();
        set_query!(url, currency_pair, limit, from, to, interval);
        let req = self.client.get(url).build()?;

        let candlesticks: Result<Vec<Vec<Value>>, Error> = send_request(self, req).await;

        let conversions: Result<Vec<Candlestick>, _> = candlesticks?
            .iter()
            .map(|c| -> Result<Candlestick, Error> {
                Ok(Candlestick {
                    timestamp: parse_from_str(&c[0])?,
                    trading_volume: parse_from_str(&c[1])?,
                    close_price: parse_from_str(&c[2])?,
                    highest_price: parse_from_str(&c[3])?,
                    lowest_price: parse_from_str(&c[4])?,
                    open_price: parse_from_str(&c[5])?,
                })
            })
            .collect();
        Ok(conversions?)
    }

    pub async fn order_book(
        &self,
        currency_pair: &str,
        interval: Option<&str>,
        limit: Option<i32>,
        with_id: Option<bool>,
    ) -> Result<Orderbook, Error> {
        let mut url = Url::parse(&format!("{}/spot/order_book", API_URL)).unwrap();
        set_query!(url, currency_pair, interval, limit, with_id);

        let book: Value = send_request(self, self.client.get(url).build()?).await?;

        println!("{}", book);

        let conversion = Orderbook {
            id: book.get("id").map(|v| v.as_i64()).flatten(),
            asks: convert_orderbook_entries(&book["asks"])?,
            bids: convert_orderbook_entries(&book["bids"])?,
        };

        Ok(conversion)
    }
}

fn convert_orderbook_entries(value: &Value) -> Result<Vec<OrderbookEntry>, Error> {
    let entries = value.as_array().ok_or(Error::ParseError)?;

    let conversions: Result<Vec<OrderbookEntry>, _> = entries
        .iter()
        .map(|v| -> Result<OrderbookEntry, Error> {
            Ok(OrderbookEntry {
                price: parse_from_str(&v[0])?,
                amount: parse_from_str(&v[1])?,
            })
        })
        .collect();
    conversions
}
