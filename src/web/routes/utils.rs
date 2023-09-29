use axum::{
	response::{Html, IntoResponse},
	routing::post,
	Router,
};
use qrcode::{render::svg, QrCode};
use reqwest::StatusCode;

use crate::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
	Router::new().route("/api/qrcode", post(api_qrcode_handler))
}

async fn api_qrcode_handler(data: String) -> impl IntoResponse {
	let code = QrCode::new(data).unwrap();
	let svg = code
		.render()
		.min_dimensions(200, 200)
		.dark_color(svg::Color("#000000"))
		.light_color(svg::Color("#FFFFFF"))
		.build();

	Html(svg)
}
