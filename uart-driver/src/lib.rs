use crate::logger::loggable::Loggable;
use crate::logger::thread_local::LOGGER;
use crate::uart::uart::MiniUART;
use clap::Parser;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

pub mod cli;
pub mod constants;
pub mod logger;
pub mod uart;

use crate::cli::Cli;
use crate::logger::get_time;
use mutually_exclusive_features::exactly_one_of;

exactly_one_of!("driver_irq", "driver_polling");
exactly_one_of!("io_data", "io_scratch");

#[cfg(all(feature = "driver_polling", feature = "io_scratch"))]
compile_error!("With polling enable d, the io_scratch feature isn't usable");

pub fn init_logging() {
    pretty_env_logger::init();
    log!(Cli::parse(), get_time())
}

pub fn spawn_uart_thread(tx: Sender<u8>, baudrate: u32) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut uart = MiniUART::new(baudrate);

        loop {
            let (data, i_count, time) = uart.get_byte();
            let stats = uart.get_stats();

            // TODO: Does the order of these operations matter?
            tx.send(data).unwrap();
            log!(data, time);
            log!((stats, i_count), time);
        }
    })
}
