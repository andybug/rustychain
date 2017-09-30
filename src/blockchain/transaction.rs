extern crate byteorder;

use std::io::Write;

use self::byteorder::{LittleEndian, WriteBytesExt};

use util::hash::{Hash256, HASH256_BYTES};
use util::hex::{FromHex, ToHex};


pub struct OutPoint {
    hash: [u8; HASH256_BYTES],
    index: u32,
}

impl OutPoint {
    fn hash(&self, hash: &mut Hash256) {
        hash.write_all(&self.hash);
        hash.write_u32::<LittleEndian>(self.index).unwrap();
    }
}

pub struct TransactionInput {
    previous_out: OutPoint,
}

impl TransactionInput {
    fn hash(&self, hash: &mut Hash256) {
        self.previous_out.hash(hash);
    }
}

pub struct TransactionOutput {
    amount: u64,
}

impl TransactionOutput {
    fn hash(&self, hash: &mut Hash256) {
        hash.write_u64::<LittleEndian>(self.amount).unwrap();
    }
}

pub struct Transaction {
    version: u32,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            version: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn get_hash(&self, mut buf: &mut [u8]) {
        let mut hash = Hash256::new();
        hash.write_u32::<LittleEndian>(self.version).unwrap();

        for i in &self.inputs {
            i.hash(&mut hash);
        }

        for o in &self.outputs {
            o.hash(&mut hash);
        }

        hash.finalize(&mut buf);
    }
}
