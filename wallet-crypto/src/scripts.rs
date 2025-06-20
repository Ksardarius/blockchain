use bincode::Encode;
use serde::{Deserialize, Serialize};

use crate::keys::PublicKeyHash;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode)]
pub enum Script {
    PayToPublicKeyHash { pub_key_hash: PublicKeyHash }, // Add other script types (e.g., PayToScriptHash) later if needed
}
