use Sinope::blockchain::Blockchain;
use Sinope::block::Block;
use Sinope::utils::dir_size;


//26110
fn main() {
    println!("Sinope version 0.0.1...");

    let mut blockchain = Blockchain::new();
    blockchain.add("Ethan".to_string());
}
