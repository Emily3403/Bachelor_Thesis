use crate::constants::{POLLING_DURATION, RESET_CHAR};
use crate::uart::err::TransmissionResult;
use crate::uart::uart::MiniUART;
use log::{debug, error, info};
use std::error::Error;
use std::fmt;
use std::fmt::{write, Debug, Display, Formatter};
use std::io::{stdout, Write};
use std::string::FromUtf8Error;
use std::thread::sleep;

pub trait RxTransmission {
    fn has_overrun(&mut self) -> bool;
    fn read_byte(&mut self) -> Option<u8>;
    fn wait_for_char(&mut self);

    // TODO: How to transmit stats?
    fn read_until_reset(&mut self) -> TransmissionResult<String> {
        let mut buf = Vec::with_capacity(0);

        loop {
            self.wait_for_char();
            let Some(c) = self.read_byte() else {
                continue;
            };

            if c == RESET_CHAR {
                break;
            }

            #[cfg(not(feature = "performance"))]
            if self.has_overrun() {
                error!("Receiver has overrun!");
            }
            buf.push(c);
        }

        Ok(String::from_utf8(buf)?)
    }
}

