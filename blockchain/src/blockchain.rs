use wallet::Wallet;

use crate::{block::Block, data::storage::{self, Storage}, transaction::Transaction};

#[derive(Debug)]
pub enum BlockchainError {
    BusinessError(String),
    StorageError(storage::StorageError),
}

impl From<storage::StorageError> for BlockchainError {
    fn from(err: storage::StorageError) -> Self {
        Self::StorageError(err)
    }
}

pub struct Blockchain<S: Storage> {
    blocks: Vec<Block>,
    mempool: Vec<Transaction>,
    difficulty: usize,
    wallet: Wallet,
    storage: S,
}

impl<S: Storage> Blockchain<S> {
    pub fn new(storage: S) -> Blockchain<S> {
        Blockchain {
            blocks: vec![Block::genesis()],
            mempool: vec![],
            difficulty: 2,
            wallet: Wallet::new(),
            storage,
        }
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn get_wallet(&self) -> &Wallet {
        &self.wallet
    }

    pub fn print_blocks(&self) {
        for block in &self.blocks {
            println!("{:#?}", block);
        }
    }

    pub fn validate_chain(&self) -> bool {
        let blocks = &self.blocks;

        if self.blocks.is_empty() {
            return false;
        }

        if blocks[0] != Block::genesis() {
            return false;
        }

        for i in 1..blocks.len() {
            let current = &blocks[i];
            let previous = &blocks[i - 1];

            // Check that current block's previous_hash matches previous block's hash
            if current.previous_hash != previous.hash {
                return false;
            }

            // Recalculate the hash of the current block
            let recalculated_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.data,
                &current.previous_hash,
                current.nonce,
            );

            // Check if the recalculated hash matches the stored hash
            if current.hash != recalculated_hash {
                return false;
            }
        }

        true
    }

    pub fn create_transaction(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<&Transaction, BlockchainError> {
        if sender != "network" {
            let sender_balance = self.get_balance(sender);
            if sender_balance < amount {
                return Err(BlockchainError::BusinessError(format!(
                    "Insufficient funds: {} has {}, needs {}",
                    sender, sender_balance, amount
                )));
            }
        }

        let tx = Transaction::new(sender, recipient, amount, Some(&self.wallet));
        self.mempool.push(tx);

        Ok(self.mempool.last().unwrap())
    }

    pub fn mine_pending_transactions(&mut self, miner_address: &str) -> Result<(), BlockchainError> {
        let reward_tx = Transaction::new("network", miner_address, 10, Some(&self.wallet));
        self.mempool.push(reward_tx);

        let transactions = self
            .mempool
            .drain(..)
            .filter(|transaction| transaction.verify(&self.wallet))
            .collect();

        self.add_block(transactions)?;

        Ok(())
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        let balance: i128 = self
            .blocks
            .iter()
            .flat_map(|block| &block.data)
            .map(|t| {
                if t.recipient == address {
                    t.amount as i128
                } else if t.sender == address {
                    -(t.amount as i128)
                } else {
                    0
                }
            })
            .sum();

        balance.max(0) as u64
    }

    fn add_block(&mut self, data: Vec<Transaction>) -> Result<(), BlockchainError> {
        let last_block = self.last_block();
        let new_block =
            Block::mine_block(last_block.index + 1, data, last_block.hash, self.difficulty);

        self.storage.save_block(&new_block)?;
        self.blocks.push(new_block);

        Ok(())
    }

    fn last_block(&self) -> &Block {
        self.blocks.last().expect("Blockchain is empty")
    }
}

#[cfg(test)]
mod tests {
    use crate::data::storage::mock_storage::MockStorage;

    use super::*;

    #[test]
    fn test_valid_chain() -> Result<(), BlockchainError> {
        let mut blockchain = Blockchain::new(MockStorage::new()); // contains genesis block
        blockchain.add_block(vec![create_sample_transaction("s1", "r1", 1)])?;
        blockchain.add_block(vec![create_sample_transaction("s2", "r2", 1)])?;

        assert!(blockchain.validate_chain());

        Ok(())
    }

    #[test]
    fn test_tampered_data() -> Result<(), BlockchainError> {
        let mut blockchain = Blockchain::new(MockStorage::new());
        blockchain.add_block(vec![create_sample_transaction("s1", "r1", 1)])?;
        blockchain.add_block(vec![create_sample_transaction("s2", "r2", 1)])?;

        // Tamper with data in the second block
        blockchain.blocks[1].data = vec![create_sample_transaction("s3", "r3", 1)];

        // Validate should fail
        assert!(!blockchain.validate_chain());

        Ok(())
    }

    #[test]
    fn test_tampered_previous_hash() -> Result<(), BlockchainError> {
        let mut blockchain = Blockchain::new(MockStorage::new());
        blockchain.add_block(vec![create_sample_transaction("s1", "r1", 1)])?;
        blockchain.add_block(vec![create_sample_transaction("s2", "r2", 1)])?;

        // Tamper with previous_hash in third block
        blockchain.blocks[2].previous_hash = [0u8; 32];

        // Validate should fail
        assert!(!blockchain.validate_chain());

        Ok(())
    }

    #[test]
    fn test_get_balance() -> Result<(), BlockchainError> {
        let user_name = "user1";
        let mut blockchain = Blockchain::new(MockStorage::new());
        blockchain.create_transaction("network", user_name, 10)?;
        blockchain.create_transaction("network", user_name, 20)?;
        blockchain.mine_pending_transactions(user_name)?;

        assert_eq!(blockchain.get_balance(user_name), 40);

        Ok(())
    }

    #[test]
    fn test_only_commited_transactions() -> Result<(), BlockchainError> {
        let user_name = "user1";
        let mut blockchain = Blockchain::new(MockStorage::new());
        blockchain.create_transaction("network", user_name, 10)?;

        assert_eq!(blockchain.get_balance(user_name), 0);

        Ok(())
    }

    #[test]
    fn test_negative_balance() {
        let user_name = "user1";
        let mut blockchain = Blockchain::new(MockStorage::new());
        let result = blockchain.create_transaction("user2", user_name, 10);

        assert!(result.is_err());
    }

    fn create_sample_transaction(owner: &str, recipient: &str, amount: u64) -> Transaction {
        Transaction::new(owner, recipient, amount, None)
    }
}
