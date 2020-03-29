use crate::block::Block;
use sha2::{Sha256, Digest};
use std::cmp::Ordering;
use num::bigint::{BigInt, Sign, ToBigInt};

//use bytes::{BytesMut, BufMut};

const TARGET_BITS: usize = 24;

type Pdata = (u64, Vec<u8>);

pub struct Proof<'s>{
    block: &'s mut Block,
    target: BigInt,
}

impl<'a> Proof<'a> {
    pub fn new(block: &'a mut Block) -> Self{
        let y: usize = 256 - TARGET_BITS;
        let target = 1.to_bigint().unwrap();        

        let target = target << y;
        Proof{block, target}
    }

    fn prepare_data(&self, nonce: u64) -> Vec<u8>{
        //TODO: add everything to Vec<u8>
        let mut result: Vec<u8> = Vec::<u8>::new();
        let mut tmp: Vec<u8> = self.block.data().ok().unwrap().clone().into_bytes();
        result.append(&mut self.block.prev_hash().ok().unwrap().to_vec());
        result.append(&mut tmp);
        result.append(&mut hex::encode(self.block.timestamp().ok().unwrap().to_string()).into_bytes());
        result.append(&mut hex::encode(TARGET_BITS.to_string()).into_bytes());
        result.append(&mut nonce.to_string().into_bytes());
        result
    }

    // TODO: FIX THIS SHIT
    pub fn run(&mut self) -> Pdata{
        let mut hash: Vec<u8> = vec![0;4];
        let mut hash_int;
        let mut nonce: u64 = 0;
        let max_nonce = u64::max_value();

        println!("Mining the block containing {}", self.block.data().ok().unwrap());
        while nonce < max_nonce {
            let data = self.prepare_data(nonce);
            let mut hasher = Sha256::new();
            hasher.input(data);
            hash = hasher.result().to_vec();

            hash_int = BigInt::from_bytes_le(Sign::Plus, &hash);
            // println!("HASH: {}", hash_int);
            // println!("NONCE: {}", nonce);
            // println!("TARGET: {}", &self.target);

            
            if hash_int.cmp(&self.target) == Ordering::Less{
                break;
            }else{
                nonce = nonce + 1;
            }
        }
    
        println!("Hash: {:?}", hash);
        (nonce, hash)
    }

   
}