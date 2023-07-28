/*
Todo:
1. Implement time stamping
2. Data races (nodes mined at the same time)
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
        return BlockChain { blocks: blocks };
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

    //Todo: Could this be implemented in the Block instead?
    //Checking if block is valid for the 'add_block' function based on 3 criterion
    fn block_is_valid(&self, block: &Block, last_block: &Block) -> bool {
        if block.previous_hash != last_block.current_hash {
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

    pub fn chain_is_valid(&self, chain: &Vec<Block>) -> bool {
        match chain.len() {
            0 => {
                println!("The chain is empty");
                false
            }
            1 => {
                println!("The chain only contains the genesis block");
                false
            }
            _ => {
                for i in 1..chain.len() {
                    let previous = chain.get(i - 1).unwrap();
                    let current = chain.get(i).unwrap();
                    if !self.block_is_valid(current, previous) {
                        return false;
                    }
                }
                println!("The chain is valid");
                true
            }
        }
    }

    //Formats the blockchain to be sendable in string format 
    pub fn to_sendable(&self) -> String {
        let mut blockchain_string: String = String::from("");
        for block in self.blocks.iter() {
            //The % and $ are used as split seperators later on to be able to help in create a blockchain from the blockchain_string
            let block_string = format!(
                "{}%{}%{}%{}%{}%{}$",
                block.id,
                block.nonce,
                block.file_name,
                block.file_data,
                block.previous_hash,
                block.current_hash
            );
            blockchain_string.push_str(&block_string);
        }
        blockchain_string
    }

    //Formats the blockchain to be viewable in string format
    pub fn to_viewable(&self) -> String {
        let mut blockchain_string: String = String::from("");
        for block in self.blocks.iter() {
            let block_string = format!(
                "Block Id: {}, Nonce: {}, File Name: {}, Previous Hash: {}, Block Hash: {}",
                block.id,
                block.nonce,
                block.file_name,
                block.previous_hash,
                block.current_hash
            );
            blockchain_string.push_str(&block_string);
        }
        blockchain_string
    }

    /*
    //Implementation needs to be readjusted for multi-node chain selection
    fn chain_selector(&self, local: Vec<Block>, remote: Vec<Block>) -> Option<Vec<Block>> {
        let local_is_valid: bool = self.chain_is_valid(&local);
        let remote_is_valid: bool = self.chain_is_valid(&remote);

        match (local_is_valid, remote_is_valid) {
            (true, true) => {
                if local.len() >= remote.len() {
                    println!("The local copy is valid");
                    Some(local)
                } else {
                    println!("The remote copy is valid");
                    Some(remote)
                }
            },
            (true, false) => {
                println!("The local copy is valid. Keeping local copy.");
                Some(local)
            },
            (false, true) => {
                println!("The remote copy is valid, returning remote copy");
                Some(remote)
            },
            (false, false) => {
                println!("Both chains are invalid");
                None
            }

        }
    }
    */
}

use md4::{Digest, Md4};

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
        };
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
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}
