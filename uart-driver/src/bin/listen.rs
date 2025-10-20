#![feature(stmt_expr_attributes)]

use clap::Parser;
use log::{error, info, logger};
use std::fs::{create_dir_all, File};
use std::io::{stdout, Write};
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

    if let Some(save_dir) = cli.savedir {
        create_dir_all(&save_dir).unwrap();
        let mut stdout = File::create(save_dir.join("stdout")).unwrap();

        infinite_read(uart, &mut stdout);
    } else {
        infinite_read(uart, &mut stdout());
        
    };
}
