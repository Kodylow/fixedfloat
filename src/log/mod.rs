use crate::ctx::Ctx;
use anyhow::Result;
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use uuid::Uuid;

pub async fn log_request(
	uuid: Uuid,
	req_method: Method,
	uri: Uri,
	ctx: Option<Ctx>,
	web_error: Option<anyhow::Error>,
	client_error: Option<anyhow::Error>,
) -> Result<()> {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis();

	let error_type = web_error
		.as_ref()
		.map(|e| e.to_string())
		.or_else(|| client_error.as_ref().map(|e| e.to_string()));
	let error_data = web_error.map(|se| json!(se.to_string()));

	// Create the RequestLogLine
	let log_line = RequestLogLine {
		uuid: uuid.to_string(),
		timestamp: timestamp.to_string(),

		http_path: uri.to_string(),
		http_method: req_method.to_string(),

		user_id: ctx.map(|c| c.user_id()),

		client_error_type: client_error.map(|e| e.to_string()),

		error_type,
		error_data,
	};

	debug!("REQUEST LOG LINE:\n{}", json!(log_line));

	// TODO - Send to cloud-watch.

	Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
	uuid: String,      // uuid string formatted
	timestamp: String, // (should be iso8601)

	// -- User and context attributes.
	user_id: Option<i64>,

	// -- http request attributes.
	http_path: String,
	http_method: String,

	// -- Errors attributes.
	client_error_type: Option<String>,
	error_type: Option<String>,
	error_data: Option<Value>,
}
