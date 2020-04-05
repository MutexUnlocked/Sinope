use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

const SUBSIDY: i32 = 10;

#[derive(Serialize, Deserialize, Clone)]
pub struct Toutput {
    pub val: i32,
    pub_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tinput{
    pub transaction_id: Vec<u8>,
    pub vout: i32,
    script_sig: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Option<Vec<u8>>,
    pub vin: Vec<Tinput>,
    pub vout: Vec<Toutput>,
}


impl Transaction {
    pub fn set_id(&mut self){
        let enc = bincode::serialize(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.input(enc);
        let hash = hasher.result().to_vec();

        self.id = Some(hash);
    }

    pub fn is_coinbase(&self) -> bool {
        unimplemented!()
    }
}

impl Tinput{
    pub fn can_unlock_output_with(&self, ud: &String) -> bool{
        self.script_sig == *ud
    }
}


impl Toutput{
    pub fn can_unlock_with(&self, ud: &String) -> bool {
        self.pub_key == *ud
    }
}


pub fn new_coinbase_t(to: String, data: &mut String) -> Transaction{
    if data == ""{
        data.push_str("Reward to ");
        data.push_str(&to);
    }

    let tin = Tinput{transaction_id: b"".to_vec(), vout: -1 as i32, script_sig: data.to_string()};
    let tout = Toutput{val:SUBSIDY,pub_key: to};
    let mut tx = Transaction{id: None, vin: vec![tin], vout: vec![tout]};
    tx.set_id();
    tx
}
