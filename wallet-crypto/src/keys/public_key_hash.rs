use std::{fmt};

use bincode::{Decode, Encode};

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
    pub fn try_from_string(data: &str) -> Result<Self, &'static str> {
        let result = hex::decode(data).map_err(|_| "Incorrect hex string")?;
        Self::from_slice(result.as_slice())

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

impl fmt::LowerHex for PublicKeyHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

impl From<[u8; 20]> for PublicKeyHash {
    fn from(bytes: [u8; 20]) -> Self {
        PublicKeyHash(bytes)
    }
}
