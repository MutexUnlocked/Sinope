use crate::block::Block;
use sha2::{Sha256, Sha512, Digest};
use std::cmp::Ordering;
use num::bigint::{BigInt, Sign, ToBigInt};

//use bytes::{BytesMut, BufMut};

const TARGET_BITS: i32 = 24;

type Pdata = (u64, Vec<u8>);

pub struct Proof<'s>{
    block: &'s mut Block,
    target: BigInt,
}

impl<'a> Proof<'a> {
    pub fn new(block: &'a mut Block) -> Self{
        let y = 256 - TARGET_BITS;
        let y = y.to_bigint().unwrap();
        let x = 1.to_bigint().unwrap();        

        let target = x * (ToBigInt::to_bigint(&2).unwrap()^y);
        Proof{block, target}
    }

    fn prepare_data(&self, nonce: u64) -> String{
        let mut result = String::new();
        result.push_str(self.block.prev_hash().ok().unwrap());
        result.push_str(self.block.data().ok().unwrap());
        result.push_str(&hex::encode(self.block.timestamp().ok().unwrap().to_string()));
        result.push_str(&hex::encode(TARGET_BITS.to_string()));
        result.push_str(&hex::encode(self.block.nonce().ok().unwrap().to_string()));
        result
    }

    pub fn run(&mut self) -> Pdata{
        let mut hash: Vec<u8> = vec![0;8];
        let mut hashInt;
        let mut nonce: u64 = 0;
        let maxNonce = u64::max_value();

        println!("Mining the block containing {}", self.block.data().ok().unwrap());
        while nonce < maxNonce {
            let data = self.prepare_data(nonce);
            let mut hasher = Sha256::new();
            hasher.input(data);
            hash = hasher.result().to_vec();

            hashInt = BigInt::from_bytes_le(Sign::Plus, &hash);
            
            if hashInt.cmp(&self.target) == Ordering::Less{
                break;
            }else{
                nonce = nonce + 1;
            }
        }
        println!("Hash: {:?}", hash);
        (nonce, hash)
    }

   
}