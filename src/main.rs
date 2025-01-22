// src/main.rs
use chrono::{DateTime, Utc};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{error::Error, io};
use tokio::sync::mpsc;
use windows::{
    Services::Store::{StoreContext, StoreProduct},
    Storage::{ApplicationData, StorageFile},
    System::Launcher,
    core::HSTRING,
};

// Config Module
#[derive(Clone, Debug)]
struct Config {
    api_base_url: String,
    store_product_id: String,
}

impl Config {
    fn new() -> Self {
        dotenv::dotenv().ok();
        Self {
            api_base_url: std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "https://smart-terminal-api-prod.azurewebsites.net".to_string()),
            store_product_id: std::env::var("STORE_PRODUCT_ID")
                .expect("STORE_PRODUCT_ID must be set"),
        }
    }
}

// Licensing Module
#[derive(Debug, Clone, Serialize, Deserialize)]
enum SubscriptionTier {
    Free,
    Basic,
    Pro,
    Enterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LicenseInfo {
    tier: SubscriptionTier,
    monthly_quota: u32,
    used_quota: u32,
    expiration_date: DateTime<Utc>,
}

struct LicenseManager {
    store_context: StoreContext,
    device_id: String,
    current_license: Option<LicenseInfo>,
}

impl LicenseManager {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let store_context = StoreContext::GetDefault()?;
        let device_info = windows::Security::ExchangeActiveSyncProvisioning::EasClientDeviceInformation::new()?;
        let device_id = device_info.Id()?.to_string();
        
        Ok(Self {
            store_context,
            device_id,
            current_license: None,
        })
    }

    async fn check_license(&mut self) -> Result<bool, Box<dyn Error>> {
        let result = self.store_context.GetStoreProductForCurrentAppAsync()?.await?;
        if let Some(product) = result {
            let license = product.License()?;
            if license.IsActive()? {
                self.current_license = Some(self.get_license_info(&product).await?);
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn get_license_info(&self, product: &StoreProduct) -> Result<LicenseInfo, Box<dyn Error>> {
        let sku_id = product.StoreId()?.to_string();
        let tier = match sku_id.as_str() {
            "SmartTerminal.Basic" => SubscriptionTier::Basic,
            "SmartTerminal.Pro" => SubscriptionTier::Pro,
            "SmartTerminal.Enterprise" => SubscriptionTier::Enterprise,
            _ => SubscriptionTier::Free,
        };

        let monthly_quota = match tier {
            SubscriptionTier::Free => 50,
            SubscriptionTier::Basic => 500,
            SubscriptionTier::Pro => 2000,
            SubscriptionTier::Enterprise => 10000,
        };

        Ok(LicenseInfo {
            tier,
            monthly_quota,
            used_quota: 0,
            expiration_date: Utc::now() + chrono::Duration::days(30),
        })
    }

    async fn get_store_token(&self) -> Result<String, Box<dyn Error>> {
        let token = self.store_context.GetCustomerCollectionsIdAsync()?.await?;
        Ok(token.to_string())
    }

    fn get_user_id(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.device_id.clone())
    }
}

// API Module
#[derive(Debug, Serialize)]
struct CompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct CompletionResponse {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

struct ApiClient {
    client: Client,
    config: Config,
    license_manager: LicenseManager,
}

impl ApiClient {
    fn new(config: Config, license_manager: LicenseManager) -> Self {
        Self {
            client: Client::new(),
            config,
            license_manager,
        }
    }

    async fn get_completion(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let store_token = self.license_manager.get_store_token().await?;
        let user_id = self.license_manager.get_user_id()?;

        let request = CompletionRequest {
            model: "claude-3-opus-20240229".to_string(),
            messages: vec![ChatMessage {
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
            _ => Err(format!("Error: {}", response.status()).into()),
        }
    }
}

// Application Module
#[derive(Clone, Debug)]
struct Message {
    content: String,
    is_user: bool,
    timestamp: DateTime<Utc>,
}

enum InputMode {
    Normal,
    Editing,
}

enum AppEvent {
    Input(KeyEvent),
    ApiResponse(Result<String, String>),
    LicenseUpdate(bool),
    Tick,
}

struct App {
    input: String,
    input_mode: InputMode,
    cursor_position: usize,
    messages: Vec<Message>,
    command_history: Vec<String>,
    history_index: Option<usize>,
    scroll_offset: u16,
    
    api_client: ApiClient,
    config: Config,
    license_manager: LicenseManager,
    
    loading: bool,
    error_message: Option<String>,
    status_message: Option<String>,
}

impl App {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::new();
        let license_manager = LicenseManager::new().await?;
        let api_client = ApiClient::new(config.clone(), license_manager.clone());
        
        Ok(Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            cursor_position: 0,
            messages: Vec::new(),
            command_history: Vec::new(),
            history_index: None,
            scroll_offset: 0,
            
            api_client,
            config,
            license_manager,
            
            loading: false,
            error_message: None,
            status_message: None,
        })
    }

    async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        self.setup().await?;
        
        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                if self.handle_input(key).await? {
                    break;
                }
            }
        }

        Ok(())
    }

    // Include all the methods from the previous App implementation...
    // [Previous methods remain exactly the same]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new().await?;
    let res = app.run(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}
