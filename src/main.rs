#![allow(unused)] // For early development.

// region:    --- Modules

mod clients;
mod config;
mod ctx;
mod log;
mod model;
mod utils;
mod web;

pub use anyhow::{Error, Result};
pub use config::config;

use crate::model::ModelManager;
use crate::web::routes;

use axum::response::Html;
use axum::routing::get;
use axum::{middleware, Router};
use clients::fixedfloat::client::FixedFloat;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time() // For early local development.
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let fixed_float = FixedFloat::new(
		&config().FIXEDFLOAT_API_KEY,
		&config().FIXEDFLOAT_API_SECRET,
	)
	.await;

	let currencies = fixed_float.get_available_currencies().await.unwrap();
	println!("{:?}", currencies);
	panic!();

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	let routes_all = Router::new()
		.merge(routes::fixedfloat::routes(mm.clone()))
		.merge(routes::utils::routes(mm.clone()))
		// .layer(middleware::map_response(mw_reponse_map))
		.fallback_service(routes::static_files::serve_dir());

	// region:    --- Start Server
	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	info!("{:<12} - {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}

// // Initialize the FixedFloat client
// let fixed_float = FixedFloat::new(
// 	config().FIXEDFLOAT_API_KEY.to_string(),
// 	config().FIXEDFLOAT_API_SECRET.to_string(),
// )
// .await;

// Get the available currencies
// let currencies = fixed_float.get_available_currencies().await.unwrap();
// println!("{:?}", currencies);

// let exchange_rate_usdceth_to_btcln = fixed_float
// 	.get_exchange_rate_usdceth_to_btcln("1.0".to_string())
// 	.await
// 	.unwrap();
// println!(
// 	"Exchange Rate USDCETH -> BTCLN {:?}",
// 	exchange_rate_usdceth_to_btcln
// );
// let exchange_rate_btcln_to_usdceth = fixed_float
// 	.get_exchange_rate_btcln_to_usdceth("1.0".to_string())
// 	.await
// 	.unwrap();
// println!(
// 	"Exchange Rate BTCLN -> USDCETH {:?}",
// 	exchange_rate_btcln_to_usdceth
// );

// let create_order_result = fixed_float
// 	.create_order_btcln_to_usdceth(
// 		"0.001".to_string(),
// 		"0x18241de74E1B36e80f91082ec1Ca1987f4e24d3b",
// 	)
// 	.await
// 	.unwrap();

// println!("Create Order Result {:?}", create_order_result);
