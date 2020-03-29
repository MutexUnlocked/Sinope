use std::cell::RefCell;
use rocksdb::{DB, Options};
use std::fs::File;
use crate::block::Block;

pub struct Blockchain {
    top: Option<Vec<u8>>,
    db: DB,
}



impl Blockchain {
    pub fn new() -> Self{
        // Create the blockchain with the genesis block
        let db = DB::open_default("blockchain").unwrap();
        let top: Option<Vec<u8>>;
        match db.get(b"l"){
            Ok(Some(value)) => top = Some(value),
            Ok(None) => {
                let mut genesis = Block::genesis();
                db.put(genesis.hash().ok().unwrap(),genesis.serialize());
                db.put(b"l",genesis.hash().ok().unwrap());
                top = Some(genesis.hash().ok().unwrap().to_vec());
            },
            Err(e) => {
                println!("Put failed {}", e);
                top = None;
            },
        }
        let mut blockchain = Blockchain {top,db};
        return blockchain;
        
    }

    pub fn add(&mut self, data: String){
        //println!("SIZE: {}", self.vec.len());
        match self.db.get(b"l"){
            Ok(Some(value)) => {
                let mut block = Block::new(value,data);
                self.db.put(block.hash().ok().unwrap(),block.serialize());
                self.db.put(b"l", block.hash().ok().unwrap());
                self.top = Some(block.hash().ok().unwrap().to_vec());
            },
            Ok(None) => println!("Did you create the blockchain?"),
            Err(e) => println!("Put failed {}", e),
        }
    }
}