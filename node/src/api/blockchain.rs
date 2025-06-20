use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use blockchain::{
    block::Block
};
use wallet_crypto::{keys::{BlockchainHash, PublicKeyHash, Signature}, scripts::Script, transaction::{Transaction, TxIn, TxOut}};

use crate::{
    api::types::{NewTransaction, NodeError, NodeState},
    broadcast::{broadcast_block, broadcast_transaction},
};

#[debug_handler]
pub async fn get_blocks(
    State(NodeState { blockchain, .. }): State<NodeState>,
) -> Result<Json<Vec<Block>>, NodeError> {
    let blockchain = blockchain.read().await;
    let blocks = blockchain.get_blocks().await?;
    Ok(Json(blocks))
}

#[debug_handler]
pub async fn post_transaction(
    State(NodeState { blockchain, peers }): State<NodeState>,
    Json(tx): Json<NewTransaction>,
) -> Result<Json<String>, NodeError> {
    let mut blockchain = blockchain.write().await;
    let peers = peers.lock().await;

    let tx = Transaction::new(
        vec![TxIn {
            prev_tx_id: BlockchainHash::new([0x08; 32]), // Corrected syntax
            prev_out_idx: 1,
            script_sig: Signature(vec![]) ,
            sequence: 0,
        }],
        vec![TxOut {
            value: 50,
            script_pubkey: Script::PayToPublicKeyHash {
                pub_key_hash: PublicKeyHash::new([0x1; 20]),
            },
        }],
    );

    let tx = blockchain.add_transaction(tx).await?;
    broadcast_transaction(&peers, &tx).await;

    Ok(Json("Transaction added".to_string()))
}

#[debug_handler]
pub async fn mine_block(
    State(NodeState { blockchain, peers }): State<NodeState>,
    Json(address): Json<String>,
) -> Result<(StatusCode, Json<String>), NodeError> {
    let mut blockchain = blockchain.write().await;
    blockchain.mine_pending_transactions().await.unwrap();

    let peers = peers.lock().await;
    broadcast_block(
        &peers,
        &blockchain
            .get_blocks()
            .await?
            .last()
            .expect("Last block could not be found"),
    )
    .await;

    Ok((StatusCode::OK, Json("Block created".to_string())))
}

#[debug_handler]
pub async fn get_balance(
    State(NodeState { blockchain, .. }): State<NodeState>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    // let blockchain = blockchain.lock().await;
    // let balance = blockchain.get_balance(&address);
    Json(0)
}
