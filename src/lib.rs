//! # Bitcoin Cash Address Library
//!
//! A simple library providing an `Address` struct enabling
//! encoding/decoding of Bitcoin Cash addresses.
//!
//! ```
//! use bitcoincash_addr::{Address, Network, Scheme};
//!
//! fn main() {
//!     // Decode base58 address
//!     let legacy_addr = "1NM2HFXin4cEQRBLjkNZAS98qLX9JKzjKn";
//!     let mut addr = Address::decode(legacy_addr).unwrap();
//!
//!     // Change the base58 address to a test network cashaddr
//!     addr.network = Network::Test;
//!     addr.scheme = Scheme::CashAddr;
//!
//!     // Encode cashaddr
//!     let cashaddr_str = addr.encode().unwrap();
//!
//!     // bchtest:qr4zgpuznfg923ntyauyeh5v7333v72xhum2dsdgfh
//!     println!("{}", cashaddr_str);
//! }
//!
//! ```
//!

mod base58;
mod cashaddr;

pub use base58::Base58Codec;
pub use cashaddr::CashAddrCodec;

use std::marker::PhantomData;

/// Bitcoin Networks.
#[derive(PartialEq, Clone, Debug)]
pub enum Network {
    /// Main network.
    Main,
    /// Test network.
    Test,
    /// Regression test network.
    Regtest,
}

/// Intepretation of the Hash160 bytes.
#[derive(PartialEq, Clone, Debug)]
pub enum HashType {
    /// Public key hash
    Key,
    /// Script hash
    Script,
}

/// Struct containing the bytes and metadata of a Bitcoin Cash address.
/// This is yeilded during decoding or consumed during encoding.
#[derive(PartialEq, Clone, Debug)]
pub struct Address<C> {
    /// Address bytes
    pub body: Vec<u8>,
    /// Address Codec
    pub scheme: PhantomData<C>,
    /// Hash type
    pub hash_type: HashType,
    /// Network
    pub network: Network,
}

/// Creates an empty `Address` struct, with the `body` bytes the empty vector,
/// `Scheme::CashAddr`, `HashType::Key`, and `Network::Main`.
impl<Codec> Default for Address<Codec> {
    fn default() -> Self {
        Address {
            body: vec![],
            scheme: PhantomData::default(),
            hash_type: HashType::Key,
            network: Network::Main,
        }
    }
}

impl<Codec> Address<Codec> {
    /// Create a new address.
    pub fn new(body: Vec<u8>, hash_type: HashType, network: Network) -> Self {
        Address {
            body,
            scheme: PhantomData::default(),
            hash_type,
            network,
        }
    }

    pub fn into_base58(self) -> Address<Base58Codec> {
        Address {
            body: self.body,
            scheme: PhantomData::<Base58Codec>::default(),
            hash_type: self.hash_type,
            network: self.network,
        }
    }

    pub fn into_cashaddr(self) -> Address<CashAddrCodec> {
        Address {
            body: self.body,
            scheme: PhantomData::<CashAddrCodec>::default(),
            hash_type: self.hash_type,
            network: self.network,
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
}

impl<Codec: AddressCodec> Address<Codec> {
    /// Attempt to convert the raw address bytes to a string.
    pub fn encode(&self) -> Result<String, Codec::EncodingError> {
        Codec::encode(
            &self.body,
            self.hash_type.to_owned(),
            self.network.to_owned(),
        )
    }

    /// Attempt to convert an address string into bytes.
    pub fn decode(addr_str: &str) -> Result<Self, Codec::DecodingError> {
        Codec::decode(addr_str)
    }
}

/// A trait providing an interface for encoding and decoding the `Address` struct for each address scheme.
pub trait AddressCodec: Sized {
    type EncodingError;
    type DecodingError;

    /// Attempt to convert the raw address bytes to a string.
    fn encode(
        raw: &[u8],
        hash_type: HashType,
        network: Network,
    ) -> Result<String, Self::EncodingError>;

    /// Attempt to convert the address string to bytes.
    fn decode(s: &str) -> Result<Address<Self>, Self::DecodingError>;
}
