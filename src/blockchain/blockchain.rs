extern crate serde_yaml;

use std::fmt;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
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

    fn append_checked(&mut self, block: Box<Block>) {
        self.chain.push_back(block);
    }

    pub fn write_chain(&self, dir: &Path) {
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

    pub fn read_chain(&mut self, block_dir: &Path) {
        let mut files: Vec<_> = fs::read_dir(block_dir).unwrap()
            .map(|r| r.unwrap())
            .collect();
        files.sort_by_key(|dir| dir.path());

        for file in files {
            let f = File::open(file.path()).unwrap();
            let mut reader = BufReader::new(f);
            let mut contents = String::new();
            reader.read_to_string(&mut contents).unwrap();

            let yaml_blocks: Vec<&str> = contents.split("---").collect();
            for yaml_block in &yaml_blocks[1..] {
                let block: Box<Block> = Box::new(serde_yaml::from_str(&yaml_block).unwrap());
                println!("{}", block);
                self.append_checked(block);
            }
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
