use crate::cli::Cli;
use crate::logger::{LogReceiver, LogSender, LoggerType};
use std::fs::{create_dir_all, File};
use std::sync::mpmc::channel;

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
    // TODO: Should actually spawn the thread
    pub fn new(cli: &Cli) -> LogReceiver {
        create_dir_all(&cli.savedir).unwrap();
        let config_out = File::create(cli.savedir.join("config")).unwrap();
        let data_out = File::create(cli.savedir.join("stdout")).unwrap();
        let packet_out = File::create(cli.savedir.join("packets.log")).unwrap();
        let (tx, rx) = channel();

        Self {
            what_to_log: LoggerType::from_cli(cli),
            rx,
            tx,

            config_out,
            data_out,
            packet_out,
        }
    }

    pub fn new_sender(&self) -> LogSender {
        LogSender {
            tx: self.tx.clone()
        }
    }
}
