use std::fmt;

use chrono::Utc;
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

use crate::transaction::Transaction;

pub type Hash = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: Vec<Transaction>,
    pub previous_hash: Hash,
    pub hash: Hash,
    pub nonce: u64
}

impl Block {
    pub fn new(index: u64, data: Vec<Transaction>, previous_hash: Hash) -> Block {
        let timestamp = Utc::now().timestamp_millis() as u128;
        let nonce = 0;
        let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash, nonce);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce
        }
    }

    pub fn mine_block(index: u64, data: Vec<Transaction>, previous_hash: Hash, difficulty: usize) -> Block {
        let mut nonce = 0;
        let timestamp = Utc::now().timestamp_millis() as u128;
        loop {
            let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash, nonce);
            if Self::hash_starts_with_zeros(&hash, difficulty) {
                return Block {
                    index,
                    timestamp,
                    data,
                    previous_hash,
                    hash,
                    nonce
                }
            }

            nonce += 1;
        }
    }

    pub fn genesis() -> Block {
        let index = 0;
        let timestamp = 0;
        let data = vec![];
        let previous_hash = [0u8; 32];
        let nonce = 0;
        let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash, nonce);

        Block {
            index,
            timestamp,
            data: data.to_owned(),
            previous_hash,
            hash,
            nonce
        }
    }

    pub fn calculate_hash(index: u64, timestamp: u128, data: &[Transaction], previous_hash: &Hash, nonce: u64) -> Hash {
        let serialized_data = serde_json::to_vec(data).expect("Failed to serialize transactions");
        
        let mut hasher = Sha256::new();

        hasher.update(index.to_le_bytes());
        hasher.update(timestamp.to_le_bytes());
        hasher.update(&serialized_data);
        hasher.update(previous_hash);
        hasher.update(nonce.to_le_bytes());

        hasher.finalize().into()
    }

    fn hash_starts_with_zeros(hash: &Hash, difficulty: usize) -> bool {
        let zeros = difficulty / 2; // 2 hex chars = 1 byte
        hash.iter().take(zeros).all(|&b| b == 0)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Block #{}", self.index)?;
        writeln!(f, "Timestamp: {}", self.timestamp)?;
        writeln!(f, "Data: {:?}", self.data)?;
        writeln!(f, "Previous Hash: {}", hex::encode(self.previous_hash))?;
        writeln!(f, "Hash: {}", hex::encode(self.hash))?;
        writeln!(f, "Nonce: {}", self.nonce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_create_block_fields() {
        let index = 5;
        let data = create_sample_transaction();

        let previous_hash = [0u8; 32]; // Genesis previous hash: all zeros

        let block = Block::new(index, vec![data.clone()], previous_hash);

        assert_eq!(block.index, index);
        assert_eq!(block.data[0], data);
        assert_eq!(block.previous_hash, previous_hash);
        assert_eq!(block.nonce, 0);

        // timestamp should be greater than 0
        assert!(block.timestamp > 0);
    }

    #[test]
    fn test_hash_is_deterministic() {
        let index = 1;
        let previous_hash = [1u8; 32];
        let data = create_sample_transaction();


        // Let's test the hash function directly for same inputs
        let timestamp = 1234567890u128;
        let nonce = 0;

        let hash1 = Block::calculate_hash(index, timestamp, &vec![data.clone()], &previous_hash, nonce);
        let hash2 = Block::calculate_hash(index, timestamp, &vec![data.clone()], &previous_hash, nonce);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_changes_with_nonce() {
        let index = 1;
        let timestamp = 1234567890u128;
        let previous_hash = [2u8; 32];
        let data = create_sample_transaction();

        let nonce1 = 0;
        let nonce2 = 1;

        let hash1 = Block::calculate_hash(index, timestamp, &vec![data.clone()], &previous_hash, nonce1);
        let hash2 = Block::calculate_hash(index, timestamp, &vec![data.clone()], &previous_hash, nonce2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_length() {
        let index = 0;
        let data = create_sample_transaction();
        let previous_hash = [0u8; 32];

        let block = Block::new(index, vec![data.clone()], previous_hash);

        assert_eq!(block.hash.len(), 32);

        // Also test hex encoding length (64 chars for SHA-256)
        let hex_hash = hex::encode(block.hash);
        assert_eq!(hex_hash.len(), 64);
    }

    #[test]
    fn test_genesis_return_block() {
        let block = Block::genesis();

        assert_eq!(block.index, 0);
        assert_eq!(block.data.len(), 0);
        assert_eq!(block.previous_hash, [0u8; 32]);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.timestamp, 0);

        let hex_hash = hex::encode(block.hash);
        assert_eq!(hex_hash.len(), 64);
    }

    #[test]
    fn test_mine_block_return_only_valid_hash() {
        let genesis = Block::genesis();
        let data = create_sample_transaction();
        let block = Block::mine_block(genesis.index + 1, vec![data.clone()], genesis.hash, 2);
        assert_eq!(true, block.hash.iter().take(1).all(|&b| b == 0));
    }

    fn create_sample_transaction() -> Transaction {
        Transaction::new("sender", "recipient", 10, None)
    }
}
