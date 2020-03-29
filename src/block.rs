use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::proof::Proof;

pub enum BarErr {
    Nothing
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    nonce: Option<u64>, 
    timestamp: Option<u128>,
    data: Option<String>,
    hash: Option<Vec<u8>>,
    prev_hash: Option<Vec<u8>>,
}

impl Block {
    // Creates a new block
    pub fn new(prev_hash: Vec<u8>, data: String) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        let timestamp = since_the_epoch.as_millis();
        
        //TODO: Implement proof of work and fix nonce
        let mut b = Block{
            nonce: None,
            timestamp: Some(timestamp),
            data: Some(data),
            hash: None,
            prev_hash: Some(prev_hash),
        };
        let mut proof = Proof::new(&mut b);
        let (n, h) = proof.run();
        b.nonce = Some(n);
        b.hash = Some(h);
        b
    }

    // Immutable access.
    pub fn data(&self) -> Result<&String, BarErr> {
        match self.data {
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


    pub fn genesis() -> Self{
        let mut gen = String::new();
        let v: Vec<u8> = vec![0;0];
        gen.push_str("GENSIS");
        let block = Block::new(v, gen);
        block
    }           
}