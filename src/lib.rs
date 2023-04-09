// Winston is a command line chatGPT client
//
// Winston utilizes the clap library to parse command line arguments and
// implement subcommands using a structopt-like syntax and a builder pattern.
// Winston requires an OpenAI API key to function; this key can be provided via
// environment variable, command line argument, or a configuration file. Winston
// will first attempt to parse the key from the command line. If the key is not
// set via command line, it will check the environment variable OPENAI_API_KEY. If
// the key is not set via environment variable, it will check the configuration
// file located at XDG_CONFIG_HOME/winston/config.toml, if XDG_CONFIG_HOME is not
// set, it will check $HOME/.config/winston/config.toml. If no key is found, winston
// will exit with a KeyNotFound error.

use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const OPENAI_ENDPOINT: &str = "https://api.openai.com";
pub const OPENAI_CHAT: &str = "/v1/chat/completions";
pub const MODEL: &str = "davinci";
pub const MAX_TOKENS: u32 = 64;
pub const TEMPERATURE: f32 = 0.9;
pub const TOP_P: f32 = 1.0;
pub const FREQUENCY_PENALTY: f32 = 0.0;
pub const PRESENCE_PENALTY: f32 = 0.0;
pub const STOP: &str = "\n";

#[derive(Debug, Deserialize)]
pub struct WinstonConfig {
    pub openai_api_key: String,
    pub api_endpoint: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub stop: String,
}

#[derive(Debug, Deserialize)]
pub struct WinstonConfigBuilder {
    pub openai_api_key: Option<String>,
    pub api_endpoint: Option<String>,
    pub model: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub stop: Option<String>,
}

impl WinstonConfigBuilder {
    pub fn new() -> Self {
        Self {
            openai_api_key: None,
            api_endpoint: None,
            model: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stop: None,
        }
    }

    pub fn openai_api_key(mut self, key: String) -> Self {
        self.openai_api_key = Some(key);
        self
    }

    pub fn api_endpoint(mut self, endpoint: String) -> Self {
        self.api_endpoint = Some(endpoint);
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn stop(mut self, stop: String) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn build(self) -> Result<WinstonConfig> {
        let openai_api_key = self.openai_api_key.unwrap();
        let api_endpoint = self.api_endpoint.unwrap_or_else(|| OPENAI_ENDPOINT.to_string());
        let model = self.model.unwrap_or_else(|| MODEL.to_string());
        let max_tokens = self.max_tokens.unwrap_or_else(|| MAX_TOKENS);
        let temperature = self.temperature.unwrap_or_else(|| TEMPERATURE);
        let top_p = self.top_p.unwrap_or_else(|| TOP_P);
        let frequency_penalty = self.frequency_penalty.unwrap_or_else(|| FREQUENCY_PENALTY);
        let presence_penalty = self.presence_penalty.unwrap_or_else(|| PRESENCE_PENALTY);
        let stop = self.stop.unwrap_or_else(|| STOP.to_string());
        
        Ok(WinstonConfig {
            openai_api_key,
            api_endpoint,
            model,
            max_tokens,
            temperature,
            top_p,
            frequency_penalty,
            presence_penalty,
            stop,
        })
    }
}
