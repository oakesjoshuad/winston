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

use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use super::Result;

pub const OPENAI_ENDPOINT: &str = "https://api.openai.com";
#[allow(dead_code)]
pub const OPENAI_CHAT: &str = "/v1/chat/completions";
pub const MODEL: &str = "davinci";
pub const MAX_TOKENS: u32 = 64;
pub const TEMPERATURE: f32 = 0.9;
pub const TOP_P: f32 = 1.0;
pub const FREQUENCY_PENALTY: f32 = 0.0;
pub const PRESENCE_PENALTY: f32 = 0.0;
pub const STOP: &str = "\n";

#[derive(Debug, Deserialize, Serialize)]
pub struct WinstonConfig {
    pub openai_org_id: String,
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

#[allow(dead_code)]
impl WinstonConfig {
    // save configuration to provided filepath or the default config file location
    pub fn save_config(self, fp: Option<PathBuf>) -> Result<()> {
        // check if provided filepath is valid
        let fp = match fp {
            Some(fp) => fp,
            None => {
                let mut fp = dirs::config_dir().ok_or("Could not find config directory")?;
                fp.push("winston");
                fp.push("config.toml");
                fp
            }
        };
        // write config to file
        let config = toml::to_string(&self)?;
        let mut file = std::fs::File::create(fp)?;
        file.write_all(config.as_bytes())?;

        Ok(())
    }
    // load configuration from provided filepath or the default config file location
    pub fn load_config(fp: Option<PathBuf>) -> Result<Self> {
        let fp = match fp {
            Some(fp) => fp,
            None => {
                let mut fp = dirs::config_dir().ok_or("Could not find config directory")?;
                fp.push("winston");
                fp.push("config.toml");
                fp
            }
        };
        let v2 = WinstonConfigBuilder::new().load_config(&fp).build()?;

        Ok(v2)
    }
}

#[derive(Debug, Deserialize)]
pub struct WinstonConfigBuilder {
    pub openai_org_id: Option<String>,
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

#[allow(dead_code)]
impl WinstonConfigBuilder {
    pub fn new() -> Self {
        Self {
            openai_org_id: None,
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

    pub fn openai_org_id(mut self, id: String) -> Self {
        self.openai_org_id = Some(id);
        self
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
        let openai_org_id = self.openai_org_id.expect("Missing OpenAI Organization ID");
        let openai_api_key = self.openai_api_key.expect("Missing OpenAI API Key");
        let api_endpoint = self.api_endpoint.unwrap_or_else(|| OPENAI_ENDPOINT.to_string());
        let model = self.model.unwrap_or_else(|| MODEL.to_string());
        let max_tokens = self.max_tokens.unwrap_or_else(|| MAX_TOKENS);
        let temperature = self.temperature.unwrap_or_else(|| TEMPERATURE);
        let top_p = self.top_p.unwrap_or_else(|| TOP_P);
        let frequency_penalty = self.frequency_penalty.unwrap_or_else(|| FREQUENCY_PENALTY);
        let presence_penalty = self.presence_penalty.unwrap_or_else(|| PRESENCE_PENALTY);
        let stop = self.stop.unwrap_or_else(|| STOP.to_string());
        
        Ok(WinstonConfig {
            openai_org_id,
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

    // load_config takes a PathBuf and returns a WinstonConfigBuilder. The config
    // file should be in the TOML format. Using the TOML crate, the function should
    // only replace fields that have valid values in the config file. If the config
    // file is missing a field, the default value should be used.
    pub fn load_config(mut self, fp: &PathBuf) -> Self {
        let config = std::fs::read_to_string(fp).unwrap();
        let config: WinstonConfigBuilder = toml::from_str(&config).unwrap();
        // check each config field, if a valid value exists, replace the self field
        if let Some(openai_org_id) = config.openai_org_id {
            self.openai_org_id = Some(openai_org_id);
        }
        if let Some(openai_api_key) = config.openai_api_key {
            self.openai_api_key = Some(openai_api_key);
        }
        if let Some(api_endpoint) = config.api_endpoint {
            self.api_endpoint = Some(api_endpoint);
        }
        if let Some(model) = config.model {
            self.model = Some(model);
        }
        if let Some(max_tokens) = config.max_tokens {
            self.max_tokens = Some(max_tokens);
        }
        if let Some(temperature) = config.temperature {
            self.temperature = Some(temperature);
        }
        if let Some(top_p) = config.top_p {
            self.top_p = Some(top_p);
        }
        if let Some(frequency_penalty) = config.frequency_penalty {
            self.frequency_penalty = Some(frequency_penalty);
        }
        if let Some(presence_penalty) = config.presence_penalty {
            self.presence_penalty = Some(presence_penalty);
        }
        if let Some(stop) = config.stop {
            self.stop = Some(stop);
        }
        self
    }
}

#[cfg(test)]
mod test {
    // sample config file
    const TEST_CONFIG: &str = r#"model = "davinci"
max_tokens = 2048
tmperature = 0.9
top_p = 1.0
frequency_penalty = 0.0
presence_penalty = 0.0
stop = "\n"
"#;

    use super::*;

    // test that the config file is parsed correctly
    #[test]
    fn parse_config_test() {
        // write TEST_CONFIG to a temp file
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_file = temp_dir.path().join("test.toml");
        std::fs::write(&temp_file, TEST_CONFIG).unwrap();

        // create a new config builder and load the test config        
        let config = WinstonConfigBuilder::new() 
            .load_config(&std::path::PathBuf::from(temp_file))
            .build().unwrap();
        // check that the config fields are set correctly
        assert_eq!(config.model, "davinci");
        assert_eq!(config.max_tokens, 2048);
        assert_eq!(config.temperature, 0.9);
        assert_eq!(config.top_p, 1.0);
        assert_eq!(config.frequency_penalty, 0.0);
        assert_eq!(config.presence_penalty, 0.0);
        assert_eq!(config.stop, "\n");
    }
}
