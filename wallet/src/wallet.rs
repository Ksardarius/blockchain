use ed25519_dalek::{PUBLIC_KEY_LENGTH, Signer, SigningKey, ed25519::SignatureBytes};
use rand::rngs::OsRng;

pub struct Wallet {
    signing_key: SigningKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Wallet { signing_key }
    }

    pub fn verify(&self, message: &[u8], signature_bytes: &[u8]) -> bool {
        if signature_bytes.len() != 64 {
            return false;
        }

        Signature::from_slice(signature_bytes)
            .and_then(|sig| self.signing_key.verify(message, &sig))
            .is_ok()
    }

    pub fn public_key_bytes(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.signing_key.verifying_key().to_bytes()
    }

    pub fn sign(&self, message: &[u8]) -> SignatureBytes {
        self.signing_key.sign(message).to_bytes()
    }
}

pub use ed25519_dalek::Signature;

#[cfg(test)]
mod tests {
    use ed25519_dalek::{SignatureError, VerifyingKey};

    use super::*;

    #[test]
    fn test_public_key_generation() -> Result<(), SignatureError> {
        let wallet = Wallet::new();
        let verifying_key_bytes = wallet.public_key_bytes();
        VerifyingKey::from_bytes(&verifying_key_bytes)?;
        let hex_key = hex::encode(verifying_key_bytes);
        println!("{hex_key}");

        Ok(())
    }

    #[test]
    fn test_can_sign_and_verify() -> Result<(), SignatureError> {
        let message = b"My test";

        let wallet = Wallet::new();
        let signer_text = wallet.sign(message);

        let verifying_key_bytes = wallet.public_key_bytes();
        let public_key = VerifyingKey::from_bytes(&verifying_key_bytes)?;

        public_key.verify_strict(message, &signer_text.into())?;

        Ok(())
    }

    #[test]
    fn test_catch_changed_data() -> Result<(), SignatureError> {
        let message = b"My test";

        let wallet = Wallet::new();
        let signer_text = wallet.sign(message);

        let verifying_key_bytes = wallet.public_key_bytes();
        let public_key = VerifyingKey::from_bytes(&verifying_key_bytes)?;

        let result = public_key.verify_strict(b"Changed text", &signer_text.into());
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_verify() -> Result<(), SignatureError> {
        let message = b"My test";

        let wallet = Wallet::new();
        let signer_text = wallet.sign(message);

        assert!(wallet.verify(message, &signer_text));

        Ok(())
    }
}
