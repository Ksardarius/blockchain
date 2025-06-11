use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;

use crate::{
    api::types::{NewTransaction, NodeState},
    broadcast::{broadcast_block, broadcast_transaction},
};

#[debug_handler]
pub async fn get_blocks(
    State(NodeState { blockchain, .. }): State<NodeState>,
) -> impl IntoResponse {
    let blockchain = blockchain.lock().await;
    Json(blockchain.get_blocks())
}

#[debug_handler]
pub async fn post_transaction(
    State(NodeState { blockchain, peers }): State<NodeState>,
    Json(tx): Json<NewTransaction>,
) -> impl IntoResponse {
    let mut blockchain = blockchain.lock().await;
    let peers = peers.lock().await;
    if let Ok(transaction) = blockchain.create_transaction(&tx.sender, &tx.recipient, tx.amount) {
        broadcast_transaction(&peers, transaction).await;

        (StatusCode::OK, Json("Transaction added"))
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Transaction can not be added"),
        )
    }
}

#[debug_handler]
pub async fn mine_block(
    State(NodeState { blockchain, peers }): State<NodeState>,
    Json(address): Json<String>,
) -> impl IntoResponse {
    let mut blockchain = blockchain.lock().await;
    blockchain.mine_pending_transactions(&address).unwrap();

    let peers = peers.lock().await;
    broadcast_block(
        &peers,
        &blockchain
            .get_blocks()
            .last()
            .expect("Last block could not be found"),
    )
    .await;

    (StatusCode::OK, Json("Block created"))
}

#[debug_handler]
pub async fn get_balance(
    State(NodeState { blockchain, .. }): State<NodeState>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    let blockchain = blockchain.lock().await;
    let balance = blockchain.get_balance(&address);
    Json(balance)
}
