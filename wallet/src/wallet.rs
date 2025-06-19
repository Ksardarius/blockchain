
use blockchain::{core::{BlockchainHash, PublicKeyHash}, crypto::calculate_p2pkh_hash};
use k256::ecdsa::{Signature, SigningKey, VerifyingKey, signature::Signer};
use rand::rngs::OsRng;

pub struct Wallet {
    private_key: SigningKey,
    public_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key().clone();

        Wallet {
            private_key: signing_key,
            public_key: verifying_key,
        }
    }

    pub fn get_public_key_bytes(&self) -> Vec<u8> {
        self.public_key.to_encoded_point(true).as_bytes().to_vec()
    }

    pub fn get_public_key_hash(&self) -> PublicKeyHash {
        let public_key_bytes = self.get_public_key_bytes();
        calculate_p2pkh_hash(&public_key_bytes)
    }

    pub fn sign_message(&self, message_hash: &BlockchainHash) -> Result<Vec<u8>, String> {
        let signature: Signature = self.private_key.sign(message_hash.as_ref());
        Ok(signature.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_key_generation() {
        let wallet = Wallet::new();
        let verifying_key_bytes = wallet.get_public_key_bytes();
        let hex_key = hex::encode(verifying_key_bytes);
        println!("{hex_key}");
    }
}
