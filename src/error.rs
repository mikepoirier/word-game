use std::{error::Error, fmt::Display};

use xmpp_parsers::JidParseError;

#[derive(Debug)]
pub enum AppError {
    XmppError(tokio_xmpp::Error),
    Unknown,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::XmppError(e) => write!(f, "XMPP Error: {}", e),
            AppError::Unknown => write!(f, "Unknown Error"),
        }
    }
}

impl Error for AppError {}

impl From<tokio_xmpp::Error> for AppError {
    fn from(e: tokio_xmpp::Error) -> Self {
        AppError::XmppError(e)
    }
}

impl From<JidParseError> for AppError {
    fn from(e: JidParseError) -> Self {
        AppError::XmppError(tokio_xmpp::Error::from(e))
    }
}
