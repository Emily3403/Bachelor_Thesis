use crate::cli::Cli;
use crate::logger::{InterruptCount, LogMessageType};
use crate::uart::packet::Packet;
use crate::uart::stats::UARTStats;

/// Multiple Object shall be delimited with `;`
pub trait Loggable: Send {
    fn generate_string(&self) -> String;

    fn typ(&self) -> LogMessageType;
}

impl Loggable for Packet {
    fn generate_string(&self) -> String {
        format!("{}", serde_json::to_string(self).unwrap())
    }

    fn typ(&self) -> LogMessageType {
        LogMessageType::Packet
    }
}

impl Loggable for u8 {
    fn generate_string(&self) -> String {
        char::from(*self).to_string()
    }

    fn typ(&self) -> LogMessageType {
        LogMessageType::Data
    }
}

impl Loggable for (Option<UARTStats>, InterruptCount) {
    fn generate_string(&self) -> String {
        let (stats, i_count) = self;

        format!("{}; {}", serde_json::to_string(stats).unwrap(), serde_json::to_string(i_count).unwrap())
    }

    fn typ(&self) -> LogMessageType {
        LogMessageType::Stats
    }
}

impl Loggable for Cli {
    fn generate_string(&self) -> String {
        format!("{}", serde_json::to_string(self).unwrap())
    }

    fn typ(&self) -> LogMessageType {
        LogMessageType::Config
    }
}
