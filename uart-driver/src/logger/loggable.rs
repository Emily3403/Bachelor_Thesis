use crate::logger::Loggable;
use crate::uart::packet::Packet;
use std::fmt::Formatter;


impl Loggable for Packet {
    fn log(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}