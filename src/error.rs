use core::fmt;
use std::io;

#[derive(Debug)]
pub enum QFError {
    Io(io::Error),
    Message(String),
}

impl fmt::Display for QFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QFError::Io(error) => write!(f, "io error: {error}"),
            QFError::Message(message) => write!(f, "{message}"),
        }
    }
}

impl From<io::Error> for QFError {
    fn from(value: io::Error) -> Self {
        QFError::Io(value)
    }
}
