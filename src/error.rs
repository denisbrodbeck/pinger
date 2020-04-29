//! error types

use std::fmt;

#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    WinpingError(winping::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::IoError(ref e) => e.fmt(f),
            CliError::ParseIntError(ref e) => e.fmt(f),
            CliError::WinpingError(ref e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for CliError {
    fn from(error: std::num::ParseIntError) -> Self {
        CliError::ParseIntError(error)
    }
}

impl From<winping::Error> for CliError {
    fn from(error: winping::Error) -> Self {
        CliError::WinpingError(error)
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            CliError::IoError(ref e) => Some(e),
            CliError::ParseIntError(ref e) => Some(e),
            CliError::WinpingError(ref e) => Some(e),
        }
    }
}
