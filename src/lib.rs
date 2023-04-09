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

use std::env;
use std::fs;
use std::path::PathBuf;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

// Configuration builder struct
#[derive(Debug, Default)]

