use std::{error::Error, fmt};

/// Error concerning encoding/decoding of base58 addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Base58Error {
    /// Unexpected character (char)
    InvalidChar(char),
    /// Checksum failed (expected, actual)
    ChecksumFailed { expected: Vec<u8>, actual: Vec<u8> },
    /// Invalid length (length)
    InvalidLength(usize),
    /// Version byte was not recognized
    InvalidVersion(u8),
}

impl fmt::Display for Base58Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Base58Error::InvalidChar(b) => write!(f, "invalid char ({})", b),
            Base58Error::ChecksumFailed { expected, actual } => write!(
                f,
                "invalid checksum (actual {:?} does not match expected {:?})",
                actual, expected
            ),
            Base58Error::InvalidLength(length) => write!(f, "invalid length ({})", length),
            Base58Error::InvalidVersion(v) => write!(f, "invalid version byte ({})", v),
        }
    }
}

impl Error for Base58Error {
    fn cause(&self) -> Option<&Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            Base58Error::InvalidChar(_) => "invalid char",
            Base58Error::ChecksumFailed { .. } => "invalid checksum",
            Base58Error::InvalidLength(_) => "invalid length",
            Base58Error::InvalidVersion(_) => "invalid version",
        }
    }
}

/// Error concerning encoding/decoding of cashaddrs
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CashAddrError {
    /// Invalid length (length)
    InvalidLength(usize),
    /// Zero or multiple prefixes
    NoPrefix,
    /// Failed to match known prefixes (prefix)
    InvalidPrefix(String),
    /// Checksum failed (checksum)
    ChecksumFailed(u64),
    /// Unexpected character (char)
    InvalidChar(char),
    /// Version byte was not recognized
    InvalidVersion(u8),
    /// Upper and lowercase address string
    MixedCase,
}

impl fmt::Display for CashAddrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CashAddrError::ChecksumFailed(actual) => {
                write!(f, "invalid checksum (actual {} != 0)", actual)
            }
            CashAddrError::InvalidChar(index) => write!(f, "invalid char ({})", index),
            CashAddrError::NoPrefix => write!(f, "zero or multiple prefixes"),
            CashAddrError::MixedCase => write!(f, "mixed case string"),
            CashAddrError::InvalidVersion(c) => write!(f, "invalid version byte ({})", c),
            CashAddrError::InvalidPrefix(prefix) => write!(f, "invalid prefix ({})", prefix),
            CashAddrError::InvalidLength(length) => write!(f, "invalid length ({})", length),
        }
    }
}

impl Error for CashAddrError {
    fn cause(&self) -> Option<&Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            CashAddrError::ChecksumFailed { .. } => "invalid checksum",
            CashAddrError::InvalidChar(_) => "invalid char",
            CashAddrError::NoPrefix => "zero or multiple prefixes",
            CashAddrError::MixedCase => "mixed case string",
            CashAddrError::InvalidVersion(_) => "invalid version byte",
            CashAddrError::InvalidPrefix(_) => "invalid prefix",
            CashAddrError::InvalidLength(_) => "invalid length",
        }
    }
}

/// Error concerning encoding/decoding of addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AddressError {
    /// Base58 address error
    Base58(Base58Error),
    /// CashAddr error
    CashAddr(CashAddrError),
}

impl From<Base58Error> for AddressError {
    fn from(e: Base58Error) -> AddressError {
        AddressError::Base58(e)
    }
}

impl From<CashAddrError> for AddressError {
    fn from(e: CashAddrError) -> AddressError {
        AddressError::CashAddr(e)
    }
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressError::Base58(ref e) => write!(f, "base58 error: {}", e),
            AddressError::CashAddr(ref e) => write!(f, "cashaddr error: {}", e),
        }
    }
}

impl Error for AddressError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            AddressError::Base58(ref e) => Some(e),
            AddressError::CashAddr(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            AddressError::Base58(_) => "base58 error",
            AddressError::CashAddr(_) => "cashaddr error",
        }
    }
}
