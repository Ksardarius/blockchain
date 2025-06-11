use std::future::Future;

use blockchain::block::Block;
use blockchain::transaction::Transaction;
use serde::Serialize;

pub fn broadcast_block(peers: &[String], block: &Block) -> impl Future<Output = ()> {
    broadcast_post(peers, "blocks", block)
}

pub fn broadcast_transaction(peers: &[String], transaction: &Transaction) -> impl Future<Output = ()> {
    broadcast_post(peers, "transactions", transaction)
}

async fn broadcast_post<T: Serialize>(peers: &[String], url: &str, data: &T) {
    // let client = reqwest::Client::new();
    
    // for peer in peers {
    //     let url = format!("{}/{}", peer, url);

    //     let res = client.post(&url)
    //         .json(data)
    //         .send()
    //         .await;

    //     if let Err(err) = res {
    //         eprintln!("Failed to send to {}: {}", peer, err);
    //         continue;
    //     }
    // }
}