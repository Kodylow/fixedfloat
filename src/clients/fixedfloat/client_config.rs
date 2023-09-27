#[derive(Debug, Clone)]
pub struct FixedFloatClientConfig {
    pub base_url: String,
    pub api_key: String,
    pub api_secret: String,
}

impl FixedFloatClientConfig {
    pub async fn new(api_key: String, api_secret: String) -> Self {
        let config = FixedFloatClientConfig {
            base_url: "https://fixedfloat.com/api/v2".to_string(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
        };
        config
    }

    pub fn extend_base_url(&mut self, path: &str) {
        self.base_url = format!("{}/{}", self.base_url, path);
    }
}
