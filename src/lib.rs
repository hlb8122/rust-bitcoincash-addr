mod base58;
mod cashaddr;
mod errors;

pub use base58::Base58Codec;
pub use cashaddr::CashAddrCodec;
pub use errors::*;

/// Bitcoin Networks
#[derive(PartialEq, Clone)]
pub enum Network {
    /// Main network
    Main,
    /// Test network
    Test,
    /// Regression test network
    Regtest,
}

/// Address encoding scheme
#[derive(PartialEq, Clone)]
pub enum Scheme {
    /// Base 58 encoding
    Base58,
    /// CashAddress encoding
    CashAddr,
}

/// Type of the hash160 raw bytes
#[derive(PartialEq, Clone)]
pub enum HashType {
    /// Public key hash
    Key,
    /// Script hash
    Script,
}

#[derive(PartialEq, Clone)]
pub struct Address {
    /// Address bytes
    pub body: Vec<u8>,
    /// Encoding scheme
    pub scheme: Scheme,
    /// Hash type
    pub hash_type: HashType,
    /// Network
    pub network: Network,
}

impl Default for Address {
    fn default() -> Self {
        Address {
            body: vec![],
            scheme: Scheme::CashAddr,
            hash_type: HashType::Key,
            network: Network::Main,
        }
    }
}

impl<'a> AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.body
    }
}

impl Address {
    /// Create a new address
    pub fn new(body: Vec<u8>, scheme: Scheme, hash_type: HashType, network: Network) -> Self {
        Address {
            body,
            scheme,
            hash_type,
            network,
        }
    }

    /// Take address bytes
    pub fn into_body(self) -> Vec<u8> {
        self.body
    }

    /// Attempt to convert the raw address bytes to a string
    pub fn encode(&self) -> Result<String, AddressError> {
        match self.scheme {
            Scheme::CashAddr => CashAddrCodec::encode(
                &self.body,
                self.hash_type.to_owned(),
                self.network.to_owned(),
            ),
            Scheme::Base58 => Base58Codec::encode(
                &self.body,
                self.hash_type.to_owned(),
                self.network.to_owned(),
            ),
        }
    }

    /// Attempt to convert an address string into bytes
    pub fn decode(addr_str: &str) -> Result<Self, AddressError> {
        CashAddrCodec::decode(addr_str).or_else(|_| Base58Codec::decode(addr_str))
    }
}

/// A codec encoding and decoding the `Address` struct
pub trait AddressCodec {
    /// Attempt to convert the raw address bytes to a string
    fn encode(raw: &[u8], hash_type: HashType, network: Network) -> Result<String, AddressError>;

    /// Attempt to convert the address string to bytes
    fn decode(s: &str) -> Result<Address, AddressError>;
}
