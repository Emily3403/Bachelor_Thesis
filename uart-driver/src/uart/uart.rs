use crate::uart::registers::MiniUartRegs;
use crate::uart::stats::UARTStats;
use uio::UioDevice;

pub struct MiniUART {
    pub regs: &'static mut MiniUartRegs,
    pub uio: UioDevice,
}

impl MiniUART {
    pub fn read_stats(&self) -> UARTStats {
        self.regs.read_stats()
    }
}

/// Private API
impl MiniUART {
    #[cfg(feature = "driver_irq")]
    fn wait_for_byte(&mut self) {
        #[cfg(feature = "io_data")]
        self.uio.irq_enable().unwrap();

        let _total_interrupts = self.uio.irq_wait();
    }

    #[cfg(feature = "driver_polling")]
    fn wait_for_byte(&mut self) {
        loop {
            if self.regs.rx_byte_ready() {
                break;
            }
        }
    }
}

impl MiniUART {
    pub fn from_uio(dev: UioDevice) -> MiniUART {
        let regs = unsafe { &mut *(dev.map_mapping(0).unwrap() as *mut MiniUartRegs) };
        MiniUART { regs, uio: dev }
    }

    pub fn init(&mut self, baudrate: u32) -> Result<(), &'static str> {
        self.regs.init(baudrate)?;

        #[cfg(feature = "driver_irq")]
        self.uio.irq_enable().unwrap();

        Ok(())
    }
}

/// Public API
impl MiniUART {
    /// Get's you the next byte that is transmitted (blocking)
    pub fn get_byte(&mut self) -> u8 {
        self.wait_for_byte();
        self.regs.read_byte_unchecked()
    }
}
