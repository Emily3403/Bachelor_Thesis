#![feature(stmt_expr_attributes)]

use uart_lib::init_uart;

pub fn main() {
    let _uart = init_uart(9600);
    // uart.polling_write("Hello, 😊🦀你好\nHow are you?? 💖\n");
}
