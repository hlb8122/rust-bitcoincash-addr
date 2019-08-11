/// Error concerning encoding/decoding of base58 addresses
#[derive(Debug, Clone)]
pub enum Base58Error {
    /// Address string contained non-base 58 characters
    Non58,
    /// Invalid length
    InvalidLength,
    /// Checksum failed
    ChecksumFailed,
    /// Failed to match known networks
    InvalidNetwork,
}
/// Error concerning encoding/decoding of cashaddrs
#[derive(Debug, Clone)]
pub enum CashAddrError {
    /// Invalid length
    InvalidLength,
    /// Zero or multiple prefixes
    NoPrefix,
    /// Failed to match known prefixes
    InvalidPrefix,
    /// Failed to match known networks
    InvalidNetwork,
    /// Checksum failed
    ChecksumFailed,
    /// Address string contained an unexpected character
    InvalidChar,
    /// Failed to determine hash type from version byte
    InvalidVersion,
    /// Upper and lowercase address string
    MixedCase,
}

/// Error concerning encoding/decoding of addresses
#[derive(Debug, Clone)]
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
