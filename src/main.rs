mod block_chain;
use block_chain::{BlockChain, Block};
use block_chain::permanence::SqliteDatabase;

use clap::{Arg, App};

fn main() {
    let database = SqliteDatabase::init();
    let mut blockchain = BlockChain::new(database);

    let args = App::new("BlockChain")
            .version("0.0.1")
            .about("BlockChain practice application.")
            .arg(Arg::with_name("command")
                .required(true)
                .index(1))
            .arg(Arg::with_name("data")
                .takes_value(true)
                .required(false))
                .get_matches();

    match args.value_of("command").unwrap() {
        "addblock" => {
            let data = args.value_of("data").unwrap().to_string();
            println!("Mining the block containing \"{}\"", &data);

            blockchain.add(Block::from(data));

            println!("Success!");
        },
        "status" => {
            for block in blockchain {
                println!("=================================== Block Data ===================================");
                println!("index: {}", block.index);
                println!("time: {}", block.timestamp);
                println!("prev: {}", hex::encode(&block.prev_hash));
                println!("curr: {}", hex::encode(&block.curr_hash));
                println!("data: {}", String::from_utf8(block.data.clone()).unwrap());
                println!("==================================================================================\n");
            }
        },
        _ => ()
    }
}

impl From<String> for Block {
    fn from(data: String) -> Self {
        let mut block = Block::new();
        let mut data = data.as_bytes().to_vec();
        data.resize(32, 0);
        block.data = data;
        block
    }
}