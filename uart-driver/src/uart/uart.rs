use crate::constants::{POLLING_DURATION, RESET_CHAR};
use crate::uart::registers::MiniUartRegs;
use crate::uart::stats::{InterruptStats, TransmissionStats, UARTStats};
use log::{debug, error, info, Log};
use std::io;
use std::io::{stdout, Write};
use std::thread::sleep;
use uio::UioDevice;

pub struct MiniUART {
    pub regs: &'static mut MiniUartRegs,
    pub uio: UioDevice,

    pub u_stats: UARTStats,
    pub i_stats: InterruptStats,
    pub t_stats: TransmissionStats,
}

impl MiniUART {
    pub fn from_uio(dev: UioDevice) -> MiniUART {
        let regs = unsafe { &mut *(dev.map_mapping(0).unwrap() as *mut MiniUartRegs) };
        MiniUART {
            regs,
            uio: dev,

            // TODO: Migrate this away
            u_stats: Default::default(),
            i_stats: Default::default(),
            t_stats: Default::default(),
        }
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
    pub fn reset_stats(&mut self) {
        self.u_stats = Default::default();
        self.i_stats = Default::default();
        self.t_stats = Default::default();
    }

    #[cfg(feature = "driver_irq")]
    pub fn wait_for_byte(&mut self) {
        #[cfg(feature = "io_data")]
        {
            self.uio.irq_enable().unwrap();
        }

        let _total_interrupts = self.uio.irq_wait();
    }

    #[cfg(feature = "driver_polling")]
    pub fn wait_for_byte(&mut self) {
        loop {
            if self.regs.rx_byte_ready() {
                break;
            }
        }
    }
}
