use colored::*;

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
    pub file_type: String,
}

impl BlockChain {
    //Constructor for BlockChain structure
    pub fn new() -> BlockChain {
        // Genesis block instantiated pre-set values
        let genesis_block: Block = Block {
            id: 1,
            nonce: 12345,
            file_name: String::from("Genesis"),
            file_data: String::from("This is just the genesis block"),
            previous_hash: String::from("00000000000000000000000000000000"),
            current_hash: String::from("ae320cf7e6f1593d57730e250307fa6a"),
            file_type: String::from("txt")
        };
        // Block vector with only Genesis block instantiated
        let mut blocks: Vec<Block> = vec![];
        blocks.push(genesis_block);

        // BlockChain object with pre-set genesis is constructed
        return BlockChain { blocks: blocks };
    }

    //Parser-friendly instance of a blockchain with no blocks (no genesis even)
    // pub fn new_no_genesis() -> BlockChain {
    //     let blocks: Vec<Block> = vec![];
    //     return BlockChain { blocks: blocks }
    // }

    pub fn add_block(&mut self, block: Block) {
        match self.blocks.last() {
            None => {
                println!("\n{}", "Blockchain Error: Could not add block.".red().bold());
                println!("{}\n",
                    "Try requesting a the newest instance of the blockchain: 'request blockchain'".red().bold()
                );
            }

            Some(last_block) => {
                if self.block_is_valid(&block, last_block) {
                    let id = block.id;
                    let file_name = block.file_name.to_owned();
                    self.blocks.push(block);
                    println!("\n{} {}", "!".red().bold(), "New file added to blockchain!".truecolor(36, 200, 255));
                    println!("{} {} {} {}\n", "ID:".truecolor(36, 200, 255), id.to_string().bright_purple(), "Name:".truecolor(36, 200, 255), file_name.to_string().bright_purple());
                } else {
                    println!("{}\n", "Block is not valid, could not add to block".red().bold())
                }
            }
        }
    }

    //Todo: Could this be implemented in the Block instead?
    //Checking if block is valid for the 'add_block' function based on 3 criterion
    fn block_is_valid(&self, block: &Block, last_block: &Block) -> bool {
        if block.previous_hash != last_block.current_hash {
            println!("\n{}{}, {}, {}", "Invalid previous hash: ID:".red().bold(), block.id.to_string().red().bold(), block.previous_hash.red().bold(), last_block.current_hash.red().bold());
            false
        } else if !block.current_hash.starts_with("000") {
            println!("\n{}{}", "Invalid hash: ID".red().bold(), block.current_hash.red().bold());
            false
        } else if block.id != last_block.id + 1 {
            println!("\n{}", "Invalid ID, ID must be one more than last block's ID".red().bold());
            false
        } else {
            true
        }
    }

    // pub fn chain_is_valid(&self, chain: &Vec<Block>) -> bool {
    //     match chain.len() {
    //         0 => {
    //             println!("The chain is empty");
    //             false
    //         }
    //         1 => {
    //             println!("The chain only contains the genesis block");
    //             false
    //         }
    //         _ => {
    //             for i in 1..chain.len() {
    //                 let previous = chain.get(i - 1).unwrap();
    //                 let current = chain.get(i).unwrap();
    //                 if !self.block_is_valid(current, previous) {
    //                     return false;
    //                 }
    //             }
    //             println!("The chain is valid");
    //             true
    //         }
    //     }
    // }

    //Formats the blockchain to be sendable in string format
    pub fn to_sendable(&self) -> String {
        let mut blockchain_string: String = String::from("222");
        for block in self.blocks.iter().skip(1) {
            //The % and $ are used as split seperators later on to be able to help in create a blockchain from the blockchain_string
            let block_string = format!(
                "{}%{}%{}%{}%{}%{}%{}$",
                block.id,
                block.nonce,
                block.file_name,
                block.file_data,
                block.file_type,
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
                "\n{} {} {} {} {} {} {} {} {} {} {} {}\n",
                "Block Id: ".truecolor(36, 200, 255),
                block.id.to_string().bright_purple(),
                " Block Nonce: ".truecolor(36, 200, 255),
                block.nonce.to_string().bright_purple(),
                " File Name: ".truecolor(36, 200, 255),
                block.file_name.bright_purple(),
                " File Type: ".truecolor(36, 200, 255),
                block.file_type.bright_purple(),
                " Previous Hash: ".truecolor(36, 200, 255),
                block.previous_hash.bright_purple(),
                " Current Hash: ".truecolor(36, 200, 255),
                block.current_hash.bright_purple()
            );
            blockchain_string.push_str(&block_string);
        }
        blockchain_string
    }

    pub fn find_block(&self, id: u64) -> Option<&Block> {
        let index = self.blocks.iter().position(|x| x.id == id);
        match index {
            Some(i) => {
                Some(&self.blocks[i])
            }
            _ => {
                None
            }
        }
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

use md4::{ Digest, Md4 };

impl Block {
    pub fn new(id: u64, file_name: String, file_data: String, file_type: String, previous_hash: String,) -> Block {
        let (nonce, hash) = Block::mine(id, &previous_hash, &file_data);

        return Block {
            id,
            nonce,
            file_name,
            file_data,
            previous_hash,
            current_hash: hash,
            file_type: file_type,
        };
    }

    //Used when creating a blockchain instance for an already exisiting blockchain in parser
    pub fn new_mined_block(
        id: u64,
        nonce: u64,
        file_name: String,
        file_data: String,
        file_type: String,
        previous_hash: String,
        current_hash: String
    ) -> Block {
        return Block {
            id,
            nonce,
            file_name,
            file_data,
            file_type,
            previous_hash,
            current_hash,
        };
    }

    pub fn to_sendable(&self) -> String {
        //The % and $ are used as split seperators later on to be able to help in create a blockchain from the blockchain_string
        let block_string = format!(
            "111{}%{}%{}%{}%{}%{}%{}",
            self.id,
            self.nonce,
            self.file_name,
            self.file_data,
            self.file_type,
            self.previous_hash,
            self.current_hash
        );
        block_string
    }

    pub fn mine(id: u64, previous_hash: &str, file_data: &str) -> (u64, String) {
        println!("\n{}", "Mining...".bright_cyan().italic());
        let mut nonce: u64 = 1;
        loop {
            let block_string: String = format!("{}{}{}{}", nonce, id, previous_hash, file_data);
            let mut hasher = Md4::new();
            hasher.update(block_string);
            let result = &hasher.finalize();
            let hash = hex::encode(result);
            if hash.starts_with("000") {
                println!("{}{}{}{}", "Mined! Nonce: ".truecolor(36, 200, 255), nonce.to_string().bright_purple(), " Hash: ".truecolor(36, 200, 255), hash.bright_purple());
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}
