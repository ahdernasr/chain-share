/*
Todo: 
1. Implement time stamping
*/

#![allow(dead_code)]

// Simple BlockChain structure, defined as a vector of Block structures
#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

// Block structure, defines all the properties of a block in this this blockchain
#[derive(Debug)]
pub struct Block { 
    pub id: u64,
    pub nonce: u64,
    pub file_name: String,
    pub file_data: String,
    pub previous_hash: String,
    pub current_hash: String,
}

impl BlockChain {
    //Constructor for BlockChain structure
    pub fn new() -> BlockChain {
        // Genesis block instantiated pre-set values
        let genesis_block: Block = Block {
            id: 1,
            nonce: 12345,
            file_name: String::from("No File"),
            file_data: String::from(" "),
            previous_hash: String::from("00000000000000000000000000000000"),
            current_hash: String::from("ae320cf7e6f1593d57730e250307fa6a"),
        };
        // Block vector with only Genesis block instantiated
        let mut blocks: Vec<Block> = vec![];
        blocks.push(genesis_block);

        // BlockChain object with pre-set genesis is constructed
        return BlockChain {
            blocks: blocks,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        match self.blocks.last() {
            None => {
                println!("Blockchain Error: Could not add block.")
            }

            Some(last_block) => {
                if self.block_is_valid(&block, last_block) {
                    self.blocks.push(block);
                    println!("Block added to blockchain!")
                } else {
                    println!("Block is not valid, could not add to block")
                }
            }
        }
    } 

    fn block_is_valid(&self, block: &Block, last_block: &Block) -> bool {
        if (block.previous_hash != last_block.current_hash) {
            println!("Invalid previous hash: ID{}", block.id);
            false
        } else if !block.current_hash.starts_with("000") {
            println!("Invalid hash: ID{}", block.id);
            false
        } else if block.id != last_block.id + 1 {
            println!("Invalid ID, ID must be one more than last block's ID");
            false
        } else {
            true
        }
    }
}

use md4::{Md4, Digest};

impl Block {
    pub fn new(id: u64, file_name: String, file_data: String, previous_hash: String) -> Block {
        let (nonce, hash) = Block::mine(id, &previous_hash, &file_data);

        return Block {
            id,
            nonce,
            file_name,
            file_data,
            previous_hash,
            current_hash: hash,
        }
    }

    pub fn mine(id: u64, previous_hash: &str, file_data: &str) -> (u64, String) {
        println!("Mining...");
        let mut nonce: u64 = 1;
        loop {
            let block_string: String = format!("{}{}{}{}", nonce, id, previous_hash, file_data);
            let mut hasher = Md4::new();
            hasher.update(block_string);
            let result = &hasher.finalize();
            let hash = hex::encode(result);
            if hash.starts_with("000") {
                println!("Mined! Nonce: {}, hash {}", nonce, hash);
                return (nonce, hash)
            }
            nonce += 1;
        }
    }
}
