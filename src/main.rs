#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs;
use std::path::Path;

mod blockchain;
use blockchain::BlockChain;

mod util;

fn create_cache_dir(dir: &Path) {
    match dir.exists() {
        true => {
            if !dir.is_dir() {
                panic!("invalid directory '{}'", dir.to_str().unwrap());
            }
        },
        false => {
            match fs::create_dir(dir) {
                Err(why) => panic!("create_dir failed '{}'", why),
                Ok(_) => {},
            }
        }
    }
}

fn main() {
    let mut args = env::args();
    let cache_arg = args.nth(1).unwrap();
    let cache_dir = Path::new(&cache_arg);

    create_cache_dir(&cache_dir);

    let mut chain = BlockChain::new();
    chain.read_chain(&cache_dir);
    println!("{}", chain);
}
