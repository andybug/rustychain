extern crate byteorder;
extern crate crypto;
extern crate serde;

use std::fmt;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use self::byteorder::{LittleEndian, WriteBytesExt};

use blockchain::BlockHash;


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

    pub fn to_bytes(&self) -> Vec<u8> {
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
