extern crate byteorder;
extern crate crypto;
extern crate serde;

use std::fmt;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use self::byteorder::{LittleEndian, WriteBytesExt};
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

use blockchain::{BlockHash, BLOCKHASH_BYTES};


#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    version: u32,
    timestamp: u64,
    previous: BlockHash,
    merkle_root: BlockHash,
}

impl Block {
    pub fn new() -> Block {
        Block {
            version: 1,
            timestamp: 0,
            previous: BlockHash::new(),
            merkle_root: BlockHash::new(),
        }
    }

    //pub fn set_timestamp(&mut self, ts: u64) {
    //    self.timestamp = ts;
    //}

    pub fn set_timestamp_now(&mut self) {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        self.timestamp = since_epoch.as_secs();
    }

    pub fn set_previous(&mut self, p: BlockHash) {
        self.previous = p;
    }

    pub fn get_hash(&self) -> BlockHash {
        const NUM_BYTES: usize = 76;
        let mut vec = Vec::with_capacity(NUM_BYTES);

        // write struct fields to buffer (little endian)
        vec.write_u32::<LittleEndian>(self.version).unwrap();
        vec.write_u64::<LittleEndian>(self.timestamp).unwrap();
        vec.write_all(self.previous.get_digest()).unwrap();
        vec.write_all(self.merkle_root.get_digest()).unwrap();
        assert_eq!(vec.len(), NUM_BYTES);

        let mut sha = Sha256::new();
        let mut buf: [u8; BLOCKHASH_BYTES] = [0u8; BLOCKHASH_BYTES];

        // first round of hashing
        sha.input(&vec);
        sha.result(&mut buf);

        // second round of hashing
        sha.reset();
        sha.input(&buf);
        sha.result(&mut buf);

        BlockHash::from(&buf)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let blockhash = self.get_hash();
        write!(f, "--- block ---\n").unwrap();
        write!(f, "version:     {}\n", self.version).unwrap();
        write!(f, "timestamp:   {}\n", self.timestamp).unwrap();
        write!(f, "previous:    {}\n", self.previous).unwrap();
        write!(f, "merkle_root: {}\n", self.merkle_root).unwrap();
        write!(f, "_blockhash:  {}\n", blockhash).unwrap();
        write!(f, "-------------\n")
    }
}
