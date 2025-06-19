use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use bincode::{Encode, config};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    blockchain::BlockchainError,
    core::BlockchainHash,
    transaction::{Transaction, TxOut},
};

// How far into the future a block timestamp is allowed to be (e.g., 2 hours for Bitcoin-like behavior)
const TIMESTAMP_FUTURITY_TOLERANCE_SECS: u64 = 2 * 60 * 60; // 2 hours
const TIMESTAMP_FUTURITY_TOLERANCE_MILLIS: u128 =
    (TIMESTAMP_FUTURITY_TOLERANCE_SECS as u128) * 1000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub height: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub prev_block_hash: BlockchainHash,
    pub merkle_root: BlockchainHash,
    pub bits: u32,
    pub nonce: u64,
    pub hash: BlockchainHash,
}

impl Block {
    pub fn new(
        height: u64,
        transactions: Vec<Transaction>,
        prev_block_hash: BlockchainHash,
    ) -> Block {
        let merkle_root = Block::calculate_merkle_root(&transactions).expect(
            "Genesis block Merkle root calculation should not fail with a coinbase transaction.",
        );

        let timestamp = Utc::now().timestamp_millis() as u128;
        let nonce = 0;
        let hash = BlockchainHash::default();
        let bits = 2;

        let mut block = Block {
            height,
            timestamp,
            transactions,
            merkle_root,
            prev_block_hash,
            hash,
            nonce,
            bits,
        };

        block.hash = block.calculate_hash();

        block
    }

    pub fn genesis() -> Block {
        let coinbase_transaction = Transaction::coinbase_transaction();
        let transactions = vec![coinbase_transaction];

        let merkle_root = Block::calculate_merkle_root(&transactions).expect(
            "Genesis block Merkle root calculation should not fail with a coinbase transaction.",
        );

        let height = 0;
        let timestamp = 1231006505;

        let prev_block_hash: BlockchainHash = BlockchainHash::default();
        let nonce = 0;
        let hash = BlockchainHash::default();
        let bits = 2;

        let mut block = Block {
            height,
            timestamp,
            transactions,
            merkle_root,
            prev_block_hash,
            hash,
            nonce,
            bits,
        };

        block.hash = block.calculate_hash();

        block
    }

