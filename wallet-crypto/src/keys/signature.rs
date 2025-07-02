use bincode::{Decode, Encode};
use k256::ecdsa::{Signature as EcdsaSignature, VerifyingKey};

use crate::{
    crypto::{calculate_p2pkh_hash, parse_p2pkh_script_sig_k256},
    keys::{PublicKey, PublicKeyWithSignature},
};

#[derive(Debug, thiserror::Error)]
pub enum SignatureError {
    #[error("Failed to parse script signature: {0}")]
    ScriptSigParseError(String),
    #[error("Invalid signature format: {0}")]
    InvalidSignatureFormat(String),
    #[error("Invalid public key format: {0}")]
    InvalidPublicKeyFormat(String),
    #[error("Invalid script execution: {0}")]
    InvalidScript(String),
}

#[derive(
    Debug,
    Clone,
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
pub struct Signature(Vec<u8>);

impl Signature {
    pub fn build(mut signature: Vec<u8>, public_key: &PublicKey) -> Self {
        signature.extend_from_slice(&public_key.to_bytes());
        Self(signature)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Signature(bytes.to_vec())
    }

    pub fn get_verifier(&self) -> Result<PublicKeyWithSignature, SignatureError> {
        let (signature_bytes, public_key_bytes) =
            parse_p2pkh_script_sig_k256(&self.0).map_err(|e| {
                SignatureError::ScriptSigParseError(format!(
                    "Failed to parse P2PKH script_sig: {}",
                    e
                ))
            })?;

        let derived_pub_key_hash = calculate_p2pkh_hash(&public_key_bytes);

        let verifying_key = VerifyingKey::from_sec1_bytes(&public_key_bytes).map_err(|e| {
            SignatureError::InvalidPublicKeyFormat(format!("Invalid public key format: {}", e))
        })?;

        let signature = EcdsaSignature::from_slice(&signature_bytes).map_err(|e| {
            SignatureError::InvalidSignatureFormat(format!("Invalid signature format: {}", e))
        })?;

        let public_key = PublicKey(verifying_key);

        Ok(PublicKeyWithSignature::new(
            derived_pub_key_hash,
            public_key,
            signature,
        ))
    }
}
