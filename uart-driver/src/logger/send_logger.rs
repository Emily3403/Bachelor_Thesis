use crate::cli::Cli;
use crate::logger::{LogReceiver, LogSender};
use crate::uart::packet::Packet;
use crate::uart::stats::UARTStats;

impl LogSender {
    pub fn from_receiver(recv: &LogReceiver) -> Self {
        Self {
            tx: recv.tx.clone(),
        }
    }

    pub fn log_config(&mut self, cli: &Cli) {
        todo!()
    }

    pub fn log_byte(&mut self, b: u8, _stats: &Option<UARTStats>) {
        todo!();
        // self.data_out.write_all(&[b]).unwrap()
    }

    pub fn log_packet(&mut self, packet: &Packet) {
        todo!()
        // self.packet_out.write_all(format!("{packet}\n").as_bytes()).unwrap();
    }
}
