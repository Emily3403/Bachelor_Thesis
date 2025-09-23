#![feature(stmt_expr_attributes)]

use log::{error, info, logger};
use text_diff::diff;
use uart_lib::{init_uart};
use uart_lib::constants::EXPECTED_BYTES;
use uart_lib::uart::read::infinite_read;
use uart_lib::uart::transmission::RxTransmission;

pub fn main() {
    let mut uart = init_uart();
    info!("Going into infinite listen!");
    infinite_read(uart);
}
