use std::path::PathBuf;

use clap::Parser;
use serde::Serialize;

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
    #[default]
    Info,
    Trace,
    Debug,
    Warn,
    Error,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom path to config file
    #[arg(short, long, value_name = "CONFIG_FILE", default_value = "config.yaml")]
    pub config: PathBuf,
    /// Sets a custom log level
    #[arg(
        short,
        long,
        value_name = "LOG_LEVEL",
        default_value = "info",
        value_enum
    )]
    pub log_level: LogLevel,
}

pub fn setup_cli() -> Cli {
    Cli::parse()
}
