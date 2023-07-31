use crate::p2p::blockchain::{ Block, BlockChain };
use std::{fs, io::Write, path::Path};

// Takes in a path and reads the file to convert to string so its sendeable across the blockchain
pub fn file_to_string(path: &str) -> Option<String> {
    // Replace "your_absolute_file_path" with the actual absolute path of the file you want to read.
    let file_path = Path::new(path);

    // Attempt to read the contents of the file into a string.
    match fs::read_to_string(file_path) {
        Ok(contents) => { Some(contents) }
        Err(err) => {
            println!("{:?}", err);
            None
        }
    }
}

pub fn string_to_file(data: String, name: String,  file_type: String, path: String) {
    let full_path: String = format!("{}{}.{}", path, name, file_type);
    let mut file = fs::File::create(full_path).expect("Failed to create file");
    let result = file.write_all(data.as_bytes());
    match result {
        Ok(_) => { println!("File successfully downloaded.")}
        Err(_) => { println!("File could not be downloaded, please try again.")}
    }
}

// Takes in the blockchain string in its sendable format and creates a blockchain object with it
pub fn block_parser(data: String) -> Block {
    /*
    Block is setup in sendable format as such in blockchain.rs
            let block_string = format!(
            "111{}%{}%{}%{}%{}%{}$",
            self.id,
            self.nonce,
            self.file_name,
            self.file_data,
            self.previous_hash,
            self.current_hash
            );
    */
    let no_prefix_data: String = data[3..].to_string();
    let block_data: Vec<&str> = no_prefix_data.split('%').collect::<Vec<&str>>();
    let block: Block = Block::new_mined_block(
        block_data[0].parse::<u64>().unwrap(),
        block_data[1].parse::<u64>().unwrap(),
        block_data[2].to_string(),
        block_data[3].to_string(),
        block_data[4].to_string(),
        block_data[5].to_string(),
        block_data[6].to_string(),
    );
    block
}
pub fn blockchain_parser(data: String) -> BlockChain {
    /*
        Blockchain is setup in sendable format as such in blockchain.rs:
        Vec<String> of:
            let block_string = format!(
            "222{}%{}%{}%{}%{}%{}$",
            block.id,
            block.nonce,
            block.file_name,
            block.file_data,
            block.previous_hash,
            block.current_hash
            );
    */
    let no_prefix_data: String = data[3..].to_string();
    let blocks: Vec<&str> = no_prefix_data.split('$').collect::<Vec<&str>>();
    let mut attr_vec: Vec<Vec<&str>> = vec![];
    for attr in blocks {
        attr_vec.push(attr.split('%').collect::<Vec<&str>>());
    }
    let mut bc: BlockChain = BlockChain::new();
    for block in attr_vec {
        if block.len() > 1 {
            let temporary_block: Block = Block::new_mined_block(
                block[0].parse::<u64>().unwrap(),
                block[1].parse::<u64>().unwrap(),
                block[2].to_string(),
                block[3].to_string(),
                block[4].to_string(),
                block[5].to_string(),
                block[6].to_string()
            );
            bc.add_block(temporary_block);
        }
    }
    bc
}
