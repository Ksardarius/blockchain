use wallet_crypto::{
    keys::{BlockchainHash, KeyPair, PublicKeyHash},
    scripts::Script,
    transaction::{DraftTransaction, TxOut, UnsignedTxIn},
};
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::storage::{get_all_accounts, get_keypair, store_private_key};

mod crypto;
mod storage;

#[wasm_bindgen]
pub fn generate_new_key_pair() -> Result<JsValue, JsValue> {
    let keypair = KeyPair::generate();
    let js_value = serde_json::to_string(&keypair)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;

    Ok(JsValue::from_str(&js_value))
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&format!("Hello, {} from Rust WebAssembly!", name).into());
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    console::log_1(&format!("Adding {} and {} in Rust Wasm...", a, b).into());
    a + b
}

#[wasm_bindgen]
pub async fn create_wallet(password: &str) -> Result<(), JsValue> {
    let keypair = KeyPair::generate();
    store_private_key(keypair, password).await.unwrap();

    Ok(())
}

#[wasm_bindgen]
pub async fn get_all_wallets() -> Result<Vec<String>, JsValue> {
    let accounts = get_all_accounts().await.unwrap();
    Ok(accounts)
}

#[wasm_bindgen]
pub async fn create_transaction(
    own_address: &str,
    password: &str,
    recipient: &str,
    amount: u64,
) -> Result<JsValue, JsValue> {
    let key = get_keypair(own_address, password).await.unwrap();
    if let Some(keypair) = key {
        let txid_prev = BlockchainHash::default(); // Simulate a previous transaction ID

        // TODO: must be used existing utxo
        let tx_in = UnsignedTxIn {
            prev_tx_id: txid_prev,
            prev_out_idx: 0,
            sequence: 0xFFFFFFFF,
        };

        // TODO: value must be passed in
        let tx_out = TxOut {
            value: amount,
            script_pubkey: Script::PayToPublicKeyHash {
                pub_key_hash: PublicKeyHash::try_from_string(recipient)?, //keypair.public_key.to_address(),
            },
        };

        let tx = DraftTransaction::new(vec![tx_in], vec![tx_out]);
        let tx = tx.sign(&keypair);

        // verify correctness
        tx.verify_signatures().map_err(|er| er.to_string())?;

        let val = serde_wasm_bindgen::to_value(&tx).unwrap();

        return Ok(val);
    }

    Err(JsValue::from_str("Unable to create transaction"))
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console::log_1(&"Rust WebAssembly module loaded!".into());
    Ok(())
}
