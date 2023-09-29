use anyhow::Result;
use axum::http::{HeaderMap, HeaderValue};
use openssl::{hash::MessageDigest, pkey::PKey, sign::Signer};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use tracing::info;

use super::client_config::FixedFloatClientConfig;
use super::models::{
	CreateOrderRequest, CurrencyResponse, ExchangeRateResponse, OrderResponse,
};

pub struct FixedFloat {
	pub config: FixedFloatClientConfig,
	pub client: reqwest::Client,
}

impl FixedFloat {
	pub async fn new(api_key: &str, api_secret: &str) -> Self {
		let config =
			FixedFloatClientConfig::new(api_key.to_string(), api_secret.to_string())
				.await;
		let client = reqwest::Client::new();

		FixedFloat { config, client }
	}

	fn sign(data: &str, secret: &str) -> Result<String, openssl::error::ErrorStack> {
		let key = PKey::hmac(secret.as_bytes())?;
		let mut signer = Signer::new(MessageDigest::sha256(), &key)?;
		signer.update(data.as_bytes())?;
		let sign = signer.sign_to_vec()?;

		Ok(hex::encode(sign))
	}

	pub async fn post(
		&self,
		method: &str,
		data_json: &str,
	) -> Result<String, anyhow::Error> {
		// Generate the signature
		let sig = match Self::sign(data_json, &self.config.api_secret) {
			Ok(sign) => sign,
			Err(e) => return Err(anyhow::Error::new(e)),
		};

		// Create headers
		let headers = self.headers(sig)?;

		// Make the POST request
		let url = format!("{}/{}", self.config.base_url, method);
		let client = &self.client;
		let res = client
			.post(&url)
			.headers(headers)
			.body(String::from(data_json))
			.send()
			.await?;
		// Return the response body as a string
		let body = res.text().await?;
		Ok(body)
	}

	fn headers(&self, sig: String) -> Result<HeaderMap, anyhow::Error> {
		let mut headers = HeaderMap::new();
		headers.insert("X-API-KEY", HeaderValue::from_str(&self.config.api_key)?);
		headers.insert("X-API-SIGN", HeaderValue::from_str(&sig)?);
		headers.insert(
			CONTENT_TYPE,
			HeaderValue::from_static("application/json; charset=UTF-8"),
		);
		headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
		Ok(headers)
	}
}

impl FixedFloat {
	pub async fn get_available_currencies(&self) -> Result<CurrencyResponse> {
		let method = "ccies";
		let data_json = "{}";
		let response = self.post(method, data_json).await?;
		let currencies: CurrencyResponse = serde_json::from_str(&response)?;
		Ok(currencies)
	}

	pub async fn get_exchange_rate(
		&self,
		order_type: &str,
		from_ccy: &str,
		to_ccy: &str,
		direction: &str,
		amount: &str,
		ccies: Option<bool>,
		usd: Option<bool>,
		refcode: Option<String>,
		afftax: Option<f64>,
	) -> Result<ExchangeRateResponse> {
		let method = "price";
		let mut data = serde_json::json!({
			"type": order_type,
			"fromCcy": from_ccy,
			"toCcy": to_ccy,
			"direction": direction,
			"amount": amount,
		});

		if let Some(ccies) = ccies {
			data["ccies"] = ccies.into();
		}

		if let Some(usd) = usd {
			data["usd"] = usd.into();
		}

		if let Some(refcode) = refcode {
			data["refcode"] = refcode.into();
		}

		if let Some(afftax) = afftax {
			data["afftax"] = afftax.into();
		}

		let data_json = data.to_string();
		let response = self.post(method, &data_json).await?;
		info!("response: {}", response);
		let exchange_rate: ExchangeRateResponse = serde_json::from_str(&response)?;

		Ok(exchange_rate)
	}

	pub async fn create_order(
		&self,
		order_type: String,
		from_ccy: String,
		to_ccy: String,
		direction: String,
		amount: String,
		to_address: String,
		tag: Option<String>,
		refcode: Option<String>,
		afftax: Option<f64>,
	) -> Result<OrderResponse> {
		let method = "create";
		let data = CreateOrderRequest {
			order_type,
			from_ccy,
			to_ccy,
			direction,
			amount,
			to_address,
			tag,
			refcode,
			afftax,
		};
		let data_json = serde_json::to_string(&data)?;
		let response = self.post(method, &data_json).await?;
		info!("response: {}", response);
		let order: OrderResponse = serde_json::from_str(&response)?;

		Ok(order)
	}

	pub async fn order_details(
		&self,
		order_id: &str,
		token: &str,
	) -> Result<OrderResponse> {
		let method = "order";
		let data = serde_json::json!({
			"id": order_id,
			"token": token,
		});
		let data_json = data.to_string();
		let response = self.post(method, &data_json).await?;
		info!("response: {}", response);
		let order: OrderResponse = serde_json::from_str(&response)?;

		Ok(order)
	}
}
