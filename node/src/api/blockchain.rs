use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode
};
use axum_macros::debug_handler;
use blockchain::block::Block;
use wallet_crypto::{
    keys::PublicKeyHash,
    transaction::{Transaction, UTXO},
};

use crate::{
    api::types::{NodeError, NodeState},
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
    Json(tx): Json<Transaction>,
) -> Result<Json<String>, NodeError> {
    let mut blockchain = blockchain.write().await;
    let peers = peers.lock().await;

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
pub async fn get_utxo_by_address(
    State(NodeState { blockchain, .. }): State<NodeState>,
    Path(address): Path<String>,
) -> Result<Json<Vec<UTXO>>, NodeError> {
    let blockchain = blockchain.read().await;
    let address = PublicKeyHash::try_from_string(&address).map_err(|_| NodeError::BadRequest("Address is incorrect hash value".to_string()))?;
    let utxos = blockchain.get_utxos_by_address(address).await;
    Ok(Json(utxos))
}
