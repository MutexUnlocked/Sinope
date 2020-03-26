use std::cell::RefCell;
use crate::block::Block;

pub struct Blockchain {
    vec: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self{
        // Create the blockchain with the genesis block
        let mut blockchain = Blockchain {vec: Vec::new()};
        let mut genesis = Block::genesis();
        blockchain.vec.push(genesis);
        blockchain
    }

    pub fn add(&mut self, data: String){
        println!("SIZE: {}", self.vec.len());

        match self.vec.last().unwrap().hash().ok() {
            Some(v) => {
                let mut block = Block::new(v.to_vec(), data);
                self.vec.push(block);
            },
            None => println!("The blockchain is empty"),
        }       
    }
}