mod cli;
mod config;

use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let opts = cli::Options::parse();
    println!("{:?}", opts);
    Ok(())
}
