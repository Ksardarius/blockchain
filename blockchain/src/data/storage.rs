use bincode::{
    self,
    config::standard,
    error::{DecodeError, EncodeError},
};
use sled::{Db, Error as SledError, IVec};
use tokio::{
    sync::mpsc,
    task::{self, JoinError},
};

use crate::block::Block;

type Hash = [u8; 32];

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Sled database error: {0}")]
    Sled(SledError),
    #[error("Serialization error: {0}")]
    Serialization(bincode::error::EncodeError),
    #[error("Deserialization error: {0}")]
    Deserialization(bincode::error::DecodeError),
    #[error("Task join error: {0}")]
    JoinError(JoinError),
    #[error("Block not found")]
    BlockNotFound,
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

impl From<JoinError> for StorageError {
    fn from(err: JoinError) -> Self {
        StorageError::JoinError(err)
    }
}

#[async_trait::async_trait]
pub trait Storage {
    async fn set_latest_block_hash(&self, hash: Hash) -> Result<(), StorageError>;
    async fn get_latest_block_hash(&self) -> Result<Option<Hash>, StorageError>;
    async fn get_latest_block(&self) -> Result<Block, StorageError>;

    async fn save_block(&self, block: Block) -> Result<Block, StorageError>;
    async fn load_block(&self, hash: Hash) -> Result<Option<Block>, StorageError>;

    async fn stream_blocks_by_height(
        &self,
    ) -> Result<mpsc::Receiver<Result<Block, StorageError>>, StorageError>;
}

pub struct SledStorage {
    db: Db,
}

impl SledStorage {
    pub fn new(path: &str) -> Result<Self, StorageError> {
        let db = sled::open(path)?;
        Ok(SledStorage { db })
    }

    // Helper to format block height for keys (for ordered iteration)
    // Using 20 digits for u64 and zero-padding ensures lexicographical sort order matches numeric order.
    fn format_height_key(height: u64) -> [u8; 15] {
        const HEIGHT_PREFIX: &[u8; 7] = b"height_";
        let height_bytes = height.to_be_bytes();

        let mut key_array = [0u8; 15];
        key_array[0..HEIGHT_PREFIX.len()].copy_from_slice(HEIGHT_PREFIX);
        key_array[HEIGHT_PREFIX.len()..].copy_from_slice(&height_bytes);

        key_array
    }

    fn format_hash_key(hash: &Hash) -> [u8; 37] {
        const HASH_PREFIX: &[u8; 5] = b"hash_";

        let mut key_array = [0u8; 37];
        key_array[0..HASH_PREFIX.len()].copy_from_slice(HASH_PREFIX);
        key_array[HASH_PREFIX.len()..].copy_from_slice(hash);

        key_array
    }
}

#[async_trait::async_trait]
impl Storage for SledStorage {
    async fn set_latest_block_hash(&self, hash: Hash) -> Result<(), StorageError> {
        let db = self.db.clone();

        task::spawn_blocking(move || {
            let key = b"latest_block_hash";
            db.insert(key, &hash).map_err(StorageError::Sled)?;

            Ok::<(), StorageError>(())
        })
        .await??;

        Ok(())
    }

    async fn get_latest_block_hash(&self) -> Result<Option<Hash>, StorageError> {
        let db = self.db.clone();

        task::spawn_blocking(move || {
            let key = b"latest_block_hash";
            match db.get(key)? {
                Some(data) => {
                    let hash: Hash = data.as_ref().try_into().unwrap();
                    Ok(Some(hash))
                }
                None => Ok::<Option<Hash>, StorageError>(None),
            }
        })
        .await?
    }

    async fn save_block(&self, block: Block) -> Result<Block, StorageError> {
        let db = self.db.clone();

        task::spawn_blocking(move || {
            // iVec to not fully clone data on 2 inserts
            let value_bytes: IVec = bincode::serde::encode_to_vec(&block, standard())
                .map_err(StorageError::Serialization)?
                .into();

            // Store by height
            let height_key = SledStorage::format_height_key(block.height);
            db.insert(height_key, value_bytes.clone())
                .map_err(StorageError::Sled)?;

            // Store by hash
            let hash_key = SledStorage::format_hash_key(block.hash.as_ref());
            db.insert(hash_key, value_bytes)
                .map_err(StorageError::Sled)?;

            Ok::<Block, StorageError>(block)
        })
        .await?
    }

    async fn load_block(&self, hash: Hash) -> Result<Option<Block>, StorageError> {
        let db = self.db.clone();

        task::spawn_blocking(move || {
            let hash_key = SledStorage::format_hash_key(&hash);
            match db.get(hash_key)? {
                Some(data) => {
                    let (block, _) =
                        bincode::serde::decode_from_slice::<Block, _>(&data, standard())
                            .map_err(StorageError::Deserialization)?;
                    Ok(Some(block))
                }
                None => Ok::<Option<Block>, StorageError>(None),
            }
        })
        .await?
    }

    async fn stream_blocks_by_height(
        &self,
    ) -> Result<mpsc::Receiver<Result<Block, StorageError>>, StorageError> {
        const HEIGHT_PREFIX: &[u8; 7] = b"height_";
        let (tx, rx) = mpsc::channel(100);

        let db_clone = self.db.clone();

        tokio::task::spawn_blocking(move || {
            let iter = db_clone.scan_prefix(HEIGHT_PREFIX);

            for iter_res in iter {
                let block_result =
                    iter_res
                        .map_err(StorageError::Sled)
                        .and_then(|(_key, value)| {
                            bincode::serde::decode_from_slice::<Block, _>(&value, standard())
                                .map(|res| res.0)
                                .map_err(StorageError::Deserialization)
                        });

                if let Err(_) = tx.blocking_send(block_result) {
                    break;
                }
            }
        })
        .await?;

        Ok(rx)
    }

    async fn get_latest_block(&self) -> Result<Block, StorageError> {
        const HEIGHT_PREFIX: &[u8; 7] = b"height_";

        let db_clone = self.db.clone();

        tokio::task::spawn_blocking(move || {
            let result = db_clone.scan_prefix(HEIGHT_PREFIX).rev().next();

            match result {
                Some(Ok((_key, value))) => {
                    bincode::serde::decode_from_slice::<Block, _>(&value, standard())
                        .map(|res| res.0)
                        .map_err(StorageError::Deserialization)
                }
                Some(Err(e)) => Err(StorageError::Sled(e)), // An error occurred during sled operation
                None => Err(StorageError::BlockNotFound),   // No blocks found with the prefix
            }
        })
        .await?
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

    #[async_trait::async_trait]
    impl Storage for MockStorage {
        async fn save_block(&self, _: Block) -> Result<Block, StorageError> {
            Ok(Block::genesis())
        }

        async fn load_block(&self, _: Hash) -> Result<Option<Block>, StorageError> {
            Ok(Some(Block::genesis()))
        }

        async fn set_latest_block_hash(&self, _: Hash) -> Result<(), StorageError> {
            todo!()
        }

        async fn get_latest_block_hash(&self) -> Result<Option<Hash>, StorageError> {
            todo!()
        }

        async fn get_latest_block(&self) -> Result<Block, StorageError> {
            todo!()
        }

        async fn stream_blocks_by_height(
            &self,
        ) -> Result<mpsc::Receiver<Result<Block, StorageError>>, StorageError> {
            todo!()
        }
    }
}
