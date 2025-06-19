use std::{collections::HashMap, sync::Arc};

use futures::future::try_join_all;
use k256::ecdsa::{Signature, VerifyingKey, signature::Verifier};
use tokio::sync::RwLock;

use crate::{
    block::Block,
    blockchain::utxo_set::UTXOSet,
    core::BlockchainHash,
    crypto::{calculate_p2pkh_hash, parse_p2pkh_script_sig_k256},
    data::storage::{self, Storage, StorageError},
    transaction::{Transaction, TxOut},
};

mod utxo_set;

#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Inconsistent storage")]
    InconsistentStorage,
    #[error("Vusiness error: {0}")]
    BusinessError(String),
    #[error("Storage error: {0}")]
    StorageError(storage::StorageError),
    #[error("Invalid coinbase transaction: {0}")]
    InvalidCoinbase(String),
    #[error("UTXO not found: {tx_id}:{out_idx}")]
    UtxoNotFound { tx_id: BlockchainHash, out_idx: u32 },
    #[error("Failed to parse script signature: {0}")]
    ScriptSigParseError(String),
    #[error("Invalid script execution: {0}")]
    InvalidScript(String),
    #[error("Invalid signature format: {0}")]
    InvalidSignatureFormat(String),
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("Insufficient funds in transaction inputs")]
    InsufficientFunds,
    #[error("Invalid public key format: {0}")]
    InvalidPublicKeyFormat(String),
    #[error("Double spend attempt for UTXO: {tx_id}:{out_idx}")]
    DoubleSpendAttempt { tx_id: BlockchainHash, out_idx: u32 },
    #[error("Invalid transaction fee: {0}")]
    InvalidFee(String),
    #[error("Mempool error {0}")]
    MempoolError(String),
    #[error("Invalid block error: {0}")]
    InvalidBlock(String),
    #[error("Invalid proof of work: {0}")]
    InvalidProofOfWork(String),
}

impl From<storage::StorageError> for BlockchainError {
    fn from(err: storage::StorageError) -> Self {
        Self::StorageError(err)
    }
}

pub struct Blockchain<S: Storage> {
    current_tip_hash: BlockchainHash,
    current_tip_block: Block,
    mempool: HashMap<BlockchainHash, Transaction>,
    utxo_set: Arc<RwLock<UTXOSet<TxOut>>>,
    storage: S,
}

impl<S: Storage> Blockchain<S> {
    pub fn new(storage: S) -> Blockchain<S> {
        Blockchain {
            mempool: HashMap::new(),
            utxo_set: Arc::new(RwLock::new(UTXOSet::new())),
            storage,
            current_tip_hash: BlockchainHash::default(),
            current_tip_block: Block::genesis(),
        }
    }

    pub async fn init(mut self) -> Result<Self, BlockchainError> {
        match self.storage.get_latest_block().await {
            Ok(block) => {
                self.current_tip_hash = block.hash;
                self.current_tip_block = block;
                Ok(())
            }
            Err(StorageError::BlockNotFound) => {
                let genesis = Block::genesis();
                let block = self.storage.save_block(genesis).await?;
                self.storage
                    .set_latest_block_hash(block.hash.as_ref().clone())
                    .await?;

                // 6. Update in memory state
                self.current_tip_hash = block.hash;
                self.current_tip_block = block;
                Ok(())
            }
            Err(err) => Err(BlockchainError::StorageError(err)),
        }?;

        self.rebuild_utxo_set().await?;

        Ok(self)
    }

    pub fn validate_chain(&self) -> bool {
        // let blocks = &self.blocks;

        // if self.blocks.is_empty() {
        //     return false;
        // }

        // if blocks[0] != Block::genesis() {
        //     return false;
        // }

        // for i in 1..blocks.len() {
        //     let current = &blocks[i];
        //     let previous = &blocks[i - 1];

        //     // Check that current block's previous_hash matches previous block's hash
        //     if current.previous_hash != previous.hash {
        //         return false;
        //     }

        //     // Recalculate the hash of the current block
        //     let recalculated_hash = Block::calculate_hash(
        //         current.index,
        //         current.timestamp,
        //         &current.data,
        //         &current.previous_hash,
        //         current.nonce,
        //     );

        //     // Check if the recalculated hash matches the stored hash
        //     if current.hash != recalculated_hash {
        //         return false;
        //     }
        // }

        true
    }

