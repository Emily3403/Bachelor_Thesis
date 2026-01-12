use crate::constants::LENGTH_OF_DATA;
use crate::uart::packet::ReceivedPacket::{ChecksumMismatch, Okayy, SeqNumMismatch};
use crate::uart::stats::UARTStats;
use crate::uart::uart::MiniUART;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::sync::mpsc::Receiver;

#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub seq_num: u8,
    pub checksum: u8, // Pop count, only over [data]
    pub data: [u8; LENGTH_OF_DATA],

    pub stats: UARTStats,
}

#[derive(Serialize, Deserialize)]
pub enum ReceivedPacket {
    Okayy(Packet),
    ChecksumMismatch { packet: Packet, val: u8, expected: u8 },
    SeqNumMismatch { packet: Packet, val: u8, expected: u8 },
}

impl Display for ReceivedPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl ReceivedPacket {
    pub fn from_bytes(seq_num: u8, last_seq_num: u8, checksum: u8, data: [u8; LENGTH_OF_DATA], stats: UARTStats) -> Self {
        let packet = Packet { seq_num, checksum, data, stats };

        let expected_checksum = calculate_checksum(data);
        if expected_checksum != checksum {
            return ChecksumMismatch { packet, val: checksum, expected: expected_checksum };
        }

        let expected_seq_num = last_seq_num.wrapping_add(1);
        if seq_num != expected_seq_num {
            return SeqNumMismatch { packet: packet, val: seq_num, expected: expected_seq_num };
        }

        Okayy(packet)
    }
}

// No reference to MiniUART →
pub fn decode_packets(tx: Receiver<u8>, packets: &mut Vec<ReceivedPacket>, out: &mut (impl Write + ?Sized), uart: &MiniUART) -> ! {
    let mut last_seq_num: u8 = 255;

    loop {
        let seq_num = tx.recv().unwrap();
        let checksum = tx.recv().unwrap();

        let mut data = [0; LENGTH_OF_DATA];
        for i in 0..LENGTH_OF_DATA {
            data[i] = tx.recv().unwrap();
        }

        let stats = uart.read_stats();

        let packet = ReceivedPacket::from_bytes(seq_num, last_seq_num, checksum, data, stats);
        out.write_all(format!("{packet}\n").as_bytes()).unwrap();

        if let Okayy(_) = packet {
            last_seq_num = last_seq_num.wrapping_add(1);
        }
        packets.push(packet);
    }
}

pub fn calculate_checksum(data: [u8; LENGTH_OF_DATA]) -> u8 {
    data.iter().map(|d| d.count_ones() as u8).sum()
}
