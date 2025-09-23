#![feature(stmt_expr_attributes)]

use uart_lib::init_uart;

pub fn main() {
    let mut uart = init_uart();
    uart.polling_write("Hello, 😊🦀你好\nHow are you?? 💖\n");
}
