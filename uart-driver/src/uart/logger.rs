use crate::cli::Cli;
use crate::uart::packet::Packet;
use crate::uart::stats::UARTStats;
use bitflags::bitflags;
use std::fs::{create_dir_all, File};
use std::io::Write;

pub struct Logger {
    pub what_to_log: LoggerType,
    pub data_out: File,   // stdout, decoded ASCII
    pub packet_out: File, // Logging of the packets
}

bitflags! {
    pub struct LoggerType: u8 {
        const DATA      = 0b0001;
        const PACKETS   = 0b0010;
    }
}

impl LoggerType {
    pub fn from_cli(cli: &Cli) -> Self {
        let mut it = LoggerType::empty();
        if cli.loglevel == "info" {
            it.insert(LoggerType::PACKETS)
        }
        if cli.loglevel != "error" {
            it.insert(LoggerType::DATA)
        }

        it
    }
}

impl Logger {
    pub fn new(cli: &Cli) -> Logger {
        create_dir_all(&cli.savedir).unwrap();
        let data_out = File::create(cli.savedir.join("stdout")).unwrap();
        let packet_out = File::create(cli.savedir.join("packets.log")).unwrap();

        Self {
            what_to_log: LoggerType::from_cli(cli),
            data_out,
            packet_out,
        }
    }

    pub fn log_byte(&mut self, b: u8, _stats: &Option<UARTStats>) {
        self.data_out.write_all(&[b]).unwrap()
    }

    pub fn log_packet(&mut self, packet: &Packet) {
        self.packet_out.write_all(format!("{packet}\n").as_bytes()).unwrap();
    }
}
