use std::time::Duration;

pub const POLLING_DURATION: Duration = Duration::new(0, 25);
pub const RESET_SLEEP_DURATION: Duration = Duration::new(0, 50_000_000);

pub const RESET_CHAR: u8 = '\n' as u8;


pub const EXPECTED_BYTES: &[u8] = include_bytes!("../kernel-driver/str.txt");
