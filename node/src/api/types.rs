use std::sync::{Arc};

use blockchain::{blockchain::Blockchain, data::storage::SledStorage};
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct NodeState {
    pub blockchain: Arc<Mutex<Blockchain<SledStorage>>>,
    pub peers: Arc<Mutex<Vec<String>>>
}


#[derive(Deserialize)]
pub struct NewTransaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub timestamp: u128,
    pub signature: Option<Vec<u8>>,
}