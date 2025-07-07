use bincode::{Encode, config};
use chrono::Utc;
use core;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    keys::{
        BlockchainHash, KeyPair, PublicKeyHash, PublicKeyWithSignature, Signature, SignatureError,
    },
    scripts::Script,
};

const GENESIS_ADDR: &'static str = "8dd45dc1a355c066d89e551db6cd9469513eb4dd";
const GENESIS_BLOCK_REWARD: u64 = 120;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode)]
pub struct UTXO {
    pub prev_tx_id: BlockchainHash,
    pub prev_out_idx: u32,
    pub value: u64
}

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
pub struct UnsignedTxIn {
    pub prev_tx_id: BlockchainHash,
    pub prev_out_idx: u32,
    pub sequence: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode)]
pub struct DraftTransaction {
    pub inputs: Vec<UnsignedTxIn>,
    pub outputs: Vec<TxOut>,
    pub timestamp: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode)]
pub struct Transaction {
    pub id: BlockchainHash,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub timestamp: u128,
}

impl DraftTransaction {
    pub fn new(inputs: Vec<UnsignedTxIn>, outputs: Vec<TxOut>) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;
        DraftTransaction {
            // id: BlockchainHash::default(),
            inputs,
            outputs,
            timestamp,
        }
    }

    pub fn sign(self, key: &KeyPair) -> Transaction {
        Transaction::new(self, key)
    }

    fn calculate_id(&self) -> BlockchainHash {
        let encoded_bytes = bincode::encode_to_vec(self, config::standard())
            .expect("Failed to serialize transaction for hashing. This should not happen.");

        let first_hash = Sha256::digest(&encoded_bytes); // Hash the bytes
        let second_hash = Sha256::digest(first_hash);

        BlockchainHash::new(second_hash.into())
    }
}

impl Transaction {
    fn new(draft: DraftTransaction, key: &KeyPair) -> Self {
        let draft_hash = draft.calculate_id();

        let signed_inputs: Vec<TxIn> = draft
            .inputs
            .into_iter()
            .map(|input| {
                let signature = key.sign(draft_hash.as_ref()).unwrap();
                TxIn {
                    prev_tx_id: input.prev_tx_id,
                    prev_out_idx: input.prev_out_idx,
                    sequence: input.sequence,
                    script_sig: Signature::build(signature, &key.public_key),
                }
            })
            .collect();

        let mut tx = Transaction {
            id: BlockchainHash::default(),
            inputs: signed_inputs,
            outputs: draft.outputs,
            timestamp: draft.timestamp,
        };

        tx.id = tx.calculate_id();

        tx
    }

    fn calculate_signing_id(&self) -> BlockchainHash {
        #[derive(Serialize, Encode)]
        struct TxForHashing<'a> {
            inputs: &'a Vec<UnsignedTxIn>,
            outputs: &'a Vec<TxOut>,
            timestamp: u128,
        }

        let temp_tx = TxForHashing {
            inputs: &self
                .inputs
                .iter()
                .map(|input| UnsignedTxIn {
                    prev_out_idx: input.prev_out_idx,
                    prev_tx_id: input.prev_tx_id,
                    sequence: input.sequence,
                })
                .collect(),
            outputs: &self.outputs,
            timestamp: self.timestamp,
        };

        let encoded_bytes = bincode::encode_to_vec(&temp_tx, config::standard())
            .expect("Failed to serialize transaction for hashing. This should not happen.");

        let first_hash = Sha256::digest(&encoded_bytes); // Hash the bytes
        let second_hash = Sha256::digest(first_hash);

        BlockchainHash::new(second_hash.into())
    }

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

    pub fn genesis_transaction() -> Transaction {
        Self::coinbase_transaction(GENESIS_ADDR, GENESIS_BLOCK_REWARD)
    }

    pub fn coinbase_transaction(miner_addr: &str, fee: u64) -> Transaction {
        
        let inital_wallet: PublicKeyHash =
            PublicKeyHash::try_from_string(miner_addr).unwrap();

        let initial_reward_output = TxOut {
            value: fee,
            script_pubkey: Script::PayToPublicKeyHash {
                pub_key_hash: inital_wallet,
            },
        };

        let mut tx = Transaction {
            id: BlockchainHash::default(),
            inputs: vec![TxIn {
                prev_tx_id: BlockchainHash::default(),
                prev_out_idx: 0xFFFFFFFF,
                sequence: 0xFFFFFFFF,
                script_sig: Signature::from_bytes(b"My custom blockchain miner message! Block X")
            }],
            outputs: vec![initial_reward_output],
            timestamp: 0,
        };

        tx.id = tx.calculate_id();

        tx
    }

    pub fn verify_signatures(&self) -> Result<(), SignatureError> {
        let message = self.calculate_signing_id();

        for tx_in in &self.inputs {
            let verifier: PublicKeyWithSignature = (&tx_in.script_sig).get_verifier()?;
            verifier.verify(message.as_ref())?;
        }

        Ok(())
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }
}
