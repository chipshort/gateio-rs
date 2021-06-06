use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use typed_builder::TypedBuilder;

pub static API_URL: &'static str = "https://api.gateio.ws/api/v4";

#[derive(Deserialize, Debug)]
pub struct Currency {
    pub currency: String,
    pub delisted: bool,
    pub withdraw_disabled: bool,
    pub withdraw_delayed: bool,
    pub deposit_disabled: bool,
    pub trade_disabled: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Debug, TypedBuilder)]
pub struct OrderRequest {
    #[builder(default, setter(strip_option))]
    pub text: Option<String>,
    pub currency_pair: String,
    #[serde(rename = "type")]
    #[builder(default, setter(strip_option))]
    pub order_type: Option<String>,
    #[builder(default, setter(strip_option))]
    pub account: Option<AccountType>,
    pub side: OrderSide,
    #[builder(default, setter(strip_option))]
    pub iceberg: Option<f64>,
    pub amount: f64,
    pub price: f64,
    #[builder(default, setter(strip_option))]
    pub time_in_force: Option<TimeInForce>,
    #[builder(default, setter(strip_option))]
    pub auto_borrow: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Order {
    pub id: String,
    pub text: String,
    pub create_time: String,
    pub update_time: String,
    pub currency_pair: String,
    pub status: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub account: AccountType,
    pub side: String,
    #[serde_as(as = "DisplayFromStr")]
    pub iceberg: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    pub time_in_force: TimeInForce,
    #[serde_as(as = "DisplayFromStr")]
    pub left: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub filled_total: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub fee: f64,
    pub fee_currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub point_fee: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub gt_fee: f64,
    pub gt_discount: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub rebated_fee: f64,
    pub rebated_fee_currency: String,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct SpotAccount {
    pub currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub available: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub locked: f64,
}

#[derive(Debug)]
pub struct Candlestick {
    pub timestamp: u64,
    pub trading_volume: f64,
    pub close_price: f64,
    pub highest_price: f64,
    pub lowest_price: f64,
    pub open_price: f64,
}

#[derive(Debug)]
pub struct Orderbook {
    pub id: Option<i64>,
    pub asks: Vec<OrderbookEntry>,
    pub bids: Vec<OrderbookEntry>,
}

#[derive(Debug)]
pub struct OrderbookEntry {
    pub price: f64,
    pub amount: f64,
}

#[derive(Deserialize, Debug)]
pub struct DepositAddress {
    pub currency: String,
    pub address: String,
    pub multichain_addresses: Vec<MultichainAddress>, //field name is incorrect in the docs
}

#[derive(Deserialize, Debug)]
pub struct MultichainAddress {
    pub chain: String,
    pub address: String,
    pub payment_id: String,
    pub payment_name: String,
    pub obtain_failed: i64,
}

pub enum Chain {
    BTC,
    ETH,
    TRX,
    BSC,
    EOS,
    HT,
    SOL,
}

impl Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chain::BTC => write!(f, "BTC"),
            Chain::ETH => write!(f, "ETH"),
            Chain::TRX => write!(f, "TRX"),
            Chain::BSC => write!(f, "BSC"),
            Chain::EOS => write!(f, "EOS"),
            Chain::HT => write!(f, "HT"),
            Chain::SOL => write!(f, "SOL"),
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub api_key: String,
    pub secret_key: String,
}

pub enum Interval {
    TenSec,
    OneMin,
    FiveMin,
    FifteenMin,
    ThirtyMin,
    OneHour,
    FourHour,
    EightHour,
    OneDay,
    SevenDay,
}
impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::TenSec => write!(f, "10s"),
            Interval::OneMin => write!(f, "1m"),
            Interval::FiveMin => write!(f, "5m"),
            Interval::FifteenMin => write!(f, "15m"),
            Interval::ThirtyMin => write!(f, "30m"),
            Interval::OneHour => write!(f, "1h"),
            Interval::FourHour => write!(f, "4h"),
            Interval::EightHour => write!(f, "8h"),
            Interval::OneDay => write!(f, "1d"),
            Interval::SevenDay => write!(f, "7d"),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Spot,
    Margin,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    GTC,
    IOC,
    POC,
}
