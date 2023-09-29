use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SupportedCurrency {
	USDCETH,
	USDCTRC,
	USDTETH,
	USDTTRC,
}

impl SupportedCurrency {
	pub fn get_all() -> Vec<String> {
		vec![
			"USDCETH".to_string(),
			"USDTETH".to_string(),
			"USDTTRC".to_string(),
		]
	}
	pub fn to_str(&self) -> String {
		match self {
			// USDC
			SupportedCurrency::USDCETH => "USDCETH".to_string(),
			SupportedCurrency::USDCTRC => "USDCTRC".to_string(),
			// USDT
			SupportedCurrency::USDTETH => "USDTETH".to_string(),
			SupportedCurrency::USDTTRC => "USDTTRC".to_string(),
		}
	}

	pub fn get_contract_address(&self) -> String {
		match self {
			SupportedCurrency::USDCETH => {
				"0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string()
			}
			SupportedCurrency::USDCTRC => {
				"TEkxiTehnzSmSe2XqrBj4w32RUN966rdz8".to_string()
			}
			SupportedCurrency::USDTETH => {
				"0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string()
			}
			SupportedCurrency::USDTTRC => {
				"TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string()
			}
		}
	}
}
