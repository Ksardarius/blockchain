use bincode::{
    self,
    config::standard,
    error::{DecodeError, EncodeError},
};
use sled::{Db, Error as SledError};

use crate::block::{Block, Hash};

#[derive(Debug)]
pub enum StorageError {
    Sled(SledError),
    Serialization(bincode::error::EncodeError),
    Deserialization(bincode::error::DecodeError),
}

impl From<SledError> for StorageError {
    fn from(err: SledError) -> Self {
        StorageError::Sled(err)
    }
}

impl From<EncodeError> for StorageError {
    fn from(err: EncodeError) -> Self {
        StorageError::Serialization(err)
    }
}

impl From<DecodeError> for StorageError {
    fn from(err: DecodeError) -> Self {
        StorageError::Deserialization(err)
    }
}

pub trait Storage {
    fn save_block(&self, block: &Block) -> Result<(), StorageError>;
    fn load_block(&self, hash: Hash) -> Result<Option<Block>, StorageError>;
}

pub struct SledStorage {
    db: Db,
}

impl SledStorage {
    pub fn new(path: &str) -> Result<Self, StorageError> {
        let db = sled::open(path)?;
        Ok(SledStorage { db })
    }
}

impl Storage for SledStorage {
    fn save_block(&self, block: &Block) -> Result<(), StorageError> {
        let data = bincode::serde::encode_to_vec(block, standard())?;
        self.db.insert(&block.hash, data)?;

        Ok(())
    }

    fn load_block(&self, hash: Hash) -> Result<Option<Block>, StorageError> {
        match self.db.get(hash)? {
            Some(data) => {
                let (block, _) = bincode::serde::decode_from_slice::<Block, _>(&data, standard())
                    .map_err(StorageError::from)?;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
pub mod mock_storage {
    use std::collections::HashMap;

    use super::*;
    
    #[allow(dead_code)]
    pub struct MockStorage {
        blocks: HashMap<String, Block>,
    }

    impl MockStorage {
        pub fn new() -> Self {
            MockStorage {
                blocks: HashMap::new(),
            }
        }
    }

    impl Storage for MockStorage {
        fn save_block(&self, _: &Block) -> Result<(), StorageError> {
            Ok(())
        }

        fn load_block(&self, _: Hash) -> Result<Option<Block>, StorageError> {
            Ok(Some(Block::genesis()))
        }
    }
}
