use super::error::AppError;
use crate::clients::fixedfloat::models::CreateOrderRequest as FixedFloatCreateOrderRequest;
use crate::clients::FixedFloat;
use crate::config;
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForCreate, UserForInsert, UserForLogin};
use crate::model::ModelManager;
use crate::web::models::{
	CreateOrderRequest, ExchangeRateRequest, OrderDetailsRequest,
};
use anyhow::{anyhow, Result};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::{debug, error, info};

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/currencies", get(api_currencies_handler))
		.route("/api/exchange-rate", post(api_exchange_rate_handler))
		.route("/api/create-order", post(api_create_order_handler))
		.route("/api/order-details", post(api_order_details_handler))
		.with_state(mm)
}

// region:    --- Currencies
#[axum::debug_handler]
pub async fn api_currencies_handler(
	State(mm): State<ModelManager>,
) -> Result<Json<Value>, AppError> {
	debug!("{:<12} - api_currencies_handler", "HANDLER");

	let root_ctx = Ctx::root_ctx();

	let fixedfloat = FixedFloat::new(
		&config().FIXEDFLOAT_API_KEY,
		&config().FIXEDFLOAT_API_SECRET,
	)
	.await;

	let currencies = match fixedfloat.get_available_currencies().await {
		Ok(currencies) => currencies,
		Err(err) => {
			error!("Error: {:?}", err);
			return Err(AppError::from(err));
		}
	};

	Ok(Json(json!(currencies)))
}
// endregion: --- Currencies

// region:    --- Exchange Rate
#[axum::debug_handler]
pub async fn api_exchange_rate_handler(
	State(mm): State<ModelManager>,
	Json(req): Json<ExchangeRateRequest>,
) -> Result<Json<Value>, AppError> {
	debug!("{:<12} - api_exchange_rate_handler", "HANDLER");

	let root_ctx = Ctx::root_ctx();

	let fixedfloat = FixedFloat::new(
		&config().FIXEDFLOAT_API_KEY,
		&config().FIXEDFLOAT_API_SECRET,
	)
	.await;

	let exchange_rate = match req.direction.as_str() {
		"from" => {
			fixedfloat
				.get_exchange_rate(
					"fixed",
					&req.ccy,
					"BTCLN",
					"from",
					&req.amount,
					None,
					None,
					None,
					None,
				)
				.await
		}
		"to" => {
			fixedfloat
				.get_exchange_rate(
					"fixed",
					"BTCLN",
					&req.ccy,
					"to",
					&req.amount,
					None,
					None,
					None,
					None,
				)
				.await
		}
		_ => {
			let err = AppError::from(anyhow!("Invalid direction"));
			error!("Error: {:?}", err);
			return Err(err);
		}
	};

	let exchange_rate = match exchange_rate {
		Ok(exchange_rate) => exchange_rate,
		Err(err) => {
			error!("Error: {:?}", err);
			return Err(AppError::from(err));
		}
	};

	Ok(Json(json!(exchange_rate)))
}

// region:    --- Create Order
#[axum::debug_handler]
pub async fn api_create_order_handler(
	State(mm): State<ModelManager>,
	Json(req): Json<CreateOrderRequest>,
) -> Result<Json<Value>, AppError> {
	debug!("{:<12} - api_create_order_handler", "HANDLER");
	let root_ctx = Ctx::root_ctx();

	info!("req: \n{:?}", req);

	let fixedfloat = FixedFloat::new(
		&config().FIXEDFLOAT_API_KEY,
		&config().FIXEDFLOAT_API_SECRET,
	)
	.await;

	let order_response = match req.direction.as_str() {
		"from" => {
			fixedfloat
				.create_order(
					"fixed".to_string(),
					req.ccy,
					"BTCLN".to_string(),
					"to".to_string(),
					req.amount,
					req.to_address,
					None,
					None,
					None,
				)
				.await
		}
		"to" => {
			fixedfloat
				.create_order(
					"fixed".to_string(),
					"BTCLN".to_string(),
					req.ccy,
					"to".to_string(),
					req.amount,
					req.to_address,
					None,
					None,
					None,
				)
				.await
		}
		_ => {
			let err = AppError::from(anyhow!("Invalid direction"));
			error!("Error: {:?}", err);
			return Err(err);
		}
	};

	let order_response = match order_response {
		Ok(order_response) => order_response,
		Err(err) => {
			error!("Error: {:?}", err);
			return Err(AppError::from(err));
		}
	};

	Ok(Json(json!(order_response)))
}
// endregion: --- Create Order

// region:    --- Order Details
#[axum::debug_handler]
pub async fn api_order_details_handler(
	State(mm): State<ModelManager>,
	Json(req): Json<OrderDetailsRequest>,
) -> Result<Json<Value>, AppError> {
	debug!("{:<12} - api_order_details_handler", "HANDLER");
	let root_ctx = Ctx::root_ctx();

	info!("req: \n{:?}", req);

	let fixedfloat = FixedFloat::new(
		&config().FIXEDFLOAT_API_KEY,
		&config().FIXEDFLOAT_API_SECRET,
	)
	.await;

	let order_response = match fixedfloat.order_details(&req.id, &req.token).await {
		Ok(order) => order,
		Err(err) => {
			error!("Error: {:?}", err);
			return Err(AppError::from(err));
		}
	};

	Ok(Json(json!(order_response)))
}
