use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyResponse {
	code: i32,
	msg: String,
	data: Vec<Currency>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateRequest {
	#[serde(rename = "type")]
	pub order_type: String,
	pub from_ccy: String,
	pub to_ccy: String,
	pub direction: String,
	pub amount: String,
	pub ccies: Option<bool>,
	pub usd: Option<bool>,
	pub refcode: Option<String>,
	pub afftax: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
	pub code: String,
	pub network: String,
	pub coin: String,
	pub amount: String,
	pub rate: Option<String>,
	pub precision: Option<i32>,
	pub min: Option<String>,
	pub max: Option<String>,
	pub usd: Option<String>,
	pub btc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ccy {
	pub code: String,
	pub recv: bool,
	pub send: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeRateResponseData {
	pub from: Asset,
	pub to: Asset,
	pub ccies: Option<Vec<Ccy>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeRateResponse {
	pub code: i32,
	pub msg: String,
	pub data: ExchangeRateResponseData,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
	#[serde(rename = "type")]
	pub order_type: String,
	pub from_ccy: String,
	pub to_ccy: String,
	pub direction: String,
	pub amount: String,
	pub to_address: String,
	pub tag: Option<String>,
	pub refcode: Option<String>,
	pub afftax: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCurrency {
	pub code: String,
	pub network: Option<String>,
	pub coin: Option<String>,
	pub amount: String,
	pub rate: Option<String>,
	pub precision: Option<i32>,
	pub min: Option<String>,
	pub max: Option<String>,
	pub usd: Option<String>,
	pub btc: Option<String>,
	pub address: Option<String>,
	pub address_alt: Option<String>,
	pub tag: Option<String>,
	pub tag_name: Option<String>,
	pub req_confirmations: Option<i32>,
	pub max_confirmations: Option<i32>,
	pub tx: Option<Transaction>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackCurrency {
	code: String,
	network: Option<String>,
	coin: Option<String>,
	amount: Option<String>,
	alias: Option<String>,
	address: Option<String>,
	tag: Option<String>,
	tag_name: Option<String>,
	tx: Option<Transaction>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
	pub id: Option<String>,
	pub amount: Option<String>,
	pub fee: Option<String>,
	pub ccyfee: Option<String>,
	pub time_reg: Option<i64>,
	pub time_block: Option<i64>,
	pub confirmations: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Time {
	pub reg: i64,
	pub start: Option<i64>,
	pub finish: Option<i64>,
	pub update: i64,
	pub expiration: i64,
	pub left: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Emergency {
	pub status: Vec<String>,
	pub choice: String,
	pub repeat: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderData {
	pub id: String,
	#[serde(rename = "type")]
	pub order_type: String,
	pub email: String,
	pub status: String,
	pub time: Time,
	pub from: OrderCurrency,
	pub to: OrderCurrency,
	pub back: BackCurrency,
	pub emergency: Emergency,
	pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderResponse {
	pub code: i32,
	pub msg: String,
	pub data: OrderData,
}
