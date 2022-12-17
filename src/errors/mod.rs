pub mod errors;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error.")]
    Internal(String),
    #[error("Not found.")]
    NotFound,
    #[error("Permission Denied.")]
    PermissionDenied,
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Internal(err.to_string()),
        }
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<web3::Error> for Error {
    fn from(err: web3::Error) -> Self {
        match err {
            web3::Error::Unreachable => Error::InvalidArgument(err.to_string()),
            _ => Error::Internal(err.to_string()),
        }
    }
}
