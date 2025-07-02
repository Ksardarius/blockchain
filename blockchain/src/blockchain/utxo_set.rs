use std::collections::{HashMap, HashSet};

use wallet_crypto::{keys::{BlockchainHash, PublicKeyHash}, transaction::{TxOut, UTXO}};

pub trait TxOutRecipient {
    fn get_address(&self) -> PublicKeyHash;
    fn get_received_amount(&self) -> u64;
}

type Key = (BlockchainHash, u32);

#[derive(Debug, Clone)]
pub struct UTXOSet<TxOut> {
    pub data: HashMap<Key, TxOut>,
    pub reserved: HashSet<Key>
}

impl<TxOut> UTXOSet<TxOut> {
    pub fn new() -> Self {
        UTXOSet {
            data: HashMap::new(),
            reserved: HashSet::new()
        }
    }

    pub fn get(&self, key: &Key) -> Option<&TxOut> {
        self.data.get(key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, key: &Key) -> Option<TxOut> {
        self.reserved.remove(key);
        self.data.remove(key)
    }

    pub fn insert(&mut self, key: Key, value: TxOut) -> Option<TxOut> {
        self.data.insert(key, value)
    }
}

impl<TxOut: TxOutRecipient + Clone> UTXOSet<TxOut> {
    pub fn get_utxos_by_address(&self, address: PublicKeyHash) -> Vec<UTXO> {
        self.data.iter().fold(vec![], |mut acc, ((tx, idx), val)| {
            let recipient = val.get_address();
            if recipient == address {
                acc.push(UTXO {
                    prev_tx_id: tx.clone(),
                    prev_out_idx: *idx,
                    value: val.get_received_amount()
                });
            }

            acc
        })
    }
}

impl TxOutRecipient for TxOut {
    fn get_address(&self) -> PublicKeyHash {
        match self.script_pubkey {
            wallet_crypto::scripts::Script::PayToPublicKeyHash { pub_key_hash } => pub_key_hash,
        }
    }
    
    fn get_received_amount(&self) -> u64 {
        self.value
    }
}