    pub fn calculate_hash(&self) -> BlockchainHash {
        #[derive(Serialize, Encode)]
        struct BlockHeaderForHashing {
            height: u64,
            timestamp: u128,
            prev_block_hash: BlockchainHash,
            merkle_root: BlockchainHash,
            bits: u32,
            nonce: u64,
        }

        let temp_block = BlockHeaderForHashing {
            height: self.height,
            timestamp: self.timestamp,
            prev_block_hash: self.prev_block_hash,
            merkle_root: self.merkle_root,
            bits: self.bits,
            nonce: self.nonce,
        };

        let encoded_bytes = bincode::encode_to_vec(&temp_block, config::standard())
            .expect("Failed to serialize block for hashing. This should not happen.");

        let first_hash = Sha256::digest(&encoded_bytes); // Hash the bytes
        let second_hash = Sha256::digest(first_hash);

        BlockchainHash::new(second_hash.into())
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> Result<BlockchainHash, String> {
        if transactions.is_empty() {
            return Err("Block must contain at least one transaction (coinbase)".to_string());
        }

        let mut leaves: Vec<[u8; 32]> = transactions
            .iter()
            .map(|tx| *tx.calculate_id().as_ref()) // assuming tx.calculate_id() returns Hash([u8; 32])
            .collect();

        while leaves.len() > 1 {
            if leaves.len() % 2 != 0 {
                let last_hash = *leaves.last().unwrap(); // Get a copy of the last hash
                leaves.push(last_hash); // Add the duplicate to the end
            }

            let mut next_level_hashes = Vec::new();

            for i in (0..leaves.len()).step_by(2) {
                let left_hash = leaves[i];
                let right_hash = leaves[i + 1];

                let mut hasher = Sha256::new();
                hasher.update(left_hash);
                hasher.update(right_hash);
                let combined_hash_bytes: [u8; 32] = hasher.finalize().into();
                next_level_hashes.push(combined_hash_bytes);
            }

            leaves = next_level_hashes;
        }

        Ok(BlockchainHash::new(leaves[0]))
    }

    pub fn verify_merkle_root(&self) -> Result<(), BlockchainError> {
        let calculated_merkle_root =
            Block::calculate_merkle_root(&self.transactions).map_err(|e| {
                BlockchainError::InvalidBlock(format!("Merkle root calculation failed: {}", e))
            })?;

        if calculated_merkle_root != self.merkle_root {
            return Err(BlockchainError::InvalidBlock(
                "Merkle root mismatch".to_string(),
            ));
        }
        Ok(())
    }

    pub fn verify_timestamp_plausibility(&self) -> Result<(), BlockchainError> {
        let current_time_millis = Utc::now().timestamp_millis() as u128;

        // Rule: Block timestamp must not be more than `TIMESTAMP_FUTURITY_TOLERANCE_MILLIS`
        // in the future compared to the validating node's current time.
        if self.timestamp > current_time_millis + TIMESTAMP_FUTURITY_TOLERANCE_MILLIS {
            return Err(BlockchainError::InvalidBlock(format!(
                "Block timestamp ({}) is too far in the future (current: {}, tolerance: {}ms)",
                self.timestamp, current_time_millis, TIMESTAMP_FUTURITY_TOLERANCE_MILLIS
            )));
        }

        Ok(())
    }

    pub fn validate_proof_of_work(&self) -> Result<(), BlockchainError> {
        let calculated_header_hash = self.calculate_hash();
        let difficulty_target = Self::get_difficulty_target_from_bits(self.bits)?;

        if calculated_header_hash.is_zero_hash() || calculated_header_hash > difficulty_target {
            return Err(BlockchainError::InvalidProofOfWork(format!(
                "Block hash ({}) does not meet difficulty target ({})",
                calculated_header_hash, difficulty_target
            )));
        }

        if calculated_header_hash != self.hash {
            return Err(BlockchainError::InvalidBlock(format!(
                "Calculated block hash ({}) does not match stored block.hash ({})",
                calculated_header_hash, self.hash
            )));
        }

        Ok(())
    }

    fn get_difficulty_target_from_bits(bits: u32) -> Result<BlockchainHash, BlockchainError> {
        // Placeholder for now:
        // Example: If bits represents the number of leading zeros required (simpler model)
        let mut target_bytes = [0xFF; 32];
        let num_leading_zeros = bits as usize;
        for i in 0..num_leading_zeros {
            target_bytes[i / 8] &= !(1 << (7 - (i % 8)));
        }
        Ok(BlockchainHash::new(target_bytes))
    }

    pub fn get_utxos<'a>(
        &'a self,
    ) -> (
        impl Iterator<Item = ((BlockchainHash, u32), TxOut)> + 'a,
        impl Iterator<Item = (BlockchainHash, u32)> + 'a,
    ) {
        let utxos_to_add_iter = self.transactions.iter().flat_map(|tx| {
            tx.outputs
                .iter()
                .enumerate()
                .map(move |(idx, tx_out)| ((tx.id, idx as u32), tx_out.clone()))
        });

        let outpoints_to_remove_iter = self
            .transactions
            .iter()
            .filter(|tx| !tx.is_coinbase()) // Only consider non-coinbase transactions
            .flat_map(|tx| {
                tx.inputs
                    .iter()
                    .map(|tx_in| (tx_in.prev_tx_id, tx_in.prev_out_idx))
            });

        (utxos_to_add_iter, outpoints_to_remove_iter)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Block #{}", self.height)?;
        writeln!(f, "Timestamp: {}", self.timestamp)?;
        writeln!(f, "Data: {:?}", self.transactions)?;
        writeln!(
            f,
            "Previous Hash: {}",
            hex::encode(self.prev_block_hash.as_ref())
        )?;
        writeln!(f, "Hash: {}", hex::encode(self.hash.as_ref()))?;
        writeln!(f, "Nonce: {}", self.nonce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_genesis_return_block() {
        let block = Block::genesis();

        assert_eq!(block.height, 0);
        assert_eq!(block.transactions.len(), 0);
        assert_eq!(block.prev_block_hash, BlockchainHash::default());
        assert_eq!(block.nonce, 0);
        assert_eq!(block.timestamp, 0);

        let hex_hash = hex::encode(block.hash.as_ref());
        assert_eq!(hex_hash.len(), 64);
    }
}
