#![feature(stmt_expr_attributes)]

use clap::Parser;
use std::sync::mpsc::channel;
use uart_lib::cli::Cli;
use uart_lib::uart::packet::main_thread_decode_packets;
use uart_lib::{init_logging, spawn_uart_thread};

pub fn main() {
    let cli = Cli::parse();
    init_logging(&cli);

    let mut packets = Vec::new();
    let (tx, rx) = channel();

    spawn_uart_thread(tx, cli.baudrate);
    main_thread_decode_packets(rx, &mut packets, &cli);
}
