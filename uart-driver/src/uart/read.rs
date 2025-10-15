use crate::constants::EXPECTED_BYTES;
use crate::uart::uart::MiniUART;
use log::error;
use std::io::{stdout, Write};

#[cfg(feature = "performance")]
/// With the performance feature enabled, the following assumptions are made
/// - A byte can always be read
pub trait Reader {
    fn read_byte(&mut self) -> u8;
}

#[cfg(feature = "performance")]
impl Reader for MiniUART {
    fn read_byte(&mut self) -> u8 {
        self.regs.read_byte_unchecked()
    }
}

#[cfg(not(feature = "performance"))]
/// With safety checks enabled, the following assumptions are made
/// - Reading a byte can fail because LSR == 1 → No data ready
pub trait Reader {
    fn has_overrun(&mut self) -> bool;
    fn read_byte(&mut self) -> Option<u8>;
}

#[cfg(not(feature = "performance"))]
impl Reader for MiniUART {
    fn has_overrun(&mut self) -> bool {
        self.regs.has_overrun()
    }

    fn read_byte(&mut self) -> Option<u8> {
        if !self.regs.rx_byte_ready() {
            error!("Tried to read byte while none is ready");
            return None;
        }

        Some(self.regs.read_byte_unchecked())
    }
}

#[cfg(not(feature = "performance"))]
pub fn infinite_read(mut uart: MiniUART, out: &mut (impl Write + ?Sized)) -> ! {
    loop {
        uart.wait_for_byte();
        let Some(c) = uart.read_byte() else {
            continue;
        };

        out.write_all(&[c]).unwrap();
    }
}

#[cfg(feature = "performance")]
pub fn infinite_read(mut uart: MiniUART, out: &mut (impl Write + ?Sized)) -> ! {
    loop {
        uart.wait_for_byte();
        out.write_all(&[uart.read_byte()]).unwrap()
    }
}