    pub async fn add_transaction(&mut self, tx: Transaction) -> Result<Transaction, BlockchainError> {
        if self.mempool.contains_key(&tx.id) {
            return Err(BlockchainError::MempoolError(
                "Transaction already exists".to_string(),
            ));
        }

        self.validate_transaction(&tx).await?;
        self.mempool.insert(tx.id.clone(), tx.clone());

        Ok(tx)
    }

    async fn validate_and_sum_tx_fees(&self, block: &Block) -> Result<u64, BlockchainError> {
        let validation_futures: Vec<_> = block
            .transactions
            .iter()
            .skip(1)
            .map(|tx| {
                // might need to clone `self` if it's an Arc<Mutex<Blockchain>>
                // or ensure `self` is validly captured across tasks.
                // For example: let blockchain_clone = Arc::clone(&self.blockchain_arc);
                // async move { blockchain_clone.validate_transaction(tx).await }
                self.validate_transaction(tx) // Map each transaction to its validation Future
            })
            .collect();

        let all_fees: Vec<u64> = try_join_all(validation_futures).await?;

        let fees = all_fees.iter().sum::<u64>();

        Ok(fees)
    }

    fn validate_coinbase_transaction(
        &self,
        tx: &Transaction,
        total_fees_in_block: u64,
    ) -> Result<(), BlockchainError> {
        // Coinbase should typically have one "null" input
        if !tx.inputs.is_empty() {
            if tx.inputs.len() != 1
                || tx.inputs[0].prev_tx_id != BlockchainHash::default()
                || tx.inputs[0].prev_out_idx != u32::MAX
            {
                return Err(BlockchainError::InvalidCoinbase(
                    "Coinbase transaction has invalid inputs".to_string(),
                ));
            }
        }

        let block_reward = 50_000_000_000u64;

        let total_output_value: u64 = tx.outputs.iter().map(|o| o.value).sum();

        if total_output_value > block_reward + total_fees_in_block {
            return Err(BlockchainError::InvalidCoinbase(format!(
                "Coinbase output value exceeds allowed limit: {} > {}",
                total_output_value,
                block_reward + total_fees_in_block
            )));
        }

        return Ok(());
    }

    async fn validate_transaction(&self, tx: &Transaction) -> Result<u64, BlockchainError> {
        let mut total_input_value: u64 = 0;

        let utxo_set = self.utxo_set.read().await;

        // Verify inputs
        for tx_in in &tx.inputs {
            let utxo_key = (tx_in.prev_tx_id.clone(), tx_in.prev_out_idx);
            let prev_utxo =
                utxo_set
                    .get(&utxo_key)
                    .ok_or_else(|| BlockchainError::UtxoNotFound {
                        tx_id: tx_in.prev_tx_id.clone(),
                        out_idx: tx_in.prev_out_idx,
                    })?;

            // Ensure the UTXO is not spent twice within this *same* transaction (though UTXO set handles across txs)
            // This requires a check on the `tx.inputs` themselves for duplicates.
            // For now, rely on `HashMap` lookup and removal during block processing.

            // Script validation (P2PKH focus)
            match &prev_utxo.script_pubkey {
                crate::core::Script::PayToPublicKeyHash { pub_key_hash } => {
                    let (signature_bytes, public_key_bytes) =
                        parse_p2pkh_script_sig_k256(&tx_in.script_sig).map_err(|e| {
                            BlockchainError::ScriptSigParseError(format!(
                                "Failed to parse P2PKH script_sig: {}",
                                e
                            ))
                        })?;

                    // a. Verify public key hash matches
                    let derived_pub_key_hash = calculate_p2pkh_hash(&public_key_bytes);
                    if &derived_pub_key_hash != pub_key_hash {
                        return Err(BlockchainError::InvalidScript(
                            "Public key hash mismatch in P2PKH script".to_string(),
                        ));
                    }

                    // b. Verify signature (USING k256)
                    let verifying_key =
                        VerifyingKey::from_sec1_bytes(&public_key_bytes).map_err(|e| {
                            BlockchainError::InvalidPublicKeyFormat(format!(
                                "Invalid public key format: {}",
                                e
                            ))
                        })?;
                    let signature = Signature::try_from(signature_bytes.as_ref()).map_err(|e| {
                        BlockchainError::InvalidSignatureFormat(format!(
                            "Invalid signature format: {}",
                            e
                        ))
                    })?;

                    // The message signed is typically the transaction's hash (ID).
                    // `tx.id.as_ref()` should provide the 32-byte message digest that was signed

                    if verifying_key.verify(tx.id.as_ref(), &signature).is_err() {
                        return Err(BlockchainError::InvalidScript(
                            "Invalid signature for transaction input".to_string(),
                        ));
                    }
                } // _ => return Err(BlockchainError::InvalidScript("Unsupported script type".to_string())),
            }

            total_input_value += prev_utxo.value;
        }

        // Verify outputs
        let total_output_value: u64 = tx.outputs.iter().map(|o| o.value).sum();
        for tx_out in &tx.outputs {
            if tx_out.value == 0 {
                return Err(BlockchainError::InvalidTransaction(
                    "Transaction output value cannot be zero".to_string(),
                ));
            }
        }

        // Verify total input value >= total output value (fees)
        if total_input_value < total_output_value {
            return Err(BlockchainError::InsufficientFunds);
        }

        let fee = total_input_value - total_output_value;
        Ok(fee)
    }

