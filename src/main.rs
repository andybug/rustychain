#[macro_use]
extern crate serde_derive;

mod block;
use block::{Block, BlockHash};

fn main() {
    let b = Block::new();
    print!("{}", b);

    let bh = BlockHash::hash(&b);
    println!("blockhash='{}'", bh);
}
