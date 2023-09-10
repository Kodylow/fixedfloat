use super::{invoices, payments};

#[derive(Debug, Clone)]
pub struct FixedFloatClientConfig {
    pub base_url: String,
    pub api_key: String,
    pub api_secret: String,
    pub generated_sig: String,
}

impl FixedFloatClientConfig {
    async fn new(api_key: String, api_secret: String) -> Self {
        let config = AlbyClientConfig {
            base_url: "https://fixedfloat.com/api/v2".to_string(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            generated_sig: generated_sig(api_key, api_secret).await,
        };
        config
    }

    fn generated_sig(data: &str, secret: &str) -> Result<String, openssl::error::ErrorStack> {
        let key = PKey::hmac(secret.as_bytes())?;
        let mut signer = Signer::new(MessageDigest::sha256(), &key)?;
        signer.update(data.as_bytes())?;
        let sign = signer.sign_to_vec()?;

        Ok(hex::encode(sign))
    }

    pub fn extend_base_url(&mut self, path: &str) {
        self.base_url = format!("{}/{}", self.base_url, path);
    }
}

pub struct FixedFloatClient {
    pub config: FixedFloatClientConfig,
    pub client: reqwest::Client,
}

impl FixedFloatClient {
    pub async fn new(token: &str) -> Self {
        let config = FixedFloatClientConfig::new(token).await;
        let client = FixedFloatClient {
            config,
            client: reqwest::Client::new(),
        };
        client
    }

    pub fn invoices(&self) -> invoices::Invoices {
        invoices::Invoices::new(&self.client, &self.config)
    }

    pub fn payments(&self) -> payments::Payments {
        payments::Payments::new(&self.client, &self.config)
    }
}
