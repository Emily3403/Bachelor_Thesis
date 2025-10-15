#![feature(stmt_expr_attributes)]

use std::io::{stdout, Write};
use clap::Parser;
use log::{error, info, logger};
use text_diff::diff;
use uart_lib::cli::Cli;
use uart_lib::constants::EXPECTED_BYTES;
use uart_lib::init_uart;
use uart_lib::uart::read::infinite_read;
use uart_lib::uart::transmission::RxTransmission;

pub fn main() {
    let cli = Cli::parse();

    let uart = init_uart(cli.baudrate);
    info!("Going into infinite listen!");
    stdout().flush().unwrap();

    let out = if let Some(it) = cli.savedir {

    } else {

    };
    infinite_read(uart, &mut stdout());
}
