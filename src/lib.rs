//! # Bitcoin Cash Address Library
//!
//! A simple library providing an `Address` struct enabling
//! encoding/decoding of Bitcoin Cash addresses.
//!
//! ```
//! use bitcoincash_addr::{Address, Network, AnyCodec, CashAddrCodec};
//!
//! fn main() {
//!     // Decode base58 address
//!     let legacy_addr = "1NM2HFXin4cEQRBLjkNZAS98qLX9JKzjKn";
//!     let mut addr = Address::decode::<AnyCodec>(legacy_addr).unwrap();
//!
//!     // Change the base58 address to a test network cashaddr
//!     addr.network = Network::Test;
//!
//!     // Encode cashaddr
//!     let cashaddr_str = addr.encode::<CashAddrCodec>().unwrap();
//!
//!     // bchtest:qr4zgpuznfg923ntyauyeh5v7333v72xhum2dsdgfh
//!     println!("{}", cashaddr_str);
//! }
//!
//! ```
//!

pub mod base58;
pub mod cashaddr;

pub use base58::Base58Codec;
pub use cashaddr::CashAddrCodec;

/// Bitcoin Networks.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Network {
    /// Main network.
    Main,
    /// Test network.
    Test,
    /// Regression test network.
    Regtest,
}

/// Intepretation of the Hash160 bytes.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum HashType {
    /// Public key hash
    Key,
    /// Script hash
    Script,
}

/// Struct containing the bytes and metadata of a Bitcoin Cash address.
/// This is yeilded during decoding or consumed during encoding.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Address {
    /// Address body
    pub body: Vec<u8>,
    /// Hash type
    pub hash_type: HashType,
    /// Network
    pub network: Network,
}

impl Default for Address {
    /// Creates an empty [`Address`] struct, with the `body` bytes the empty vector,
    /// [`Scheme::CashAddr`], [`HashType::Key`], and [`Network::Main`].
    fn default() -> Self {
        Address {
            body: Vec::new(),
            hash_type: HashType::Key,
            network: Network::Main,
        }
    }
}

impl Address {
    /// Create a new address.
    pub fn new(body: Vec<u8>, hash_type: HashType, network: Network) -> Self {
        Address {
            body,
            hash_type,
            network,
        }
    }

    /// Borrow address bytes.
    pub fn as_body(&self) -> &[u8] {
        &self.body
    }

    /// Take address bytes.
    pub fn into_body(self) -> Vec<u8> {
        self.body
    }

    /// Attempt to convert the raw address bytes to a string.
    pub fn encode<C: AddressEncoder>(&self) -> Result<String, C::EncodingError> {
        C::encode(&self.body, self.hash_type, self.network)
    }

    /// Attempt to convert an address string into bytes.
    pub fn decode<C: AddressDecoder>(addr_str: &str) -> Result<Self, C::DecodingError> {
        C::decode(addr_str)
    }
}

/// A trait providing an interface for encoding and decoding the `Address` struct for each address scheme.
pub trait AddressEncoder {
    type EncodingError;

    /// Attempt to convert the raw address bytes to a string.
    fn encode(
        raw: &[u8],
        hash_type: HashType,
        network: Network,
    ) -> Result<String, Self::EncodingError>;
}

pub trait AddressDecoder {
    type DecodingError;

    /// Attempt to convert the address string to bytes.
    fn decode(s: &str) -> Result<Address, Self::DecodingError>;
}

/// Codec allowing the encoding and decoding of either Base58 addresses or CashAddrs.
pub struct AnyCodec;

impl AddressDecoder for AnyCodec {
    type DecodingError = (cashaddr::DecodingError, base58::DecodingError);

    fn decode(s: &str) -> Result<Address, Self::DecodingError> {
        CashAddrCodec::decode(s)
            .or_else(|cash_err| Base58Codec::decode(s).map_err(|base58_err| (cash_err, base58_err)))
    }
}
