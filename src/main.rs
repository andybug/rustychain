#[macro_use]
extern crate serde_derive;

use std::path::Path;

mod blockchain;
use blockchain::{Block, BlockChain};

fn add_shiz(chain: &mut BlockChain) {
    let mut b = Box::new(Block::new());
    b.set_timestamp_now();
    chain.append(b);
}

fn main() {
    let mut chain = BlockChain::new();
    add_shiz(&mut chain);
    add_shiz(&mut chain);

    chain.write_chain(Path::new("/home/andy/tmp/rc"));
    println!("{}", chain);
}
