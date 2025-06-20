use crate::crypto::calculate_p2pkh_hash;
use k256::ecdsa::{
    Signature as EcdsaSignature, SigningKey, VerifyingKey,
    signature::{Signer, Verifier},
};
use rand::rngs::OsRng;

mod blockchain_hash;
mod public_key_hash;
mod signature;

pub use blockchain_hash::BlockchainHash;
pub use public_key_hash::PublicKeyHash;
pub use signature::{Signature, SignatureError};

#[derive(Debug, Clone, /*Serialize, Deserialize, */ PartialEq, Eq)]
// #[serde(transparent)]
pub struct SecretKey(pub SigningKey);

#[derive(Debug, Clone, /*Serialize, Deserialize,*/ PartialEq, Eq /*Hash */)]
// #[serde(transparent)]
pub struct PublicKey(pub VerifyingKey);

impl PublicKey {
    pub fn to_address(&self) -> PublicKeyHash {
        let enc = self.0.to_encoded_point(true);
        let public_key_bytes = enc.as_bytes();
        calculate_p2pkh_hash(public_key_bytes)
    }

    pub fn verify(&self, message: &[u8], signature: &EcdsaSignature) -> bool {
        self.0.verify(message, signature).is_ok()
    }
}

pub struct PublicKeyWithSignature {
    pub pub_key_hash: PublicKeyHash,
    public_key: PublicKey,
    signature: EcdsaSignature,
}

impl PublicKeyWithSignature {
    pub fn new(
        pub_key_hash: PublicKeyHash,
        public_key: PublicKey,
        signature: EcdsaSignature,
    ) -> Self {
        PublicKeyWithSignature {
            pub_key_hash,
            public_key,
            signature,
        }
    }

    pub fn verify(&self, msg: &[u8]) -> Result<(), SignatureError> {
        if self.public_key.verify(msg, &self.signature) {
            Ok(())
        } else {
            Err(SignatureError::InvalidScript(
                "Invalid signature for transaction input".to_string(),
            ))
        }
    }
}

#[derive(Debug, Clone, /*Serialize, Deserialize,*/ PartialEq, Eq)]
pub struct KeyPair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key().clone();

        KeyPair {
            secret_key: SecretKey(signing_key),
            public_key: PublicKey(verifying_key),
        }
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        let signature: EcdsaSignature = self.secret_key.0.sign(message);
        Ok(signature.to_vec())
    }

    pub fn verify(&self, message: &[u8], signature: &EcdsaSignature) -> bool {
        self.public_key.verify(message, signature)
    }
}
