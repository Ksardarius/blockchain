use axum::{Json, extract::State, response::IntoResponse};
use axum_macros::debug_handler;

use crate::api::types::NodeState;

#[debug_handler]
pub async fn get_peers(State(NodeState { peers, .. }): State<NodeState>) -> impl IntoResponse {
    let peers = peers.lock().await;
    Json(peers.clone())
}
