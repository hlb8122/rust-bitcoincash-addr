#[derive(Debug, Clone)]
pub enum Base58Error {
    Non58,
    InvalidLength,
    InvalidChecksum,
    InvalidNetwork,
}
#[derive(Debug, Clone)]
pub enum CashAddrError {
    InvalidLength,
    NoPrefix,
    InvalidPrefix,
    InvalidNetwork,
    InvalidChecksum,
    InvalidChar,
    InvalidVersion,
    MixedCase,
    Empty,
}

#[derive(Debug, Clone)]
pub enum AddressError {
    Base58(Base58Error),
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
