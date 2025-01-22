use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CompletionResponse {
    pub content: Vec<Content>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

pub struct ApiClient {
    client: Client,
    config: Config,
    license_manager: LicenseManager,
}

impl ApiClient {
    pub fn new(config: Config, license_manager: LicenseManager) -> Self {
        Self {
            client: Client::new(),
            config,
            license_manager,
        }
    }

    pub async fn get_completion(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        // Check license and get tokens
        let store_token = self.license_manager.get_store_token().await?;
        let user_id = self.license_manager.get_user_id()?;

        let request = CompletionRequest {
            model: "claude-3-opus-20240229".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: 1024,
        };

        let response = self.client
            .post(format!("{}/api/completion", self.config.api_base_url))
            .header("x-store-token", store_token)
            .header("x-user-id", user_id)
            .json(&request)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let completion: CompletionResponse = response.json().await?;
                Ok(completion.content[0].text.clone())
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err("Monthly quota exceeded. Please upgrade your subscription.".into())
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                // Trigger license refresh
                self.license_manager.refresh_license().await?;
                Err("Please check your subscription status.".into())
            }
            _ => Err(format!("Unexpected error: {}", response.status()).into()),
        }
    }
}
