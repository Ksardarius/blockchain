use http::Method;
use std::{net::SocketAddr, sync::Arc};

use ::blockchain::{blockchain::Blockchain, data::storage::SledStorage};
use api::blockchain;
use axum::{
    Router,
    routing::{get, post},
};
use serde_json::from_str;
use tokio::sync::{Mutex, RwLock};
use tower_http::cors::{Any, CorsLayer};

use crate::api::{peers::get_peers, types::NodeState};

mod api;
mod broadcast;

pub async fn load_peers_from_config(path: &str) -> Vec<String> {
    let file_content = tokio::fs::read_to_string(path)
        .await
        .expect("Failed to read configuration file");
    from_str::<Vec<String>>(&file_content)
        .expect("Invalid JSON format")
        .into_iter()
        // .filter_map(|s| Url::parse(&s).ok())
        .collect()
}

#[tokio::main]
async fn main() {
    let peers =
        load_peers_from_config("/Users/morlovs/Projects/rust/rust_chain/node/peers.json").await;
    let storage = SledStorage::new("/Users/morlovs/Projects/rust/rust_chain/node/data").unwrap();

    let blockchain = Blockchain::new(storage);
    let blockchain = blockchain.init().await.unwrap();

    let blockchain = Arc::new(RwLock::new(blockchain));
    let state = NodeState {
        blockchain,
        peers: Arc::new(Mutex::new(peers)),
    };

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/blocks", get(blockchain::get_blocks))
        .route("/transactions", post(blockchain::post_transaction))
        .route("/mine", post(blockchain::mine_block))
        .route("/utxo/{address}", get(blockchain::get_utxo_by_address))
        .route("/peers", get(get_peers))
        .layer(cors)
        .with_state(state);

    // .route("/mine", post(mine_block))
    // .route("/balance/:address", get(get_balance));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8989));
    println!("🚀 Listening on http://{}", addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8989").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Blockchain Node is running"
}
