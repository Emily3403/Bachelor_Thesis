use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Default, Clone)]
pub struct InterruptStats {
    pub total_interrupts: u32,
    pub missed_interrupts: u32,
}

#[derive(Default, Clone)]
pub struct TransmissionStats {
    pub bytes_sent: u32,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UARTStats {
    pub tx_num_bytes: u8, // 0-8
    pub rx_num_bytes: u8, // 0-8
    pub tx_done: bool,
    pub tx_empty: bool,

    pub cts: bool,
    pub rts: bool,

    pub tx_full: bool,
    pub rx_overrun: bool,

    pub tx_idle: bool,
    pub rx_idle: bool,

    pub tx_ready: bool,
    pub rx_ready: bool,
}

impl Display for UARTStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

impl Display for InterruptStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "total={}; missed={}", self.total_interrupts, self.missed_interrupts)
    }
}

impl Display for TransmissionStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "sent={}", self.bytes_sent)
    }
}
