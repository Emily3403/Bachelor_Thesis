use crate::uart::uart::MiniUART;
use std::process::Command;
use uio::UioDevice;

pub mod cli;
pub mod constants;
pub mod uart;

use mutually_exclusive_features::exactly_one_of;
exactly_one_of!("driver_irq", "driver_polling");
exactly_one_of!("io_data", "io_scratch");

#[cfg(all(feature = "driver_polling", feature = "io_scratch"))]
compile_error!("With polling enabled, the io_scratch feature isn't usable");

/// This is the common setup routine shared between all code to get a MiniUART.
/// Any shared startup code should thus be located or called from here.
///
/// The `.init()` will be called from here.
pub fn init_uart(baudrate: u32) -> MiniUART {
    pretty_env_logger::init();

    let dev = UioDevice::try_new(0).unwrap();
    let mut uart = MiniUART::from_uio(dev);
    uart.init(baudrate).unwrap();

    uart
}
