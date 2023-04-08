use clap::Parser;
use serde::{Deserialize, Serialize};

// return result or std error
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const LONG_ABOUT: &str = r#"
    Winston is a command line chatGPT implementation.
"#;
const CONFIG_ENV: &str = "XDG_CONFIG_HOME";
const CONFIG_PATH: &str = "winston/config.toml";
const ENV: &str = "OPENAI_API_KEY";
const MODEL: &str = "davinci";
const MAX_TOKENS: usize = 2048;
const TEMPERATURE: f64 = 0.7;
const TOP_P: f64 = 1.0;
const FREQUENCY_PENALTY: f64 = 0.5;
const PRESENCE_PENALTY: f64 = 0.5;
const TIMEOUT: f64 = 0.5;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub presence_penalty: Option<f64>,
    pub stop_sequence: Option<String>,
    pub timeout: Option<f64>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            api_key: None,
            model: Some(MODEL.to_string()),
            max_tokens: Some(MAX_TOKENS),
            temperature: Some(TEMPERATURE),
            top_p: Some(TOP_P),
            frequency_penalty: Some(FREQUENCY_PENALTY),
            presence_penalty: Some(PRESENCE_PENALTY),
            timeout: Some(TIMEOUT),
            stop_sequence: None,
        }
    }
    pub fn load_config() -> Result<Config> {
        let config_file = std::env::var(CONFIG_ENV)
            .unwrap_or_else(|_| String::from(std::env::var("HOME").unwrap() + "/.config"))
            + "/"
            + CONFIG_PATH;
        let config = std::fs::read_to_string(config_file)?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }

    pub fn save_config(&self) -> Result<()> {
        let config_file = std::env::var(CONFIG_ENV)
            .unwrap_or_else(|_| String::from(std::env::var("HOME").unwrap() + "/.config"))
            + "/"
            + CONFIG_PATH;
        let mut file = std::fs::File::create(config_file)?;
        file.write_all(toml::to_string(&self)?.as_bytes())?;
        Ok(())
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
struct Options {
    #[clap(short, long, env = ENV, hide_env_values = true)]
    api_key: String,
}

#[cfg(test)]
mod test {
    // test write config file
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_write_config() {
        let config = Config::default();
        let config_file = std::env::var(CONFIG_ENV)
            .unwrap_or_else(|_| String::from(std::env::var("HOME").unwrap() + "/.config"))
            + "/"
            + CONFIG_PATH;
        let mut file = File::create(config_file).unwrap();
        file.write_all(toml::to_string(&config).unwrap().as_bytes())
            .unwrap();
    }

    #[test]
    fn test_load_config() {
        let config = Config::load_config().unwrap();
        assert_eq!(config.api_key, None);
        assert_eq!(config.model, Some(MODEL.to_string()));
        assert_eq!(config.max_tokens, Some(MAX_TOKENS));
        assert_eq!(config.temperature, Some(TEMPERATURE));
        assert_eq!(config.top_p, Some(TOP_P));
        assert_eq!(config.frequency_penalty, Some(FREQUENCY_PENALTY));
        assert_eq!(config.presence_penalty, Some(PRESENCE_PENALTY));
        assert_eq!(config.timeout, Some(TIMEOUT));
        assert_eq!(config.stop_sequence, None);
    }
}
