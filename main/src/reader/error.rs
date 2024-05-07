use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum ReaderErrorKind {
    IncorrectHostname(String),
    CouldNotConnect(String),
    LostConnection,
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ReaderError {
    pub kind: ReaderErrorKind,
    pub message: String,
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.kind {
            ReaderErrorKind::IncorrectHostname(hostname) => write!(
                f,
                "Hostname {} is incorrect. Original error: {}",
                hostname, self.message
            ),
            ReaderErrorKind::CouldNotConnect(hostname) => write!(
                f,
                "Could not connect to hostname {}. Original error: {}",
                hostname, self.message
            ),
            ReaderErrorKind::LostConnection => write!(f, "Connection with the reader was lost!"),
            ReaderErrorKind::Unknown => write!(
                f,
                "Encountered an unexpected error in the reader. Message: {}",
                self.message
            ),
        }
    }
}

impl ToString for ReaderErrorKind {
    fn to_string(&self) -> String {
        match self {
            ReaderErrorKind::IncorrectHostname(_) => String::from("IncorrectHostname"),
            ReaderErrorKind::CouldNotConnect(_) => String::from("CouldNotConnect"),
            ReaderErrorKind::LostConnection => String::from("LostConnection"),
            ReaderErrorKind::Unknown => String::from("Unknown"),
        }
    }
}
