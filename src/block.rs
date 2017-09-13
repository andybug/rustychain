extern crate serde;
extern crate crypto;
extern crate byteorder;

use std::fmt;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use self::serde::ser::{Serialize, Serializer};
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::byteorder::{LittleEndian, WriteBytesExt};


pub const BLOCKHASH_BYTES: usize = 32;

#[derive(Eq, Hash, Copy, Clone, Deserialize, PartialEq)]
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
        let encoded: Vec<u8> = b.to_bytes();
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

    //fn to_string(&self) -> String {
    //    self.digest.to_hex()
    //}

    fn get_digest(&self) -> &[u8] {
        &self.digest
    }

    fn to_hex(&self) -> String {
        static CHARS: &'static[u8] = b"0123456789abcdef";
        let mut v = Vec::with_capacity(self.digest.len() * 2);
        for &byte in self.digest.iter() {
            v.push(CHARS[(byte >> 4) as usize]);
            v.push(CHARS[(byte & 0xf) as usize]);
        }

        unsafe {
            String::from_utf8_unchecked(v)
        }
    }
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Serialize for BlockHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.to_hex().as_str())
    }
}



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

    fn to_bytes(&self) -> Vec<u8> {
        const NUM_BYTES: usize = 76;
        let mut vec = vec![];

        vec.write_u32::<LittleEndian>(self.version).unwrap();
        vec.write_u64::<LittleEndian>(self.timestamp).unwrap();
        vec.write_all(self.previous.get_digest()).unwrap();
        vec.write_all(self.merkle_root.get_digest()).unwrap();

        assert_eq!(vec.len(), NUM_BYTES);

        vec
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let blockhash = BlockHash::hash(&self);
        write!(f, "--- block ---\n").unwrap();
        write!(f, "version:     {}\n", self.version).unwrap();
        write!(f, "timestamp:   {}\n", self.timestamp).unwrap();
        write!(f, "previous:    {}\n", self.previous).unwrap();
        write!(f, "merkle_root: {}\n", self.merkle_root).unwrap();
        write!(f, "_blockhash:  {}\n", blockhash).unwrap();
        write!(f, "-------------\n")
    }
}
