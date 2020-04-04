use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::proof::Proof;
use crate::transcation::Transaction;

pub enum BarErr {
    Nothing
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    nonce: Option<u64>, 
    timestamp: Option<u128>,
    transactions: Option<Vec<Transaction>>,
    hash: Option<Vec<u8>>,
    prev_hash: Option<Vec<u8>>,
}

impl Block {
    // Creates a new block
    pub fn new(prev_hash: Vec<u8>, transactions: Vec<Transaction>) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        let timestamp = since_the_epoch.as_millis();
        
        //TODO: Implement proof of work and fix nonce
        let mut b = Block{
            nonce: None,
            timestamp: Some(timestamp),
            transactions: Some(transactions),
            hash: None,
            prev_hash: Some(prev_hash),
        };
        let mut proof = Proof::new(&mut b);
        let (n, h) = proof.run();
        b.nonce = Some(n);
        b.hash = Some(h);
        b
    }

    pub fn hash_transactions(&self) -> Vec<u8>{
        let mut t_hash: Vec<u8> = Vec::new();
        for t in self.transactions().ok().unwrap().iter(){
            t_hash.append(&mut t.id.clone().unwrap());
        }
        t_hash
    }

    // Immutable access.
    pub fn transactions(&self) -> Result<&Vec<Transaction>, BarErr> {
        match self.transactions {
            Some(ref x) => Ok(x),
            None => Err(BarErr::Nothing)
        }
    }

    pub fn timestamp(&self) -> Result<&u128, BarErr> {
        match self.timestamp {
            Some(ref x) => Ok(x),
            None => Err(BarErr::Nothing)
        }
    }

    pub fn hash(&self) -> Result<&Vec<u8>, BarErr> {
        match self.hash {
            Some(ref x) => Ok(x),
            None => Err(BarErr::Nothing)
        }
    }
    pub fn prev_hash(&self) -> Result<&Vec<u8>, BarErr> {
        match self.prev_hash {
            Some(ref x) => Ok(x),
            None => Err(BarErr::Nothing)
        }
    }

    pub fn nonce(&self) -> Result<&u64, BarErr> {
        match self.nonce {
            Some(ref x) => Ok(x),
            None => Err(BarErr::Nothing)
        }
    }

    pub fn serialize(&self) -> Vec<u8>{
        bincode::serialize(&self).unwrap()
    }

    pub fn deserialize(encoded: Vec<u8>) -> Block{
        bincode::deserialize(&encoded[..]).unwrap()
    }


    pub fn genesis(coinbase: Vec<Transaction>) -> Self{
        let v: Vec<u8> = vec![0;0];
        let block = Block::new(v, coinbase);
        block
    }           
}