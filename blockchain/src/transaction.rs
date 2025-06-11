use chrono::Utc;
use serde::{Deserialize, Serialize};
use wallet::{Wallet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub timestamp: u128,
    pub signature: Option<Vec<u8>>,
}

impl Transaction {
    pub fn new(sender: &str, recipient: &str, amount: u64, wallet: Option<&Wallet>) -> Transaction {
        let timestamp = Utc::now().timestamp_millis() as u128;
        let message = Self::signable_message(sender, recipient, amount, timestamp);
        let signature = wallet.map(|w| w.sign(&message).to_vec());
        
        Transaction {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
            timestamp,
            signature
        }
    }

    pub fn verify(&self, wallet: &Wallet) -> bool {
        if let Some(signature) = &self.signature {
            let message = Self::signable_message(&self.sender, &self.recipient, self.amount, self.timestamp);
            wallet.verify(&message, signature)
        } else {
            false
        }
    }

    #[inline]
    fn signable_message(sender: &str, recipient: &str, amount: u64, timestamp: u128) -> Vec<u8> {
        let mut message = Vec::with_capacity(sender.len() + recipient.len() + 40);
        message.extend_from_slice(sender.as_bytes());
        message.extend_from_slice(recipient.as_bytes());
        message.extend_from_slice(amount.to_string().as_bytes());
        message.extend_from_slice(timestamp.to_string().as_bytes());
        
        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_unsigned_transaction() {
        let transaction = Transaction::new("S1", "R1", 100, None);

        assert_eq!(transaction.amount, 100);
        assert!(transaction.signature.is_none());
    }

    #[test]
    fn it_can_create_signed_transaction() {
        let wallet = Wallet::new();
        let transaction = Transaction::new("S1", "R1", 100, Some(&wallet));

        println!("{:?}", transaction.signature);

        assert_eq!(transaction.amount, 100);
        assert!(transaction.signature.is_some());
    }

    #[test]
    fn it_can_verify_signed_transaction() {
        let wallet = Wallet::new();
        let transaction = Transaction::new("S1", "R1", 100, Some(&wallet));

        assert!(transaction.verify(&wallet));
    }
}
