extern crate rustc_serialize;

use std::fmt;
use self::rustc_serialize::hex::ToHex;


const BLOCKHASH_BYTES: usize = 32;

#[derive(Copy,Clone)]
struct BlockHash {
    digest: [u8; BLOCKHASH_BYTES],
}

impl BlockHash {
    fn new() -> BlockHash {
        BlockHash {
            digest: [0u8; BLOCKHASH_BYTES],
        }
    }
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.digest.to_hex())
    }
}



pub struct Block {
    version: u32,
    timestamp: u64,
    prev: BlockHash,
    merkle_root: BlockHash,
}

impl Block {
    pub fn new() -> Block {
        Block {
            version: 1,
            timestamp: 0,
            prev: BlockHash::new(),
            merkle_root: BlockHash::new(),
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--- block ---\n").expect("write error");
        write!(f, "version:     {}\n", self.version).expect("write error");
        write!(f, "timestamp:   {}\n", self.timestamp).expect("write error");
        write!(f, "prev:        {}\n", self.prev).expect("write error");
        write!(f, "merkle_root: {}\n", self.merkle_root)
    }
}
