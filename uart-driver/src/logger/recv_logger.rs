use crate::cli::Cli;
use crate::logger::{LogMessageType, LogReceiver, Logger, LoggerType};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::sync::mpsc::channel;

/// Contains all configuration and Ownership of Logger (for a single thread)

impl LoggerType {
    pub fn from_cli(cli: &Cli) -> Self {
        let mut it = LoggerType::empty();
        if cli.loglevel == "info" {
            it.insert(LoggerType::PACKETS)
        }

        if cli.loglevel != "error" {
            it.insert(LoggerType::CONFIG);
            it.insert(LoggerType::DATA);
        }

        it
    }
}

impl LogReceiver {
    pub fn do_logging(mut self) -> ! {
        loop {
            let (msg, typ, time) = self.rx.recv().unwrap();
            let file = typ.decide_file(&mut self);

            match typ {
                LogMessageType::Data => write!(file, "{}", msg),
                _ => writeln!(file, "{} | {}", time, msg),
            }
                .expect("Log Writing Failed!");
        }
    }

    /// Opens files for logging and saves it all to `Self`
    pub fn new(cli: &Cli) -> (LogReceiver, Logger) {
        macro_rules! create_file {
            ($it: expr) => {
                File::create(cli.savedir.join($it)).unwrap()
            };
        }

        create_dir_all(&cli.savedir).unwrap();
        let (tx, rx) = channel();

        let receiver = Self {
            what_to_log: LoggerType::from_cli(cli),
            rx,

            config_out: create_file!("config"),
            data_out: create_file!("stdout"),
            packet_out: create_file!("packets.log"),
            stat_out: create_file!("stats.log"),
        };

        let logger = Logger { tx };

        (receiver, logger)
    }
}
