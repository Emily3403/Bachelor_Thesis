#![feature(stmt_expr_attributes)]

use clap::Parser;
use log::info;
use std::fs::{File, create_dir_all};
use std::io::{Write, stdout};
use std::mem::transmute;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use uart_lib::cli::Cli;
use uart_lib::uart::packet::decode_packets;
use uart_lib::{init_uart, spawn_uart_thread};

pub fn main() {
    let cli = Cli::parse();

    let (tx, rx) = channel();
    let _uart_thread = spawn_uart_thread(tx, &cli);

    info!("Going into infinite listen!");
    stdout().flush().unwrap();

    let mut packets = Vec::new();

    if let Some(save_dir) = cli.savedir
        && save_dir != PathBuf::from("None")
    {
        create_dir_all(&save_dir).unwrap();
        decode_packets(rx, &mut packets, &mut File::create(save_dir.join("stdout")).unwrap());
    } else {
        decode_packets(rx, &mut packets, &mut stdout());
    }
}
