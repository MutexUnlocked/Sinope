use std::cell::RefCell;
use rocksdb::{DB, Options, Error};
use std::fs::File;
use std::collections::HashMap;
use crate::block::Block;
use crate::transcation::{Transaction, new_coinbase_t, Toutput, Tinput};

pub struct Blockchain {
    top: Option<Vec<u8>>,
    db: DB,
}

pub struct BlockchainIterator<'a> {
    db: &'a DB,
    current_hash: Vec<u8>,
}

impl Blockchain {
    pub fn new(address: String) -> Self{
        // Create the blockchain with the genesis block
        //println!("HERE");
        let db = DB::open_default("/tmp/apple").unwrap();
        let top: Option<Vec<u8>>;
        match db.get(b"l"){
            Ok(Some(value)) => top = Some(value),
            Ok(None) => {
                let mut gen_data = String::from("Why should I let data be misused?");
                let cbtx = new_coinbase_t(address, &mut gen_data);
                let cbtx_vec = vec![cbtx];
                let mut genesis = Block::genesis(cbtx_vec);
                println!("{:?}", genesis.hash().ok().unwrap());
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
        blockchain
    }

    pub fn add(&mut self, transcations: Vec<Transaction>){
        //println!("SIZE: {}", self.vec.len());
        match self.db.get(b"l"){
            Ok(Some(value)) => {
                let mut block = Block::new(value,transcations);
                self.db.put(block.hash().ok().unwrap(),block.serialize());
                self.db.put(b"l", block.hash().ok().unwrap());
                self.top = Some(block.hash().ok().unwrap().to_vec());
            },
            Ok(None) => println!("Did you create the blockchain?"),
            Err(e) => println!("Put failed {}", e),
        }
    }

    pub fn find_unspent_transactions(&self, address: &String) -> Vec<Transaction>{
        let mut unspent: Vec<Transaction> = Vec::new();
        let mut spent_t: HashMap<String, Vec<usize>> =  HashMap::new();
        let mut b_iterator = self.iterator();

        loop{
            let block = b_iterator.next();

    'outputs: for tr in block.clone().unwrap().unwrap().transactions().ok().unwrap(){
                let tid =  hex::encode(tr.id.as_ref().unwrap());

                for (i, out) in tr.vout.iter().enumerate() {
                    // Was the output spent
                    if !spent_t.get(&tid).is_none(){
                        for spent_output in spent_t[&tid].iter() {
                            if *spent_output == i {
                                continue 'outputs;
                            }
                        }
                    }

                    if out.can_unlock_with(address){
                        unspent.push(tr.clone());
                    }
                }

                if !tr.is_coinbase(){
                    for input in tr.vin.iter(){
                        if input.can_unlock_output_with(address){
                            let input_tr_id = hex::encode(input.transaction_id.clone());
                            spent_t.get_mut(&input_tr_id).unwrap().push(input.vout as usize);
                        }
                    }
                }
            }
            if block.unwrap().unwrap().prev_hash().ok().unwrap().is_empty(){
                break;
            }
        }
        unspent
    }

    pub fn find_utr(&self, address: &String) -> Vec<Toutput>{
        let unspet_trs = self.find_unspent_transactions(address);
        let mut utrs: Vec<Toutput> = Vec::new();

        for tr in unspet_trs.iter(){
            for out in tr.vout.iter() {
                if out.can_unlock_with(address){
                    utrs.push(out.clone());
                }
            }
        }
        utrs
    }
    

    pub fn iterator(&self) -> BlockchainIterator{
        BlockchainIterator{db: &self.db, current_hash: self.top.as_ref().unwrap().to_vec()}
    }
}

impl<'a> BlockchainIterator<'a> {
    pub fn next(&mut self) -> Result<Option<Block>, Error>{
        let block;
        match self.db.get(self.current_hash.to_vec()) {
            Ok(Some(value)) => {
                block = Block::deserialize(value);
                self.current_hash = block.prev_hash()
                        .ok().unwrap().to_vec();
                Ok(Some(block))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}