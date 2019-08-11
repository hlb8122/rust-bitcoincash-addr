use bitcoin_hashes::{sha256d::Hash as Sha256d, Hash};
use rust_base58::base58::{FromBase58, ToBase58};

use crate::*;

/// Codec allowing the encoding and decoding of base58 addresses
pub struct Base58Codec;

impl AddressCodec for Base58Codec {
    fn encode(raw: &[u8], hash_type: HashType, network: Network) -> Result<String, AddressError> {
        let addr_type_byte = match (hash_type, network) {
            (HashType::Key, Network::Main) => 0x00,
            (HashType::Key, Network::Test) => 0x6f,
            (HashType::Key, Network::Regtest) => 0x6f,
            (HashType::Script, Network::Main) => 0x05,
            (HashType::Script, Network::Test) => 0xc4,
            (HashType::Script, Network::Regtest) => 0xc4,
        };

        let mut body = Vec::with_capacity(raw.len() + 5);
        body.push(addr_type_byte);
        body.extend(raw);

        let checksum = Sha256d::hash(&body);
        body.extend(&checksum[0..4]);
        Ok(body.to_base58())
    }

    fn decode(addr_str: &str) -> Result<Address, AddressError> {
        // Convert from base58
        let raw = addr_str.from_base58().map_err(|_| Base58Error::Non58)?;
        if raw.len() != 25 {
            return Err(Base58Error::InvalidLength.into());
        }

        // Parse network and hash type
        let (network, hash_type) = match raw[0] {
            0x00 => (Network::Main, HashType::Key),
            0x05 => (Network::Main, HashType::Script),
            0x6f => (Network::Test, HashType::Key),
            0xc4 => (Network::Test, HashType::Script),
            _ => return Err(Base58Error::InvalidNetwork.into()),
        };

        // Verify checksum
        let payload = &raw[0..raw.len() - 4];
        let checksum_actual = &raw[raw.len() - 4..];
        let checksum_expected = Sha256d::hash(payload);
        if &checksum_expected[0..4] != checksum_actual {
            return Err(Base58Error::InvalidChecksum.into());
        }

        // Extract hash160 address and return
        let body = payload[1..].to_vec();
        Ok(Address {
            scheme: Scheme::Base58,
            body,
            hash_type,
            network,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin_hashes::hash160::Hash as Hash160;
    use hex;

    #[test]
    fn to_legacyaddr() {
        let pubkey_hex = "04005937fd439b3c19014d5f328df8c7ed514eaaf41c1980b8aeab461dffb23fbf3317e42395db24a52ce9fc947d9c22f54dc3217c8b11dfc7a09c59e0dca591d3";
        let pubkeyhash = Hash160::hash(&hex::decode(pubkey_hex).unwrap()).to_vec();
        let legacyaddr = Base58Codec::encode(&pubkeyhash, HashType::Key, Network::Main).unwrap();
        assert!(legacyaddr == "1NM2HFXin4cEQRBLjkNZAS98qLX9JKzjKn");
    }

    #[test]
    fn from_legacyaddr() {
        let legacyaddr = "1NM2HFXin4cEQRBLjkNZAS98qLX9JKzjKn";
        let result = Base58Codec::decode(legacyaddr).unwrap();
        let hash160 = result.as_ref();
        assert!(hex::encode(hash160) == "ea2407829a5055466b27784cde8cf463167946bf");
    }

    #[test]
    fn from_legacyaddr_errors() {
        assert!(Base58Codec::decode("0").is_err());
        assert!(Base58Codec::decode("1000000000000000000000000000000000").is_err());
    }
}
