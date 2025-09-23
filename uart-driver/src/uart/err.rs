use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum TransmissionError {
    FifOverflow,  // :o
    UTF8Encoding(FromUtf8Error)
}

pub type TransmissionResult<T> = Result<T, TransmissionError>;

impl Error for TransmissionError {}

impl Display for TransmissionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TransmissionError::FifOverflow => write!(f, "FIFO Overflow"),
            TransmissionError::UTF8Encoding(it) => write!(f, "UTF8 Encoding Error: {it}")
        }
    }
}

impl From<FromUtf8Error> for TransmissionError {
    fn from(value: FromUtf8Error) -> Self {
        Self::UTF8Encoding(value)
    }
}