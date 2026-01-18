#![feature(stmt_expr_attributes)]

use clap::Parser;
use log::info;
use std::io::{stdout, Write};
use std::sync::mpsc::channel;
use uart_lib::cli::Cli;
use uart_lib::spawn_uart_thread;
use uart_lib::uart::logger::Logger;
use uart_lib::uart::packet::decode_packets;

pub fn main() {
    let cli = Cli::parse();

    let (tx, rx) = channel();
    let _uart_thread = spawn_uart_thread(tx, &cli);

    info!("Going into infinite listen!");
    stdout().flush().unwrap();

    let mut packets = Vec::new();
    let mut logger = Logger::new(&cli);

    decode_packets(rx, &mut packets, &mut logger);
}
