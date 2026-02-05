#![feature(stmt_expr_attributes)]

use clap::Parser;
use std::sync::mpsc::channel;
use uart_lib::cli::Cli;
use uart_lib::logger::LogReceiver;
use uart_lib::uart::packet::decode_packets;
use uart_lib::{init_logging, spawn_uart_thread};

pub fn main() {
    let cli = Cli::parse();
    init_logging();

    let logger = LogReceiver::new(&cli);
    let (uart_tx, uart_rx) = channel();

    let _uart_thread = spawn_uart_thread(uart_tx, cli.baudrate, logger.new_sender());

    let mut packets = Vec::new();
    decode_packets(uart_rx, &mut packets, logger.new_sender(), &cli);
}
