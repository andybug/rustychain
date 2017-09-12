#[macro_use]
extern crate serde_derive;

mod block;
mod chain;

use block::Block;
use chain::BlockChain;

fn add_shiz(chain: &mut BlockChain) {
    let mut b = Box::new(Block::new());
    b.set_timestamp_now();
    chain.append(b);
}

fn main() {
    let mut chain = BlockChain::new();
    add_shiz(&mut chain);
    add_shiz(&mut chain);

    println!("{}", chain);
}
