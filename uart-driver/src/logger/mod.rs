use crate::cli::Cli;
use bitflags::bitflags;
use clap::Parser;
use once_cell::sync::Lazy;
use std::fmt::Formatter;
use std::fs::File;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub mod loggable;
pub mod recv_logger;
pub mod send_logger;
pub mod time;

pub trait Loggable: Send {
    fn log(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}


pub type TimeStamp = u64;
pub type InterruptCount = u32;
pub type LogMessage = (Box<dyn Loggable>, LogMessageType, LogThreadOrigin, TimeStamp);

pub enum LogMessageType {
    Config,
    Data,
    Packet,
}

#[derive(Clone)]
pub struct Logger {
    pub tx: Sender<LogMessage>,
}

pub struct LogReceiver {
    pub what_to_log: LoggerType,
    pub rx: Receiver<LogMessage>,

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

impl Logger {
    /// Spawns the Thread and sets up the channel.
    fn spawn(cli: &Cli) -> Logger {
        let (receiver, logger) = LogReceiver::new(cli);
        thread::spawn(move || receiver.do_logging());

        logger
    }
}

/// This is the root "owner"
const _LOGGER: Lazy<Logger> = Lazy::new(|| Logger::spawn(&Cli::parse()));


thread_local! {
    pub static LOGGER: Logger = _LOGGER.clone()
}

#[macro_export] macro_rules! log_packet {
    ($packet: expr) => {
        LOGGER.with(|logger| logger.log_packet($packet))
    };
}

#[macro_export] macro_rules! log_config {
    ($config: expr) => {
        LOGGER.with(|logger| logger.log_config($config))
    };
}

#[macro_export] macro_rules! log_byte {
    ($cli: expr, $stats: expr) => {
        LOGGER.with(|logger| logger.log_byte($cli, $stats))
    };
}