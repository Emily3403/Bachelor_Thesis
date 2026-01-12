#![feature(stmt_expr_attributes)]

use uart_lib::init_uart;

pub fn main() {
    let _uart = init_uart(32);
}
