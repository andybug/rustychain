extern crate serde_yaml;

use std::fmt;
use std::fs;
use std::io::Write;
use std::path::Path;
//use std::collections::HashMap;
use std::collections::LinkedList;

use blockchain::{Block, BlockHash};


pub struct BlockChain {
    //block_map: HashMap<BlockHash, Block>,
    chain: LinkedList<Box<Block>>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        BlockChain {
            //block_map: HashMap::new(),
            chain: LinkedList::new(),
        }
    }

    pub fn append(&mut self, mut block: Box<Block>) {
        match self.chain.back() {
            Some(tail) => {
                let blockhash = BlockHash::hash(&tail);
                block.set_previous(blockhash);
            },
            None => {},
        }
        self.chain.push_back(block);
    }

    fn create_chain_dir(dir: &Path) {
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

    pub fn write_chain(&mut self, dir: &Path) {
        BlockChain::create_chain_dir(dir);
        let block_file = dir.join("blocks.yaml");

        let mut f = match fs::File::create(block_file.as_path()) {
            Ok(f) => f,
            Err(e) => panic!("open file error: {}", e),
        };

        for ref block in self.chain.iter() {
            let serialized = serde_yaml::to_string(block).unwrap();
            //f.write(serialized.as_bytes()).unwrap();
            write!(f, "{}\n", serialized).unwrap();
        }
    }
}

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, ref block) in self.chain.iter().enumerate() {
            let blockhash = BlockHash::hash(&block);
            write!(f, "{:08}: {}\n", i, blockhash).unwrap();
        }

        Ok(())
    }
}
