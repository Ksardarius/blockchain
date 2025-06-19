pub mod script;

use std::fmt;

use bincode::{Decode, Encode};
pub use script::Script;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    Encode,
    Decode,
)]
pub struct BlockchainHash([u8; 32]);

impl BlockchainHash {
    pub fn new(bytes: [u8; 32]) -> Self {
        BlockchainHash(bytes)
    }
    pub fn from_slice(slice: &[u8]) -> Result<Self, &'static str> {
        if slice.len() == 32 {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(slice);
            Ok(BlockchainHash(bytes))
        } else {
            Err("Slice length must be 32 bytes for Hash")
        }
    }
    pub fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
    // You might want a method to convert to hex string for display/debug
    pub fn to_string_owned(&self) -> String {
        hex::encode(self.0)
    }
    // And a default if needed
    pub fn default() -> Self {
        BlockchainHash([0u8; 32])
    }

    pub fn is_zero_hash(&self) -> bool {
        self.0 == [0u8; 32]
    }
}

impl fmt::Display for BlockchainHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?; // Format each byte as two hexadecimal characters
        }
        Ok(())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    Encode,
    Decode,
)]
pub struct PublicKeyHash([u8; 20]);

impl PublicKeyHash {
    pub fn new(bytes: [u8; 20]) -> Self {
        PublicKeyHash(bytes)
    }
    pub fn from_slice(slice: &[u8]) -> Result<Self, &'static str> {
        if slice.len() == 20 {
            let mut bytes = [0u8; 20];
            bytes.copy_from_slice(slice);
            Ok(PublicKeyHash(bytes))
        } else {
            Err("Slice length must be 20 bytes for PublicKeyHash")
        }
    }
    pub fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }

    pub fn to_string_owned(&self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for PublicKeyHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to your existing to_string_owned method for hex conversion
        write!(f, "{}", self.to_string_owned())
    }
}
