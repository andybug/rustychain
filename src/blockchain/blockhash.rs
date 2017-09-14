extern crate crypto;
extern crate serde;

use std::fmt;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::serde::ser::{Serialize, Serializer};

use blockchain::Block;


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

    pub fn get_digest(&self) -> &[u8] {
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
