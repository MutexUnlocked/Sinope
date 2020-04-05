#[macro_use]
extern crate lazy_static;

use Sinope::blockchain::Blockchain;
use Sinope::blockchain::BlockchainIterator;
use Sinope::block::Block;
use Sinope::utils::dir_size;
use Sinope::proof::Proof;
use clap::{Arg, App};

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref blockchain: Blockchain = Blockchain::new("Friedrich".to_string());
}

//26110
fn main() {
    let matches = App::new("Sinope")
        .version("0.0.1")
        .author("Mutex Unlocked")
        .about("Sinope blockchain")
        .arg(Arg::with_name("createblockchain")
                 .short("c")
                 .long("createblockchain")
                 .takes_value(true)
                 .help("Creates a blockchain"))
        .arg(Arg::with_name("print")
                 .short("p")
                 .long("print")
                 .takes_value(false)
                 .help("Prints the blockchain"))
        .arg(Arg::with_name("getbalance")
                 .short("g")
                 .long("getbalance")
                 .takes_value(true)
                 .help("Gets the balance"))
        .get_matches();

    //let data = matches.value_of("createblockchain").unwrap();
    let data1: String = matches.value_of("getbalance").unwrap().to_string();
    //blockchain = Blockchain::new(data.to_string());
    get_balance(data1);
    
    
    match matches.occurrences_of("print") {
        _ => print_blockchain(),
    }
}

fn print_blockchain(){
    let mut iterator = blockchain.iterator();

    loop{
        match iterator.next().ok(){
            Some(block) => {
                // TODO: Fix unwrap() None value problem
                let mut b = block.unwrap();
                println!("Prev hash: {:?}", b.prev_hash().ok().unwrap());
               // println!("Data: {}", b.data().ok().unwrap());
                println!("Hash: {:?}", b.hash().ok().unwrap());
                
                let mut proof = Proof::new(&mut b);
                println!("Valid: {:?}", proof.validate());

            },
            _ => { break; }
        }
    }
}

fn get_balance(address: String){
    //blockchain = Blockchain::new(address.clone());

    let mut balance: i32 = 0;
    let utrs = blockchain.find_utr(&address);

    for out in utrs {
        balance += out.val;
    }

    println!("{}'s balance is {}", address, balance);
}