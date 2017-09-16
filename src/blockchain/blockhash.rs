extern crate crypto;
extern crate serde;

use std::fmt;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::serde::ser::{Serialize, Serializer};
use self::serde::de::{Visitor, Deserialize, Deserializer};

use blockchain::Block;

//mod util;
use util::hex::{FromHex, ToHex};


pub const BLOCKHASH_BYTES: usize = 32;

#[derive(Eq, Hash, Copy, Clone, PartialEq)]
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
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.digest.to_hex())
    }
}

impl Serialize for BlockHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.digest.to_hex().as_str())
    }
}

impl<'de> Deserialize<'de> for BlockHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct BlockHashVisitor;

        impl<'de> Visitor<'de> for BlockHashVisitor {
            type Value = BlockHash;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("sha256 sum in hex (str)")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where E: serde::de::Error
            {
                let byte_vec = s.from_hex().unwrap();
                let mut digest = [0u8; BLOCKHASH_BYTES];
                for i in 0..BLOCKHASH_BYTES {
                    digest[i] = byte_vec[i];
                }
                Ok(BlockHash { digest: digest })
            }

        }

        const FIELDS: &'static [&'static str] = &["digest"];
        deserializer.deserialize_struct("BlockHash", FIELDS, BlockHashVisitor)
    }
}
