#![feature(stmt_expr_attributes, impl_trait_in_bindings)]

use clap::Parser;
use log::{error, info, logger};
use std::fs::{create_dir_all, File};
use std::io::{stdout, Write};
use std::mem::transmute;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use text_diff::diff;
use uart_lib::cli::Cli;
use uart_lib::constants::EXPECTED_BYTES;
use uart_lib::uart::packet::{decode_packets, infinite_read};
use uart_lib::{init_uart, spawn_uart_thread};

pub fn main() {
    let cli = Cli::parse();

    let (tx, rx) = channel();
    let mut uart = init_uart(cli.baudrate);

    // This is prime ub and what the borrow checker is designed to stop: two mutable pointers to the same memory region
    // In our case this is memory to the registers of the UART, and in the `decode_packets` loop only the stat register is read. It is never touched by the uart thread.
    let uart_thread = spawn_uart_thread(&cli, tx, unsafe { transmute(&mut uart) });

    info!("Going into infinite listen!");
    stdout().flush().unwrap();

    let mut packets = Vec::new();

    if let Some(save_dir) = cli.savedir && save_dir != PathBuf::from("None")
    {
        create_dir_all(&save_dir).unwrap();
        decode_packets(rx, &mut packets, &mut File::create(save_dir.join("stdout")).unwrap(), &uart);
    } else {
        decode_packets(rx, &mut packets, &mut stdout(), &uart);
    }
}
