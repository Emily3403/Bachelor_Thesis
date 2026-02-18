use crate::cli::Cli;
use crate::logger::{LogReceiver, Logger, LoggerType};
use std::fs::{create_dir_all, File};
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
    pub fn do_logging(self) -> ! {
        loop {
            let it = self.rx.recv();
        }
    }

    /// Opens files for logging and saves it all to `Self`
    pub fn new(cli: &Cli) -> (LogReceiver, Logger) {
        create_dir_all(&cli.savedir).unwrap();
        let config_out = File::create(cli.savedir.join("config")).unwrap();
        let data_out = File::create(cli.savedir.join("stdout")).unwrap();
        let packet_out = File::create(cli.savedir.join("packets.log")).unwrap();
        let (tx, rx) = channel();

        let receiver = Self {
            what_to_log: LoggerType::from_cli(cli),
            rx,

            config_out,
            data_out,
            packet_out,
        };
        let logger = Logger { tx };

        (receiver, logger)
    }
}
