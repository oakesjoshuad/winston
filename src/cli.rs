// Winston command line argument parser
// Winston should be able to parse all configuration options from the command
// line. It should check and prioritize environment variables, command line
// options are secondary and configuration files are tertiary.

use crate::config::{OPENAI_ENDPOINT, MODEL, MAX_TOKENS, TEMPERATURE, TOP_P, STOP, FREQUENCY_PENALTY, PRESENCE_PENALTY};

#[derive(Debug, clap::Parser)]
#[command(version, author, about)]
pub struct Options {
    #[arg(short = 'e', long, default_value_t = OPENAI_ENDPOINT.to_string())]
    pub openai_endpoint: String,
    #[arg(short = 'k', long, env = "OPENAI_API_KEY")]
    pub openai_api_key: String,
    #[arg(short = 'm', long, env = "OPENAI_MODEL", default_value_t = MODEL.to_string())]
    pub openai_model: String,
    #[arg(short = 'l', long, default_value_t = MAX_TOKENS)]
    pub openai_max_tokens: u32,
    #[arg(short = 't', long, default_value_t = TEMPERATURE)]
    pub openai_temperature: f32,
    #[arg(short = 'p', long, default_value_t = TOP_P)]
    pub openai_top_p: f32,
    #[arg(short = 'd', long, default_value_t = STOP.to_string())]
    pub stop: String,
    #[arg(short = 'f', long, default_value_t = FREQUENCY_PENALTY)]
    pub frequency_penalty: f32,
    #[arg(short = 'r', long, default_value_t = PRESENCE_PENALTY)]
    pub presence_penalty: f32,
}
