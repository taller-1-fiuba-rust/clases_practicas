use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ChatError {
    InvalidParameters,
    NetworkError(std::io::Error),
    MessageError,
    InternalError,
}

impl Display for ChatError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ChatError::InvalidParameters => write!(f, "Invalid Parameters"),
            ChatError::NetworkError(e) => write!(f, "Network Error: {}", e),
            ChatError::MessageError => write!(f, "Mensaje invalido"),
            ChatError::InternalError => write!(f, "Error inesperado"),
        }
    }
}

impl std::error::Error for ChatError {}

impl From<std::io::Error> for ChatError {
    fn from(error: std::io::Error) -> Self {
        ChatError::NetworkError(error)
    }
}

impl From<std::fmt::Error> for ChatError {
    fn from(_error: std::fmt::Error) -> Self {
        ChatError::InternalError
    }
}
