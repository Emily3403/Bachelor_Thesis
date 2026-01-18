use crate::constants::LENGTH_OF_DATA;
use crate::uart::logger::Logger;
use crate::uart::stats::UARTStats;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Receiver;

#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub seq_num: u8,
    pub checksum: u8, // Pop count, only over [data]
    pub data: [u8; LENGTH_OF_DATA],

    pub stats: Vec<UARTStats>,
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

    pub fn from_bytes(seq_num: u8, last_seq_num: u8, checksum: u8, data: [u8; LENGTH_OF_DATA], stats: Vec<UARTStats>) -> Self {
        let mut errors = PacketErrors::empty();

        let expected_checksum = calculate_checksum(data);
        if expected_checksum != checksum {
            errors.insert(PacketErrors::CHECKSUM_MISMATCH)
        }

        let expected_seq_num = last_seq_num.wrapping_add(1);
        if seq_num != expected_seq_num {
            errors.insert(PacketErrors::SEQNUM_MISMATCH)
        }

        Packet {
            seq_num,
            checksum,
            data,
            stats,
            errors: PacketErrors::empty(),
        }
    }
}

pub fn decode_packets(tx: Receiver<(u8, Option<UARTStats>)>, packets: &mut Vec<Packet>, logger: &mut Logger) -> ! {
    let mut last_seq_num: u8 = 255;

    loop {
        let mut all_stats = Vec::new();
        macro_rules! get_byte {
            () => {{
                let (data, stats) = tx.recv().unwrap();
                logger.log_byte(data, &stats);

                if let Some(it) = stats {
                    all_stats.push(it);
                }
                data
            }};
        }

        let seq_num = get_byte!();
        let checksum = get_byte!();

        let mut data = [0; LENGTH_OF_DATA];
        for i in 0..LENGTH_OF_DATA {
            data[i] = get_byte!();
        }

        let packet = Packet::from_bytes(seq_num, last_seq_num, checksum, data, all_stats);
        logger.log_packet(&packet);

        if packet.is_valid() {
            last_seq_num = last_seq_num.wrapping_add(1);
        }
        packets.push(packet);
    }
}

pub fn calculate_checksum(data: [u8; LENGTH_OF_DATA]) -> u8 {
    data.iter().map(|d| d.count_ones() as u8).sum()
}
