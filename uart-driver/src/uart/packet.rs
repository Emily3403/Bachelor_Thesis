use crate::cli::Cli;
use crate::log;
use crate::logger::time::get_time;
use crate::Loggable;
use crate::LOGGER;
use bitflags::bitflags;
use log::info;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::{stdout, Write};
use std::sync::mpsc::Receiver;

#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub seq_num: u8,
    pub checksum: u8,  // Pop count, only over [data]
    pub data: Vec<u8>, // TODO: Benchmark if Vec is a Performance Bottleneck

    pub errors: PacketErrors,
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct PacketErrors: u8 {
        const CHECKSUM_MISMATCH = 0b0001;
        const SEQNUM_MISMATCH   = 0b0010;
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Packet {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn new(seq_num: u8, last_seq_num: u8, checksum: u8, data: Vec<u8>) -> Self {
        let mut errors = PacketErrors::empty();

        let expected_checksum = calculate_checksum(&data);
        if expected_checksum != checksum {
            errors.insert(PacketErrors::CHECKSUM_MISMATCH)
        }

        let expected_seq_num = last_seq_num.wrapping_add(1);
        if seq_num != expected_seq_num {
            errors.insert(PacketErrors::SEQNUM_MISMATCH)
        }

        Packet { seq_num, checksum, data, errors: PacketErrors::empty() }
    }
}

pub fn main_thread_decode_packets(tx: Receiver<u8>, packets: &mut Vec<Packet>, cli: &Cli) -> ! {
    let mut last_seq_num: u8 = 255;

    info!("Going into infinite listen!");
    stdout().flush().unwrap();

    loop {
        let seq_num = tx.recv().unwrap();
        // TODO: If the seq_num doesn't match, try reinterpreting the packet with a byte offset and see if it makes a difference
        let checksum = tx.recv().unwrap();

        let mut data = Vec::new();
        for _ in 0..cli.num_data_bytes {
            data.push(tx.recv().unwrap());
        }

        let packet = Packet::new(seq_num, last_seq_num, checksum, data);
        log!(&packet, get_time());

        if packet.is_valid() {
            last_seq_num = last_seq_num.wrapping_add(1);
        }
        packets.push(packet);
    }
}

pub fn calculate_checksum(data: &Vec<u8>) -> u8 {
    data.iter().map(|d| d.count_ones() as u8).sum()
}
