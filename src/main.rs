use Sinope::blockchain::Blockchain;
use Sinope::blockchain::BlockchainIterator;
use Sinope::block::Block;
use Sinope::utils::dir_size;
use Sinope::proof::Proof;
use clap::{Arg, App};

//26110
fn main() {
    let matches = App::new("Sinope")
        .version("0.0.1")
        .author("Mutex Unlocked")
        .about("Sinope blockchain")
        .arg(Arg::with_name("addblock")
                 .short("a")
                 .long("addblock")
                 .takes_value(true)
                 .help("Adds a block to the blockchain"))
        .arg(Arg::with_name("print")
                 .short("p")
                 .long("print")
                 .takes_value(false)
                 .help("Prints the blockchain"))
        .get_matches();

    let data = matches.value_of("addblock").unwrap();

    let mut blockchain = Blockchain::new();
    blockchain.add(data.to_string());

    match matches.occurrences_of("print") {
        _ => print_blockchain(blockchain),
    }
}

fn print_blockchain(bc: Blockchain){
    let mut iterator = bc.iterator();

    loop{
        match iterator.next().ok(){
            Some(block) => {
                let mut b = block.unwrap();
                println!("Prev hash: {:?}", b.prev_hash().ok().unwrap());
                println!("Data: {}", b.data().ok().unwrap());
                println!("Hash: {:?}", b.hash().ok().unwrap());
                
                let mut proof = Proof::new(&mut b);
                println!("Valid: {:?}", proof.validate());

            },
            _ => { break; }
        }
    }
}
