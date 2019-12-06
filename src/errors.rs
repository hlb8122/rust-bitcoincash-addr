use std::{error::Error, fmt};

/// Error concerning decoding of addresses.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AddressDecodingError {
    /// Base58 address error.
    Base58(Base58Error),
    /// CashAddr error.
    CashAddr(CashAddrDecodingError),
}

impl From<Base58Error> for AddressDecodingError {
    fn from(e: Base58Error) -> Self {
        AddressDecodingError::Base58(e)
    }
}

impl From<CashAddrDecodingError> for AddressDecodingError {
    fn from(e: CashAddrDecodingError) -> Self {
        AddressDecodingError::CashAddr(e)
    }
}

impl fmt::Display for AddressDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressDecodingError::Base58(ref e) => write!(f, "base58 error: {}", e),
            AddressDecodingError::CashAddr(ref e) => write!(f, "cashaddr error: {}", e),
        }
    }
}

impl Error for AddressDecodingError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            AddressDecodingError::Base58(ref e) => Some(e),
            AddressDecodingError::CashAddr(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            AddressDecodingError::Base58(_) => "base58 error: {}",
            AddressDecodingError::CashAddr(_) => "cashaddr error: {}",
        }
    }
}

/// Error concerning decoding of base58 addresses.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Base58Error {
    /// Unexpected character (char).
    InvalidChar(char),
    /// Checksum failed (expected, actual).
    ChecksumFailed { expected: Vec<u8>, actual: Vec<u8> },
    /// Invalid length (length).
    InvalidLength(usize),
    /// Version byte was not recognized.
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
    fn cause(&self) -> Option<&dyn Error> {
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

/// Error concerning decoding of cashaddrs.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CashAddrDecodingError {
    /// Invalid length (length).
    InvalidLength(usize),
    /// Zero or multiple prefixes.
    NoPrefix,
    /// Failed to match known prefixes (prefix).
    InvalidPrefix(String),
    /// Checksum failed (checksum).
    ChecksumFailed(u64),
    /// Unexpected character (char).
    InvalidChar(char),
    /// Version byte was not recognized.
    InvalidVersion(u8),
    /// Upper and lowercase address string.
    MixedCase,
}

impl fmt::Display for CashAddrDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CashAddrDecodingError::ChecksumFailed(actual) => {
                write!(f, "invalid checksum (actual {} != 0)", actual)
            }
            CashAddrDecodingError::InvalidChar(index) => write!(f, "invalid char ({})", index),
            CashAddrDecodingError::NoPrefix => write!(f, "zero or multiple prefixes"),
            CashAddrDecodingError::MixedCase => write!(f, "mixed case string"),
            CashAddrDecodingError::InvalidVersion(c) => write!(f, "invalid version byte ({})", c),
            CashAddrDecodingError::InvalidPrefix(prefix) => {
                write!(f, "invalid prefix ({})", prefix)
            }
            CashAddrDecodingError::InvalidLength(length) => {
                write!(f, "invalid length ({})", length)
            }
        }
    }
}

impl Error for CashAddrDecodingError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            CashAddrDecodingError::ChecksumFailed { .. } => "invalid checksum",
            CashAddrDecodingError::InvalidChar(_) => "invalid char",
            CashAddrDecodingError::NoPrefix => "zero or multiple prefixes",
            CashAddrDecodingError::MixedCase => "mixed case string",
            CashAddrDecodingError::InvalidVersion(_) => "invalid version byte",
            CashAddrDecodingError::InvalidPrefix(_) => "invalid prefix",
            CashAddrDecodingError::InvalidLength(_) => "invalid length",
        }
    }
}

/// Error concerning encoding of cashaddrs.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CashAddrInvalidLength(pub usize);

impl fmt::Display for CashAddrInvalidLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid length ({})", self.0)
    }
}

impl Error for CashAddrInvalidLength {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        "invalid length"
    }
}
