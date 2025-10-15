use std::path::{Path, PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "9600")]
    pub baudrate: u32,

    #[arg(short, long, help = "Optional Path to save results to.")]
    pub savedir: Option<PathBuf>,
}
