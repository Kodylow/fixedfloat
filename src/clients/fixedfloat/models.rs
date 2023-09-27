use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Currency {
    code: String,
    coin: String,
    network: String,
    name: String,
    recv: u8,
    send: u8,
    tag: Option<String>,
    logo: String,
    color: String,
    priority: u8,
}

#[derive(Debug, Deserialize)]
pub struct CurrencyResponse {
    code: i32,
    msg: String,
    data: Vec<Currency>,
}

#[derive(Serialize)]
pub struct ExchangeRateRequest {
    #[serde(rename = "type")]
    pub order_type: String,
    pub fromCcy: String,
    pub toCcy: String,
    pub direction: String,
    pub amount: f64,
    pub ccies: Option<bool>,
    pub usd: Option<bool>,
    pub refcode: Option<String>,
    pub afftax: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub code: String,
    pub network: String,
    pub coin: String,
    pub amount: String,
    pub rate: String,
    pub precision: i32,
    pub min: String,
    pub max: String,
    pub usd: String,
    pub btc: String,
}

#[derive(Debug, Deserialize)]
pub struct Ccy {
    pub code: String,
    pub recv: bool,
    pub send: bool,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeRateResponseData {
    pub from: Asset,
    pub to: Asset,
    pub errors: Vec<String>,
    pub ccies: Option<Vec<Ccy>>,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeRateResponse {
    pub code: i32,
    pub msg: String,
    pub data: ExchangeRateResponseData,
}
