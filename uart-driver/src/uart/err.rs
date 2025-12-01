use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RxError {
    ChecksumMismatch,
    InvalidSeqNum,  // TODO: This would require too keep track of transmission state
}

pub type RxResult<T> = Result<T, RxError>;

impl Error for RxError {}

impl Display for RxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RxError::ChecksumMismatch => write!(f, "ChecksumMismatch"),
            RxError::InvalidSeqNum => write!(f, "InvalidSeqNum"),
        }
    }
}
