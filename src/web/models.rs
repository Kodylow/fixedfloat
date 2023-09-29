use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeRateRequest {
	pub ccy: String,
	pub direction: String,
	pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrderRequest {
	pub ccy: String,
	pub direction: String,
	pub amount: String,
	#[serde(rename = "toAddress")]
	pub to_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderDetailsRequest {
	pub id: String,
	pub token: String,
}
