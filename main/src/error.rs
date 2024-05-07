use std::fmt::{Display, Formatter};

use crate::{reader::ReaderError, tags::TagError};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum GeneralErrorKind {
    IncorrectProject(String),
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GeneralError {
    pub kind: GeneralErrorKind,
    pub message: String,
}

impl Display for GeneralError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.kind {
            GeneralErrorKind::IncorrectProject(project) => {
                write!(f, "Could not find project {}", project)
            }
            GeneralErrorKind::Unknown => write!(
                f,
                "Encountered an unexpected error. Message: {}",
                self.message
            ),
        }
    }
}

impl ToString for GeneralErrorKind {
    fn to_string(&self) -> String {
        match self {
            GeneralErrorKind::IncorrectProject(_) => String::from("IncorrectProject"),
            GeneralErrorKind::Unknown => String::from("Unknown"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum CirclesErrorType {
    GeneralError,
    ReaderError,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CirclesError {
    pub error_type: CirclesErrorType,
    pub kind: String,
    pub message: String,
}

impl From<ReaderError> for CirclesError {
    fn from(error: ReaderError) -> Self {
        CirclesError {
            error_type: CirclesErrorType::ReaderError,
            kind: error.kind.to_string(),
            message: error.message,
        }
    }
}

impl From<GeneralError> for CirclesError {
    fn from(error: GeneralError) -> Self {
        CirclesError {
            error_type: CirclesErrorType::GeneralError,
            kind: error.kind.to_string(),
            message: error.message,
        }
    }
}
