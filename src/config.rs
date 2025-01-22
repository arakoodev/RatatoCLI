use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_base_url: String,
    pub store_client_id: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_base_url: env::var("API_BASE_URL")
                .unwrap_or_else(|_| "https://smart-terminal-api-prod.azurewebsites.net".to_string()),
            store_client_id: env::var("STORE_CLIENT_ID")
                .expect("STORE_CLIENT_ID must be set"),
        }
    }
}
