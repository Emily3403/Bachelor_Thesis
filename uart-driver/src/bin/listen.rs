#![feature(stmt_expr_attributes)]

use clap::Parser;
use std::sync::mpsc::channel;
use uart_lib::cli::Cli;
use uart_lib::uart::logger::Logger;
use uart_lib::uart::packet::decode_packets;
use uart_lib::{init_logging, spawn_uart_thread};

pub fn main() {
    let cli = Cli::parse();
    init_logging();

    let (tx, rx) = channel();
    let _uart_thread = spawn_uart_thread(tx, cli.baudrate);

    let mut packets = Vec::new();
    let mut logger = Logger::new(&cli);

    decode_packets(rx, &mut packets, &mut logger);
}
