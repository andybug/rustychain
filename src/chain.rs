use std::fmt;
//use std::collections::HashMap;
use std::collections::LinkedList;

use block::{Block, BlockHash};


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
                block.set_prev(blockhash);
            },
            None => {},
        }
        println!("{}", block);
        self.chain.push_back(block);
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
