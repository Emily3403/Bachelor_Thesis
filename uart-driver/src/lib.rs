#![feature(mpmc_channel)]
extern crate core;

use crate::uart::uart::MiniUART;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

pub mod cli;
pub mod constants;
pub mod logger;
pub mod uart;

use crate::logger::LogSender;
use mutually_exclusive_features::exactly_one_of;

exactly_one_of!("driver_irq", "driver_polling");
exactly_one_of!("io_data", "io_scratch");

#[cfg(all(feature = "driver_polling", feature = "io_scratch"))]
compile_error!("With polling enabled, the io_scratch feature isn't usable");

/// This is the common setup routine shared between all code to get a MiniUART.
/// Any shared startup code should thus be located or called from here.
///
/// The `.init()` will be called from here.
pub fn init_logging() {
    pretty_env_logger::init();
}

pub fn spawn_uart_thread(tx: Sender<u8>, baudrate: u32, mut logger: LogSender) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut uart = MiniUART::new(baudrate);

        loop {
            let data = uart.get_byte();
            let stats = uart.get_stats();
            logger.log_byte(data, &stats);

            tx.send(data).unwrap();
        }
    })
}
