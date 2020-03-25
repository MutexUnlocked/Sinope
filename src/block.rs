use std::time::{SystemTime, UNIX_EPOCH};
use crate::proof::Proof;

pub enum BarErr {
    Nothing
}

pub struct Block {
    nonce: Option<u64>, 
    timestamp: Option<u128>,
    data: Option<String>,
    hash: Option<String>,
    prev_hash: Option<String>,
}

impl Block {
    // Creates a new block
    pub fn new(prev_hash: String, data: String) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        let timestamp = since_the_epoch.as_millis();
        
        //TODO: IMplement proof of work and fix nonce
        let mut b = Block{
            nonce: None,
            timestamp: Some(timestamp),
            data: Some(data),
            hash: None,
            prev_hash: Some(prev_hash),
        };
        let mut proof = Proof::new(&mut b);
        proof.run();
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

    pub fn hash(&self) -> Result<&String, BarErr> {
        match self.hash {
            Some(ref x) => Ok(x),
            None => Err(BarErr::Nothing)
        }
    }
    pub fn prev_hash(&self) -> Result<&String, BarErr> {
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


    pub fn genesis() -> Self{
        let mut gen = String::new();
        gen.push_str("GENSIS");
        let block = Block::new("".to_string(), gen);
        block
    }           
}