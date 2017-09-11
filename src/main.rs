#[macro_use]
extern crate serde_derive;

mod block;
use block::Block;

fn main() {
    let mut b = Block::new();
    print!("{}", b);

    b.set_timestamp_now();
    print!("{}", b);
}
