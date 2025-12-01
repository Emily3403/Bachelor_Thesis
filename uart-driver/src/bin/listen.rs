#![feature(stmt_expr_attributes)]

use clap::Parser;
use log::{error, info, logger};
use std::fs::{create_dir_all, File};
use std::io::{stdout, Write};
use std::path::PathBuf;
use text_diff::diff;
use uart_lib::cli::Cli;
use uart_lib::constants::EXPECTED_BYTES;
use uart_lib::init_uart;
use uart_lib::uart::packet::infinite_read;

pub fn main() {
    let cli = Cli::parse();

    let mut uart = init_uart(cli.baudrate);
    info!("Going into infinite listen!");
    stdout().flush().unwrap();

    if let Some(save_dir) = cli.savedir
        && save_dir != PathBuf::from("None")
    {
        create_dir_all(&save_dir).unwrap();
        let mut stdout = File::create(save_dir.join("stdout")).unwrap();
        infinite_read(uart, &mut stdout);
    } else {
        infinite_read(uart, &mut stdout());

        // let it  = uart.read_packet();
        // if let Some(it) = it {
        //     info!("{it}")
        // }
        // else {
        //     error!("Packet is None!")
        // }
    };
}
