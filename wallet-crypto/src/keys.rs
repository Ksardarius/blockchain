use std::fmt::{self, Debug};

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

    pub fn to_bytes(&self) -> Vec<u8> {
        let enc = self.0.to_encoded_point(true);
        enc.as_bytes().to_vec()
    }

    pub fn verify(&self, message: &[u8], signature: &EcdsaSignature) -> bool {
        self.0.verify(message, signature).is_ok()
    }
}

impl fmt::LowerHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let enc = self.0.to_encoded_point(true);
        let public_key_bytes = enc.as_bytes();

        write!(f, "{}", hex::encode(public_key_bytes))
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

impl Debug for PublicKeyWithSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Public key: {:x}\nSignature: {:x}",
            self.public_key, self.signature
        )
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{
        scripts::Script,
        transaction::{DraftTransaction, TxOut, UnsignedTxIn},
    };

    use super::*;

    #[test]
    fn test_key_generation_and_signing() {
        let keypair = KeyPair::generate();
        println!(
            "Generated Public Key Hash: {}",
            hex::encode(keypair.public_key.to_address().as_ref())
        );
        println!("Generated Public Key: {:x}", keypair.public_key);
        println!(
            "Generated Secret Key: {}",
            hex::encode(keypair.secret_key.0.to_bytes())
        );

        let message = b"Hello, blockchain!";
        let signature = keypair.sign(message).unwrap();

        let sig = EcdsaSignature::from_slice(&signature).unwrap();
        assert!(keypair.verify(message, &sig));
    }

    #[test]
    fn test_transaction_hashing_and_signing() -> Result<(), Box<dyn Error>> {
        let keypair_alice = KeyPair::generate();
        let keypair_bob = KeyPair::generate();

        let txid_prev = BlockchainHash::default(); // Simulate a previous transaction ID

        let tx_in = UnsignedTxIn {
            prev_tx_id: txid_prev,
            prev_out_idx: 0,
            sequence: 0xFFFFFFFF,
        };

        let tx_out = TxOut {
            value: 100_000_000, // 1 coin
            script_pubkey: Script::PayToPublicKeyHash {
                pub_key_hash: keypair_bob.public_key.to_address(),
            },
        };

        let tx = DraftTransaction::new(vec![tx_in], vec![tx_out]);
        let tx = tx.sign(&keypair_alice);

        // Verify the transaction
        tx.verify_signatures()?;

        println!("Transaction ID: {}", tx.id);

        Ok(())
    }
}
