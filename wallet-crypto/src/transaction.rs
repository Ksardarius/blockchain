use bincode::{Encode, config};
use chrono::Utc;
use core;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    keys::{BlockchainHash, PublicKeyHash, PublicKeyWithSignature, Signature, SignatureError},
    scripts::Script,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode)]
pub struct TxOut {
    pub value: u64,
    pub script_pubkey: Script,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode)]
pub struct TxIn {
    pub prev_tx_id: BlockchainHash,
    pub prev_out_idx: u32,
    pub script_sig: Signature,
    pub sequence: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode)]
pub struct Transaction {
    pub id: BlockchainHash,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub timestamp: u128,
}

impl Transaction {
    pub fn calculate_id(&self) -> BlockchainHash {
        #[derive(Serialize, Encode)]
        struct TxForHashing<'a> {
            inputs: &'a Vec<TxIn>,
            outputs: &'a Vec<TxOut>,
            timestamp: u128,
        }

        let temp_tx = TxForHashing {
            inputs: &self.inputs,
            outputs: &self.outputs,
            timestamp: self.timestamp,
        };

        let encoded_bytes = bincode::encode_to_vec(&temp_tx, config::standard())
            .expect("Failed to serialize transaction for hashing. This should not happen.");

        let first_hash = Sha256::digest(&encoded_bytes); // Hash the bytes
        let second_hash = Sha256::digest(first_hash);

        BlockchainHash::new(second_hash.into())
    }

    pub fn new(inputs: Vec<TxIn>, outputs: Vec<TxOut>) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;
        let mut transaction = Transaction {
            id: BlockchainHash::default(),
            inputs,
            outputs,
            timestamp,
        };

        transaction.id = transaction.calculate_id();
        transaction
    }

    pub fn coinbase_transaction() -> Transaction {
        const INITIAL_BLOCK_REWARD: u64 = 50;

        let initial_reward_output = TxOut {
            value: INITIAL_BLOCK_REWARD, // Define this constant
            script_pubkey: Script::PayToPublicKeyHash {
                pub_key_hash: PublicKeyHash::new([0u8; 20]),
            },
        };

        Transaction {
            id: BlockchainHash::default(),
            inputs: vec![],
            outputs: vec![initial_reward_output],
            timestamp: 0,
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }

    pub fn verify_signatures(&self) -> Result<(), SignatureError> {
        let message = self.calculate_id();

        for tx_in in &self.inputs {
            let verifier: PublicKeyWithSignature = (&tx_in.script_sig).try_into()?;
            verifier.verify(message.as_ref())?;
        }

        Ok(())
    }
}
