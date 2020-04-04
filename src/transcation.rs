use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

const SUBSIDY: i32 = 10;

#[derive(Serialize, Deserialize)]
pub struct Touput {
    val: i32,
    pub_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tinput{
    transaction_id: Vec<u8>,
    vout: i32,
    script_sig: String,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<Vec<u8>>,
    vin: Tinput,
    vout: Touput,
}


impl Transaction {
    pub fn set_id(&mut self){
        let enc = bincode::serialize(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.input(enc);
        let hash = hasher.result().to_vec();

        self.id = Some(hash);
    }
}


pub fn new_coinbase_t(to: String, data: &mut String) -> Transaction{
    if data == ""{
        data.push_str("Reward to ");
        data.push_str(&to);
    }

    let tin = Tinput{transaction_id: b"".to_vec(), vout: -1 as i32, script_sig: data.to_string()};
    let tout = Touput{val:SUBSIDY,pub_key: to};
    let mut tx = Transaction{id: None, vin: tin, vout: tout};
    tx.set_id();
    tx
}
