use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::core::PublicKeyHash;

// Parses the scriptSig for k256 P2PKH: [DER_Signature] [Compressed_PublicKey]
// The sizes can vary, especially for DER signatures.
pub fn parse_p2pkh_script_sig_k256(script_sig_bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
    // This parsing is complex in real Bitcoin, using variable-length pushes.
    // Example: Assume signature is first, and public key is the last 33 bytes.
    if script_sig_bytes.len() < 33 {
        // Min possible length if signature is short + pubkey
        return Err("ScriptSig too short for P2PKH (k256)".to_string());
    }
    let pubkey_len = 33; // Compressed public key length for secp256k1
    if script_sig_bytes.len() < pubkey_len {
        return Err("ScriptSig too short for P2PKH pubkey".to_string());
    }
    let public_key = script_sig_bytes[script_sig_bytes.len() - pubkey_len..].to_vec();
    let signature = script_sig_bytes[0..script_sig_bytes.len() - pubkey_len].to_vec();

    Ok((signature, public_key))
}

/// Calculates the Bitcoin-style P2PKH hash (RIPEMD160(SHA256(PublicKey))).
/// Takes the raw public key bytes (e.g., 33 bytes for compressed k256).
/// Returns the 20-byte PublicKeyHash type.
pub fn calculate_p2pkh_hash(public_key_bytes: &[u8]) -> PublicKeyHash {
    let sha256_hash_bytes = Sha256::digest(public_key_bytes);
    let ripemd160_hash_bytes: [u8; 20] = Ripemd160::digest(sha256_hash_bytes).into();

    PublicKeyHash::new(ripemd160_hash_bytes)
}
