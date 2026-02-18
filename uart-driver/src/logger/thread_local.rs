use crate::cli::Cli;
use crate::logger::{LogReceiver, Logger};
use clap::Parser;
use once_cell::sync::Lazy;
use std::thread;

/// This is the root "owner" of all loggers
const _LOGGER: Lazy<Logger> = Lazy::new(|| {
    let (receiver, logger) = LogReceiver::new(&Cli::parse());
    thread::spawn(move || receiver.do_logging());

    logger
});

thread_local! {
    pub static LOGGER: Logger = _LOGGER.clone()
}

#[macro_export]
macro_rules! log {
    ($it: expr, $time: expr) => {{
        let it = $it;  // This is necessary to bind it to a object together. Otherwise after the first use, the inner objects are moved.
        let msg = (it.generate_string(), it.typ(), $time);
        LOGGER.with(|logger| logger.send(msg))
    }};
}