    pub async fn mine_pending_transactions(&mut self) -> Result<(), BlockchainError> {
        let mut transactions: Vec<Transaction> = self.mempool.drain().map(|(_, tx)| tx).collect();

        transactions.insert(0, Transaction::coinbase_transaction());
        let last_block = self.last_block();

        let block = Block::new(last_block.height + 1, transactions, last_block.hash);

        self.add_block(block).await?;

        Ok(())
    }

    pub async fn add_block(&mut self, block: Block) -> Result<(), BlockchainError> {
        let last_block = self.last_block();

        // 1. block continuity checks
        if block.height != last_block.height + 1 {
            return Err(BlockchainError::InvalidBlock(format!(
                "Block height is incorrect"
            )));
        }

        if block.prev_block_hash != last_block.hash {
            return Err(BlockchainError::InvalidBlock(format!(
                "Block hash do not match previous hash"
            )));
        }

        // 2. Full header validation
        block.verify_merkle_root()?;
        block.verify_timestamp_plausibility()?;
        block.validate_proof_of_work()?;

        // 3. Transactions validation
        let total_fee = self.validate_and_sum_tx_fees(&block).await?;
        let coinbase_tx = block
            .transactions
            .first()
            .ok_or(BlockchainError::InvalidCoinbase(format!(
                "Coinbase transaction is missing."
            )))?;
        self.validate_coinbase_transaction(coinbase_tx, total_fee)?;

        // 4. UTXO set update
        let (utxo_add, utxo_remove) = block.get_utxos();
        let mut utxo_set = self.utxo_set.write().await;

        for utxo in utxo_remove {
            utxo_set.remove(&utxo);
        }

        for (key, value) in utxo_add {
            utxo_set.insert(key, value);
        }

        // 5. Persistance
        let block = self.storage.save_block(block).await?;
        // may be need to save height too?
        self.storage
            .set_latest_block_hash(block.hash.as_ref().clone())
            .await?;

        // 6. Update in memory state
        self.current_tip_hash = block.hash;
        self.current_tip_block = block;

        Ok(())
    }

    pub async fn get_blocks(&self) -> Result<Vec<Block>, BlockchainError> {
        let mut receiver = self.storage.stream_blocks_by_height().await?;

        let mut all_blocks: Vec<Block> = Vec::new();

        while let Some(block_result) = receiver.recv().await {
            let block = block_result?;
            all_blocks.push(block);
        }

        Ok(all_blocks)
    }

    pub async fn rebuild_utxo_set(&mut self) -> Result<(), BlockchainError> {
        let mut block_receiver = self.storage.stream_blocks_by_height().await?;
        let mut utxo_set = self.utxo_set.write().await;

        utxo_set.clear();

        while let Some(block_res) = block_receiver.recv().await {
            let block = block_res?;

            for tx in &block.transactions {
                if !tx.is_coinbase() {
                    for tx_in in &tx.inputs {
                        utxo_set.remove(&(tx_in.prev_tx_id.clone(), tx_in.prev_out_idx));
                    }
                }

                // Add new UTXOs
                for (idx, tx_out) in tx.outputs.iter().enumerate() {
                    utxo_set.insert((tx.id.clone(), idx as u32), tx_out.clone());
                }
            }
        }

        println!("UTXO set rebuilt successfully via streaming.");
        Ok(())
    }

    fn last_block(&self) -> &Block {
        &self.current_tip_block
    }
}
