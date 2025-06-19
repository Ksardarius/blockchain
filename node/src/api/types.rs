use axum::response::IntoResponse;
use axum::{Json, http::StatusCode};
use blockchain::{
    blockchain::{Blockchain, BlockchainError},
    data::storage::SledStorage,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

#[derive(Clone)]
pub struct NodeState {
    pub blockchain: Arc<RwLock<Blockchain<SledStorage>>>,
    pub peers: Arc<Mutex<Vec<String>>>,
}

#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error(transparent)]
    BlockchainError(#[from] BlockchainError),

    #[allow(dead_code)]
    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[allow(dead_code)]
    #[error("Unauthorized access")]
    Unauthorized,

    #[allow(dead_code)]
    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Axum rejection: {0}")]
    Axum(#[from] axum::extract::rejection::FailedToDeserializeFormBody),
}

impl IntoResponse for NodeError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            //NodeError::Storage(StorageError::BlockNotFound) => (StatusCode::NOT_FOUND, "Block not found".to_string()),
            NodeError::BlockchainError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Blockchain error: {}", e),
            ),
            NodeError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            NodeError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Authentication required".to_string(),
            ),
            NodeError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", msg),
            ),
            NodeError::Axum(e) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid request body: {}", e),
            ), // Handle Json parse errors nicely
        };

        let body = Json(json!({
            "error": error_message,
            "status_code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

#[derive(Deserialize)]
pub struct NewTransaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub timestamp: u128,
    pub signature: Option<Vec<u8>>,
}
