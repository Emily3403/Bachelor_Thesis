use crate::logger::time::get_time;
use crate::logger::{InterruptCount, TimeStamp};
use crate::uart::registers::MiniUartRegs;
use crate::uart::stats::UARTStats;
use uio::UioDevice;

pub struct MiniUART {
    pub regs: &'static mut MiniUartRegs,
    pub uio: UioDevice,
}

/// Private API
impl MiniUART {
    #[cfg(feature = "driver_irq")]
    fn wait_for_byte(&mut self) -> (InterruptCount, TimeStamp) {
        #[cfg(feature = "io_data")]
        self.uio.irq_enable().unwrap();

        (self.uio.irq_wait().unwrap(), get_time())
    }

    #[cfg(feature = "driver_polling")]
    fn wait_for_byte(&mut self) -> TimeStamp {
        loop {
            if self.regs.rx_byte_ready() {
                break;
            }
        }
    }

    /// Initializes registers of the UART and might enable interrupts
    fn init(&mut self, baudrate: u32) -> Result<(), &'static str> {
        self.regs.init(baudrate)?;

        #[cfg(feature = "driver_irq")]
        self.uio.irq_enable().unwrap();

        Ok(())
    }
}

/// Public API
impl MiniUART {
    /// Creates the MiniUART Memory mapping and crashes the program if that was unsuccessful
    pub fn new(baudrate: u32) -> MiniUART {
        let dev = UioDevice::try_new(0).unwrap();
        let regs = unsafe { &mut *(dev.map_mapping(0).unwrap() as *mut MiniUartRegs) };
        let mut it = MiniUART { regs, uio: dev };

        it.init(baudrate).unwrap();
        it
    }

    /// Gets you the next byte that is transmitted (blocking) together with the timestamp that it was taken
    pub fn get_byte(&mut self) -> (u8, InterruptCount, TimeStamp) {
        let (i_count, time) = self.wait_for_byte();
        let data = self.regs.read_byte_unchecked();

        (data, i_count, time)
    }

    pub fn _get_stats(&self) -> UARTStats {
        self.regs.read_stats()
    }

    /// "Real" Method that might do subsampling of uart stats
    pub fn get_stats(&self) -> Option<UARTStats> {
        Some(self._get_stats())
    }
}
