use crate::constants::LENGTH_OF_DATA;
use crate::uart::stats::UARTStats;
use crate::uart::uart::MiniUART;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::sync::mpsc::Receiver;
use crate::uart::packet::Packet::{ChecksumMismatch, Okay, SeqNumMismatch};

#[derive(Serialize, Deserialize)]
pub struct InnerPacket {
    pub seq_num: u8,
    pub checksum: u8, // Pop count, only over [data]
    pub data: [u8; LENGTH_OF_DATA],

    pub stats: UARTStats,
}

pub enum PacketResult {
    Ok(Packet),
    Err {
        packet: Packet,
        error: PacketError,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketError {
    ChecksumMismatch {
        expected: u8,
        actual: u8,
    },
    SequenceMismatch {
        expected: u8,
        actual: u8,
    },
}




// TODO: Encode Error type
#[derive(Serialize, Deserialize)]
pub enum Packet {
    Okay(InnerPacket),
    ChecksumMismatch { inner: InnerPacket, val: u8, expected: u8 },
    SeqNumMismatch { inner: InnerPacket, val: u8, expected: u8 },
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Packet {
    pub fn from_bytes(seq_num: u8, last_seq_num: u8, checksum: u8, data: [u8; LENGTH_OF_DATA], stats: UARTStats) -> Self {
        let inner = InnerPacket { seq_num, checksum, data, stats };

        let expected_checksum = calculate_checksum(data);
        if expected_checksum != checksum {
            return ChecksumMismatch { inner, val: checksum, expected: expected_checksum };
        }

        let expected_seq_num = last_seq_num.wrapping_add(1);
        if seq_num != expected_seq_num {
            return SeqNumMismatch { inner, val: seq_num, expected: expected_seq_num };
        }

        Okay(inner)
    }
}

// No reference to MiniUART →
pub fn decode_packets(tx: Receiver<u8>, packets: &mut Vec<Packet>, out: &mut (impl Write + ?Sized), uart: &MiniUART) -> ! {
    let mut last_seq_num: u8 = 255;

    loop {
        let seq_num = tx.recv().unwrap();
        let checksum = tx.recv().unwrap();

        let mut data = [0; LENGTH_OF_DATA];
        for i in 0..LENGTH_OF_DATA {
            data[i] = tx.recv().unwrap();
        }

        let stats = uart.read_stats();

        let packet = Packet::from_bytes(seq_num, last_seq_num, checksum, data, stats);
        out.write_all(format!("{packet}\n").as_bytes()).unwrap();

        if let Okay(_) = packet {
            last_seq_num = last_seq_num.wrapping_add(1);
        }
        packets.push(packet);
    }
}

pub fn calculate_checksum(data: [u8; LENGTH_OF_DATA]) -> u8 {
    data.iter().map(|d| d.count_ones() as u8).sum()
}

pub fn infinite_read(mut uart: MiniUART, out: &mut (impl Write + ?Sized)) -> ! {
    loop {
        // match uart.read_packet() {
        //     Ok(packet) => out.write_all(format!("{packet}\n").as_bytes()).unwrap(),
        //     Err(e) => error!("Reading packet failed! {e}"),
        // }
    }
}
