use crate::constants::RESET_SLEEP_DURATION;
use crate::uart::registers::CNTL::{RX_DISABLED, TX_DISABLED};
use crate::uart::stats::UARTStats;
use std::thread::sleep;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite};
use tock_registers::{register_bitfields, register_structs};

register_bitfields! {
    u32,

    EN [ UART OFFSET(0) NUMBITS(1) [] ],

    // IO Data (write = transmit, read = receive)
    IO [ DATA OFFSET(0) NUMBITS(8) [] ],

    // Interrupt Identify
    IIR [
        ID OFFSET(2) NUMBITS(2) [
            // TODO: How to express better in tock_registers
            None = 0b00,
            TX = 0b01,
            RX = 0b10,
        ],
    ],

    // Line Control
    LCR [ DATA_SIZE OFFSET(0) NUMBITS(2) [
        seven_bit = 0,
        eight_bit = 0b11,
    ] ],

    // Data Status  (relevant for polling)
    LSR [
        TX_IDLE OFFSET(6) NUMBITS(1) [],
        TX_EMPTY OFFSET(5) NUMBITS(1) [],

        RX_OVERRUN OFFSET(1) NUMBITS(1) [],
        RX_READY OFFSET(0) NUMBITS(1) [],
    ],


    SCRATCH [ DATA OFFSET(0) NUMBITS (8) [] ],

    // Extra Control
    CNTL [
        TX_DISABLED OFFSET(1) NUMBITS(1) [],
        RX_DISABLED OFFSET(0) NUMBITS(1) []
    ],

    // Extra Status
    STAT [
        TX_FIFO_NUMBYTES OFFSET(24) NUMBITS(4) [],
        RX_FIFO_NUMBYTES OFFSET(16) NUMBITS(4) [],

        TX_DONE OFFSET(9) NUMBITS(1) [],
        TX_FIFO_EMPTY OFFSET(8) NUMBITS(1) [],

        CTS OFFSET(7) NUMBITS(1) [],
        RTS OFFSET(6) NUMBITS(1) [],

        TX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_OVERRUN OFFSET(4) NUMBITS(1) [],

        TX_IDLE OFFSET(3) NUMBITS(1) [],
        RX_IDLE OFFSET(2) NUMBITS(1) [],

        TX_READY OFFSET(1) NUMBITS(1) [],
        RX_READY OFFSET(0) NUMBITS(1) [],
    ],

    // Baudrate
    BAUD [
        RATE OFFSET(0) NUMBITS(16) [],
    ],
}

register_structs! {
    #[allow(non_snake_case)]
    pub MiniUartRegs {
        (0x00 => _unused1),
        (0x04 => EN: ReadWrite<u32, EN::Register>),
        (0x08 => _unused2),
        (0x40 => IO: ReadWrite<u32, IO::Register>),
        (0x44 => _ier),  // we shouldn't interfere with the kernel state of enabling / disabling interrupts
        (0x48 => IIR: ReadWrite<u32, IIR::Register>),
        (0x4c => LCR: ReadWrite<u32, LCR::Register>),
        (0x50 => _unused3),
        (0x54 => LSR: ReadOnly<u32, LSR::Register>),
        (0x58 => _unused4),
        (0x5c => SCRATCH: ReadWrite<u32, SCRATCH::Register>),  // not used
        (0x60 => CNTL: ReadWrite<u32, CNTL::Register>),
        (0x64 => STAT: ReadOnly<u32, STAT::Register>),
        (0x68 => BAUD: ReadWrite<u32, BAUD::Register>),
        (0x6c => @END),
    }
}

/// Initialization
impl MiniUartRegs {
    pub fn init(&mut self, baudrate: u32) -> Result<(), &'static str> {
        self.reset();
        self.disable();
        self.clear();

        self.set_baudrate(baudrate);
        self.set_data_size();
        self.enable();

        // Wait for the circuit to stabilize
        sleep(RESET_SLEEP_DURATION);

        Ok(())
    }

    fn reset(&mut self) {
        self.EN.modify(EN::UART::CLEAR);
        sleep(RESET_SLEEP_DURATION);
        self.EN.modify(EN::UART::SET);

        // TODO: Clear FIFOS
    }

    fn enable(&mut self) {
        self.CNTL.write(TX_DISABLED::SET + RX_DISABLED::SET);
    }

    fn disable(&mut self) {
        self.CNTL.write(TX_DISABLED::CLEAR + RX_DISABLED::CLEAR);
    }

    fn clear(&mut self) {
        self.IIR.write(IIR::ID::TX);
        self.IIR.write(IIR::ID::RX);
    }

    /// The formula for the `baudrate_reg` is
    ///     baudrate     = system_clock_freq / (8 * (baudrate_reg + 1) )
    ///  ⇔  baudrate_reg = system_clock_freq / (8 * baudrate) - 1
    ///
    /// Because we round down to the next int, we skip the -1 (don't @ me)
    /// Also, the system clock is fixed to 250MHz in our setup.
    fn set_baudrate(&mut self, baudrate: u32) {
        let baudrate_reg = 250_000_000 / (8 * baudrate) - 1;
        self.BAUD.set(baudrate_reg);
    }

    fn set_data_size(&mut self) {
        self.LCR.write(LCR::DATA_SIZE::eight_bit)
    }
}

/// Public implementations
impl MiniUartRegs {
    pub fn has_overrun(&mut self) -> bool {
        self.LSR.is_set(LSR::RX_OVERRUN)
    }

    pub fn read_byte_unchecked(&mut self) -> u8 {
        #[cfg(feature = "io_data")]
        return self.IO.get() as u8;

        #[cfg(feature = "io_scratch")]
        {
            let c = self.SCRATCH.get() as u8;
            self.SCRATCH.set(0);
            c
        }
    }

    pub fn write_byte_unchecked(&mut self, data: u8) {
        #[cfg(feature = "io_data")]
        self.IO.set(data as u32);

        #[cfg(feature = "io_scratch")]
        self.SCRATCH.set(data as u32);
    }

    pub fn rx_byte_ready(&mut self) -> bool {
        #[cfg(feature = "io_data")]
        return self.LSR.is_set(LSR::RX_READY);

        #[cfg(feature = "io_scratch")]
        return self.SCRATCH.get() != 0;
    }

    pub fn tx_byte_ready(&mut self) -> bool {
        self.LSR.is_set(LSR::TX_EMPTY)
    }

    pub fn read_stats(&self) -> UARTStats {
        let reg = self.STAT.extract();

        UARTStats {
            tx_num_bytes: reg.read(STAT::TX_FIFO_NUMBYTES) as u8,
            rx_num_bytes: reg.read(STAT::RX_FIFO_NUMBYTES) as u8,
            tx_done: reg.is_set(STAT::TX_DONE),
            tx_empty: reg.is_set(STAT::TX_FIFO_EMPTY),
            cts: reg.is_set(STAT::CTS),
            rts: reg.is_set(STAT::RTS),
            tx_full: reg.is_set(STAT::TX_FIFO_FULL),
            rx_overrun: reg.is_set(STAT::RX_FIFO_OVERRUN),
            tx_idle: reg.is_set(STAT::TX_IDLE),
            rx_idle: reg.is_set(STAT::RX_IDLE),
            tx_ready: reg.is_set(STAT::TX_READY),
            rx_ready: reg.is_set(STAT::RX_READY),
        }
    }
}
