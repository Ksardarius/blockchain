use std::collections::HashMap;

use crate::core::BlockchainHash;

type Key = (BlockchainHash, u32);

#[derive(Debug, Clone)]
pub struct UTXOSet<TxOut> {
    pub data: HashMap<Key, TxOut>,
}

impl<TxOut> UTXOSet<TxOut> {
    pub fn new() -> Self {
        UTXOSet {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &Key) -> Option<&TxOut> {
        self.data.get(key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, key: &Key) -> Option<TxOut> {
        self.data.remove(key)
    }

    pub fn insert(&mut self, key: Key, value: TxOut) -> Option<TxOut> {
        self.data.insert(key, value)
    }
}
