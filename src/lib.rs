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
#[derive(PartialEq, Clone)]
pub enum Scheme {
    /// Base 58 encoding
    Base58,
    /// CashAddress encoding
    CashAddr,
}

#[derive(PartialEq, Clone)]
pub enum HashType {
    /// Public key hash
    Key,
    /// Script hash
    Script,
}

#[derive(PartialEq, Clone)]
pub struct Address {
    body: Vec<u8>,
    scheme: Scheme,
    hash_type: HashType,
    network: Network,
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
    pub fn new(body: Vec<u8>, scheme: Scheme, hash_type: HashType, network: Network) -> Self {
        Address {
            body,
            scheme,
            hash_type,
            network,
        }
    }

    pub fn into_body(self) -> Vec<u8> {
        self.body
    }

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

    pub fn decode(addr_str: &str) -> Result<Self, AddressError> {
        CashAddrCodec::decode(addr_str).or_else(|_| Base58Codec::decode(addr_str))
    }
}

pub trait AddressCodec {
    fn encode(raw: &[u8], hash_type: HashType, network: Network) -> Result<String, AddressError>;

    fn decode(s: &str) -> Result<Address, AddressError>;
}
