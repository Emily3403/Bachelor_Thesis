use clap::Parser;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "9600")]
    pub baudrate: u32,

    #[arg(short, long, help = "Optional Path to save results to.", default_value = "uart-out", value_parser)]
    pub savedir: PathBuf,

    /// Info logs everything: Packets.log, data, etc
    /// Error logs nothing except errors, no data
    #[arg(long, default_value = "info", value_parser=parse_loglevel)]
    pub loglevel: String,

    #[arg(long, default_value = "1")]
    pub num_data_bytes: usize,
}

pub fn parse_loglevel(level: &str) -> Result<String, String> {
    // log::LOG_LEVEL_NAMES is private :/
    let level = level.to_lowercase();
    if level == "off" || level == "error" || level == "warn" || level == "info" || level == "debug" || level == "trace" {
        unsafe { env::set_var("RUST_LOG", &level) };
        return Ok(level.into());
    }

    Err(format!(r#"Expected loglevel to be in {{"off", "error", "warn", "info", "debug", "trace"}}, got "{level}""#))
}
