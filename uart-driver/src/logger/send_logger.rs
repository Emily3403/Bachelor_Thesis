use crate::cli::Cli;
use crate::logger::Logger;
use crate::uart::packet::Packet;
use crate::uart::stats::UARTStats;

impl Logger {
    pub fn log_config(&self, cli: &Cli) {
        todo!()
    }

    pub fn log_byte(&self, b: u8, _stats: &Option<UARTStats>) {
        todo!();
        // self.data_out.write_all(&[b]).unwrap()
    }

    pub fn log_packet(&self, packet: &Packet) {
        todo!()
        // self.packet_out.write_all(format!("{packet}\n").as_bytes()).unwrap();
    }
}
