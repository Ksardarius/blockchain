use gloo_console::log;
use indexed_db_futures::error::Error;
use indexed_db_futures::prelude::QuerySource;
use indexed_db_futures::transaction::TransactionMode;
use indexed_db_futures::{Build, database::Database};
use wallet_crypto::keys::KeyPair;
use wasm_bindgen::JsValue;

use crate::crypto::{decrypt_data_aes_gcm, encrypt_data_aes_gcm};

const DB_NAME: &str = "MyBlockchainWalletDB";
const STORE_NAME: &str = "wallets";
const DB_VERSION: u8 = 2;

pub async fn open_db(name: &str, version: u8) -> Result<Database, Error> {
    let db: Database = Database::open(name)
        .with_version(version)
        .with_on_upgrade_needed(|event, db| {
            // Convert versions from floats to integers to allow using them in match expressions
            let old_version = event.old_version() as u64;
            let new_version = event.new_version().map(|v| v as u64);

            match (old_version, new_version) {
                (0, Some(1)) => {
                    db.create_object_store(STORE_NAME)
                        // .with_auto_increment(true)
                        .build()?;
                }
                (prev, Some(2)) => {
                    if prev == 1 {
                        let _ = db.delete_object_store(STORE_NAME);
                    }

                    db.create_object_store(STORE_NAME).build()?;
                }
                _ => {}
            }

            Ok(())
        })
        .await
        .map_err(|e| JsValue::from_str(&format!("IndexedDB open error: {:?}", e)))?;
    Ok(db)
}

pub async fn store_private_key(key: KeyPair, password: &str) -> Result<(), Error> {
    let db_key = key.public_key.to_address();

    let private_key = key.secret_key.0.to_bytes().to_vec();
    let encrypted_key = encrypt_data_aes_gcm(private_key, password).await?;

    let db = open_db(DB_NAME, DB_VERSION).await?;

    // Start a readwrite transaction
    let transaction = db
        .transaction(STORE_NAME)
        .with_mode(TransactionMode::Readwrite)
        .build()?;

    let object_store = transaction.object_store(STORE_NAME)?;

    object_store
        .put(encrypted_key.as_str())
        .with_key(db_key.to_string_owned().as_str())
        .await?;

    transaction.commit().await?;

    log!(&format!(
        "Key entry '{}' put successfully.",
        db_key.to_string_owned()
    ));

    Ok(())
}

pub async fn get_all_accounts() -> Result<Vec<String>, Error> {
    let db = open_db(DB_NAME, DB_VERSION).await?;

    // Start a readwrite transaction
    let transaction = db
        .transaction(STORE_NAME)
        .with_mode(TransactionMode::Readonly)
        .build()?;

    let object_store = transaction.object_store(STORE_NAME)?;
    let keys = object_store.get_all_keys::<String>().await?;

    let result: Vec<String> = keys.map(|k| k.unwrap()).collect();
    Ok(result)
}

pub async fn get_keypair(address: &str, password: &str) -> Result<Option<KeyPair>, Error> {
    let db = open_db(DB_NAME, DB_VERSION).await?;

    // Start a readwrite transaction
    let transaction = db
        .transaction(STORE_NAME)
        .with_mode(TransactionMode::Readonly)
        .build()?;

    let object_store = transaction.object_store(STORE_NAME)?;

    let result: Option<String> = object_store.get(address).await?;

    if let Some(encoded_key) = result {
        let private_key = decrypt_data_aes_gcm(encoded_key, password).await.unwrap();
        let keypair = KeyPair::from_private_key(&private_key);
        Ok(Some(keypair))
    } else {
        Ok(None)
    }
}
