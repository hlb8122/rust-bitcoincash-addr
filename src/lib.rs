//! # Bitcoin Cash Address Library
//!
//! A simple library providing an `Address` struct enabling 
//! encoding/decoding of Bitcoin Cash addresses.
//! 
//! ```
//! use bitcoincash_addr::{Address, Network, Scheme};
//! 
//! fn main() {
//!     // Decode base 58 address
//!     let legacy_addr = "1NM2HFXin4cEQRBLjkNZAS98qLX9JKzjKn";
//!     let mut addr = Address::decode(legacy_addr).unwrap();
//! 
//!     // Change the base 58 address to a test network cashaddr
//!     addr.network = Network::Test;
//!     addr.scheme = Scheme::CashAddr;
//! 
//!     // Encode cashaddr
//!     let cash_addr = addr.encode().unwrap();
//! 
//!     // bchtest:qr4zgpuznfg923ntyauyeh5v7333v72xhum2dsdgfh
//!     println!("{}", cash_addr);
//! }
//! 
//! ```
//! 

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

/// Struct containing address bytes and metadata required to encode an address 
/// string and yielded via decoding.
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

/// Creates an empty `Address` struct, with the `body` bytes the empty vector, 
/// `Scheme::CashAddr`, `HashType::Key`, and `Network::Main`.
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

/// A trait providing an interface for encoding and decoding the `Address` struct
/// for each address scheme.
pub trait AddressCodec {
    /// Attempt to convert the raw address bytes to a string
    fn encode(raw: &[u8], hash_type: HashType, network: Network) -> Result<String, AddressError>;

    /// Attempt to convert the address string to bytes
    fn decode(s: &str) -> Result<Address, AddressError>;
}
