#![feature(stmt_expr_attributes)]

use uart_lib::init_uart;

pub fn main() {
    let uart = init_uart();
}
