use bitflags::bitflags;
use std::fs::File;
use std::io::BufWriter;
use std::sync::mpsc::{Receiver, Sender};

pub mod loggable;
pub mod recv_logger;
pub mod thread_local;
pub mod time;

pub type TimeStamp = u64;
pub type InterruptCount = u32;
pub type LogMessage = (String, LogMessageType, TimeStamp); // TODO: How to integrate `LogThreadOrigin`?

pub enum LogMessageType {
    Config,
    Data,
    Packet,
    Stats,
}

#[derive(Clone)]
pub struct Logger {
    tx: Sender<LogMessage>,
}

impl Logger {
    pub fn send(&self, message: LogMessage) {
        self.tx.send(message).unwrap()
    }
}

pub struct LogReceiver {
    pub what_to_log: LoggerType,
    pub rx: Receiver<LogMessage>,

    pub config_out: BufWriter<File>, // Log the parameters as json
    pub data_out: BufWriter<File>,   // stdout, decoded ASCII
    pub packet_out: BufWriter<File>, // Logging of the packets
    pub stat_out: BufWriter<File>,    // Logging of stats (with Timestamps)
}

bitflags! {
    pub struct LoggerType: u8 {
        const CONFIG    = 0b0001;
        const DATA      = 0b0010;
        const PACKETS   = 0b0100;
    }
}

impl LogMessageType {
    pub fn decide_file<'a>(&self, it: &'a mut LogReceiver) -> &'a mut BufWriter<File> {
        match self {
            LogMessageType::Config => &mut it.config_out,
            LogMessageType::Data => &mut it.data_out,
            LogMessageType::Packet => &mut it.packet_out,
            LogMessageType::Stats => &mut it.stat_out
        }
    }
}

pub enum LogThreadOrigin {
    UART,
    PacketDecoder, // = Main Thread for now
}
