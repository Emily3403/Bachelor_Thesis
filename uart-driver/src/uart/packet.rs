use crate::constants::LENGTH_OF_DATA;
use crate::uart::err::RxResult;
use crate::uart::stats::UARTStats;
use crate::uart::uart::MiniUART;
use log::error;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::Write;

// TODO: Theoretically this can be packed into 64 bit with 2 bytes of data
//  8 + 8 + _ + 32
#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub seq_num: u8,
    pub checksum: u8, // Pop count
    pub data: [u8; LENGTH_OF_DATA],

    pub stats: UARTStats,
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet {}", serde_json::to_string(self).unwrap())
    }
}

pub trait PacketReader {
    fn read_packet(&mut self) -> RxResult<Packet>;
}

impl PacketReader for MiniUART {
    fn read_packet(&mut self) -> RxResult<Packet> {
        let seq_num = self.get_byte();

        let mut data = [0; LENGTH_OF_DATA];
        for i in 0..LENGTH_OF_DATA {
            data[i] = self.get_byte();
        }

        let checksum = self.get_byte();
        Ok(Packet {
            seq_num,
            data,
            checksum,
            stats: self.regs.read_stats(),
        })
    }
}

pub fn infinite_read(mut uart: MiniUART, out: &mut (impl Write + ?Sized)) -> ! {
    loop {
        match uart.read_packet() {
            Ok(packet) => out.write_all(format!("{packet}\n").as_bytes()).unwrap(),
            Err(e) => error!("Reading packet failed! {e}"),
        }
    }
}
