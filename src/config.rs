use crate::{Error, Result};
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct Config {
	// -- Crypt
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,

	// -- FixedFloat
	pub FIXEDFLOAT_API_KEY: String,
	pub FIXEDFLOAT_API_SECRET: String,

	// -- Db
	pub DB_URL: String,

	// -- Web
	pub WEB_FOLDER: String,
}

impl Config {
	fn load_from_env() -> Result<Config> {
		Ok(Config {
			// -- Crypt
			PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,
			TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
			TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,

			// -- FixedFloat
			FIXEDFLOAT_API_KEY: get_env("SERVICE_FIXEDFLOAT_API_KEY")?,
			FIXEDFLOAT_API_SECRET: get_env("SERVICE_FIXEDFLOAT_API_SECRET")?,

			// -- Db
			DB_URL: get_env("SERVICE_DB_URL")?,

			// -- Web
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}

fn get_env(name: &'static str) -> Result<String> {
	env::var(name).map_err(|_| anyhow::anyhow!("Missing env var: {name}"))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
	let val = get_env(name)?;
	val.parse::<T>()
		.map_err(|_| anyhow::anyhow!("Fail to parse env var: {name}"))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
	base64_url::decode(&get_env(name)?)
		.map_err(|_| anyhow::anyhow!("Fail to decode b64u env var: {name}"))
}
