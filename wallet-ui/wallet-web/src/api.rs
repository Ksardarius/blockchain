use gloo_console::log;
use gloo_net::{http::Request, Error};
use wallet_crypto::{keys::PublicKeyHash, transaction::{Transaction, TxOut, UTXO}};

pub struct NodeClient {
    base_url: String,
    next_request_id: u64,
}

impl NodeClient {
    pub fn new(base_url: &str) -> Self {
        NodeClient { 
            base_url: base_url.to_string(),
            next_request_id: 1 
        }
    }

    pub async fn get_utxos(&self, address: PublicKeyHash) -> Result<Vec<UTXO>, Error> {
        let response = Request::get(&format!("{}/{}/{}", self.base_url, "utxo", address.to_string_owned()))
            .send().await?;

        let data: Vec<UTXO> = response.json().await?;

        log!(format!("Response: {:?}", data));

        Ok(data)
    }

    pub async fn post_transaction(&self, tx: &Transaction) -> Result<(), Error> {
        let _ = Request::post(&format!("{}/{}", self.base_url, "transactions"))
            .json(tx)?
            .send().await?;

        Ok(())
    }
}



