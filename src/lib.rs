use std::io::Write;
use std::path::PathBuf;
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

    // load config file: takes an optional pathbuf to a config file and returns a config struct
    pub fn load_config(&self, config_file: Option<&str>) -> Result<Config> {
        // if config_file is none, check env for config path, default to ~/.config/winston/config.toml
        let filepath = match config_file {
            Some(config_file) => config_file.to_owned(),
            None => Config::get_config_path()?,
        };
        let cfg = PathBuf::from(filepath);
        // if config file exists, load it, otherwise return default config
        if cfg.exists() {
            let config = std::fs::read_to_string(cfg)?;
            let config: Config = toml::from_str(&config)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    // save config file: takes a pathbuf to a config file, writes a config struct to it
    pub fn save_config(&self, config_file: Option<&str>) -> Result<()> {
        // if config_file is none, check env for config path, default to ~/.config/winston/config.toml
        let filepath = match config_file {
            Some(config_file) => config_file.to_owned(),
            None => Config::get_config_path()?,
        };
        // create config file if it doesn't exist, otherwise overwrite it
        let config_file = PathBuf::from(filepath);
        let mut config_file = std::fs::File::create(config_file)?;
        let config = toml::to_string(&self)?;
        config_file.write_all(config.as_bytes())?;
        Ok(())
    }

    fn get_config_path() -> Result<String> {
        let mut config_path = match std::env::var(CONFIG_ENV) {
            Ok(config_path) => std::fs::canonicalize(config_path)?,
            Err(_) => {
                let mut home = std::fs::canonicalize(std::env::var("HOME")?)?;
                home.push(".config");
                home
            }
        };
        // append CONFIG_PATH to config_path
        config_path.push(CONFIG_PATH);
        // return config_path as a string
        let path = config_path.to_str().unwrap();
        Ok(path.to_owned())
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
struct Options {
    #[arg(short, long, env = ENV, hide_env_values = true)]
    api_key: String,
    #[arg(short, long, default_value_t = MODEL.to_string())]
    model: String,
    #[arg(short, long, default_value_t = MAX_TOKENS)]
    max_tokens: usize,
    #[arg(short, long, default_value_t = TEMPERATURE)]
    temperature: f64,
    #[arg(short, long, default_value_t = TOP_P)]
    top_p: f64,
    #[arg(short, long, default_value_t = FREQUENCY_PENALTY)]
    frequency_penalty: f64,
    #[arg(short, long, default_value_t = PRESENCE_PENALTY)]
    presence_penalty: f64,
    #[arg(short, long)]
    stop_sequence: Option<String>,
    #[arg(short, long, default_value_t = TIMEOUT)]
    timeout: f64,
    #[arg(short, long)]
    config_path: Option<PathBuf>,
}

#[cfg(test)]
mod test {
    // test write config file
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_save_load_config() {
        let config = Config::default();
        let config_file = PathBuf::from("test.toml");
        config.save_config(config_file.to_str()).unwrap();
        assert!(config_file.exists());

        let config = config.load_config(config_file.to_str()).unwrap();
        assert_eq!(config.api_key, None);
        std::fs::remove_file(config_file).unwrap();
    }
}
