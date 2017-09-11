extern crate rustc_serialize;
extern crate bincode;
extern crate crypto;

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use self::rustc_serialize::hex::ToHex;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;


pub const BLOCKHASH_BYTES: usize = 32;

#[derive(Eq, Hash, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockHash {
    digest: [u8; BLOCKHASH_BYTES],
}

impl BlockHash {
    pub fn new() -> BlockHash {
        BlockHash {
            digest: [0u8; BLOCKHASH_BYTES],
        }
    }

    pub fn hash(b: &Block) -> BlockHash {
        // serialize struct to little endian vector of u8
        let limit = bincode::Bounded(76);
        let encoded: Vec<u8> = bincode::serialize(b, limit).unwrap();
        //println!("encoded='{}'", encoded.to_hex());

        let mut bhash = BlockHash::new();
        let mut sha = Sha256::new();

        // first round of hashing
        sha.input(&encoded);
        sha.result(&mut bhash.digest);

        sha.reset();

        // second round of hashing
        sha.input(&bhash.digest);
        sha.result(&mut bhash.digest);

        bhash
    }
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.digest.to_hex())
    }
}



#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
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

    pub fn set_timestamp(&mut self, ts: u64) {
        self.timestamp = ts;
    }

    pub fn set_timestamp_now(&mut self) {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        self.timestamp = since_epoch.as_secs();
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let blockhash = BlockHash::hash(&self);
        write!(f, "--- block ---\n").unwrap();
        write!(f, "version:     {}\n", self.version).unwrap();
        write!(f, "timestamp:   {}\n", self.timestamp).unwrap();
        write!(f, "prev:        {}\n", self.prev).unwrap();
        write!(f, "merkle_root: {}\n", self.merkle_root).unwrap();
        write!(f, "_blockhash:  {}\n", blockhash).unwrap();
        write!(f, "-------------\n")
    }
}
