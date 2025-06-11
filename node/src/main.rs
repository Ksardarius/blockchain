

use std::{net::SocketAddr, sync::{Arc}};

use api::blockchain;
use axum::{routing::{get, post}, Router};
use ::blockchain::{blockchain::Blockchain, data::storage::SledStorage};
use serde_json::from_str;
use tokio::sync::Mutex;

use crate::api::{peers::get_peers, types::NodeState};

mod api;
mod broadcast;

pub async fn load_peers_from_config(path: &str) -> Vec<String> {
    let file_content = tokio::fs::read_to_string(path).await.expect("Failed to read configuration file");
    from_str::<Vec<String>>(&file_content)
        .expect("Invalid JSON format")
        .into_iter()
        // .filter_map(|s| Url::parse(&s).ok())
        .collect()
}

#[tokio::main]
async fn main() {
    let peers = load_peers_from_config("peers.json").await;
    let storage = SledStorage::new("./data").unwrap();

    let blockchain = Arc::new(Mutex::new(Blockchain::new(storage)));
    let state = NodeState {
        blockchain,
        peers: Arc::new(Mutex::new(peers))
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/blocks", get(blockchain::get_blocks))
        .route("/transactions", post(blockchain::post_transaction))
        .route("/mine", post(blockchain::mine_block))
        .route("/balance/{address}", get(blockchain::get_balance))
        .route("/peers", get(get_peers))
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

