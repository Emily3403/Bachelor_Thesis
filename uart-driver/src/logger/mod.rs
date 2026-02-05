use bitflags::bitflags;
use std::fmt::Formatter;
use std::fs::File;
use std::sync::mpmc::{Receiver, Sender};

pub mod loggable;
pub mod recv_logger;
pub mod send_logger;

pub trait Loggable: Send {
    fn log(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub type TimeStamp = u64;
pub type LogChannelT = (Box<dyn Loggable>, LogMessageType, LogThreadOrigin, TimeStamp);

pub enum LogMessageType {
    Config,
    Data,
    Packet,
}

pub struct LogSender {
    pub tx: Sender<LogChannelT>,
}

pub struct LogReceiver {
    pub what_to_log: LoggerType,
    pub rx: Receiver<LogChannelT>,
    pub tx: Sender<LogChannelT>, // Maintain the ownership to the channel such that LogSender's can be created from LogReceiver

    pub config_out: File, // Log the parameters as json
    pub data_out: File,   // stdout, decoded ASCII
    pub packet_out: File, // Logging of the packets
}

bitflags! {
    pub struct LoggerType: u8 {
        const CONFIG    = 0b0001;
        const DATA      = 0b0010;
        const PACKETS   = 0b0100;
    }
}

impl LogMessageType {
    pub fn decide_file<'a>(&self, it: &'a LogReceiver) -> &'a File {
        match self {
            LogMessageType::Config => &it.config_out,
            LogMessageType::Data => &it.data_out,
            LogMessageType::Packet => &it.packet_out,
        }
    }
}

pub enum LogThreadOrigin {
    UART,
    PacketDecoder, // = Main Thread for now
}
