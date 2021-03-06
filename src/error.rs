use std::fmt;

use http::StatusCode;

use crate::couchdb;

#[derive(Debug)]
pub struct Error {
    status: Option<StatusCode>,
    message: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::from(message.to_string())
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error { message, source: None, status: None }
    }
}

impl From<couchdb::error::Error> for Error {
    fn from(err: couchdb::error::Error) -> Self {
        Error { message: err.to_string(), status: Some(err.status()), source: Some(Box::new(err)) }
    }
}
