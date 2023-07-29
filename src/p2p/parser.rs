use crate::p2p::blockchain::BlockChain;
use crate::p2p::blockchain::Block;

//takes in a path and reads the file to convert to string so 
//its sendeable across the blockchain
// pub fn file_to_string(path: &str) -> String {
    
// }

// pub fn string_to_file(data: String, path: &str) {
//     //Should download a file to path
// }

// Takes in the blockchain string in its sendable format and creates a blockchain object with it
pub fn blockchain_parser(data: String) -> BlockChain {
    /*
        Blockchain is setup in sendable format as such in blockchain.rs:
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
    };
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
        );
        bc.add_block(temporary_block)
    }
    };
    bc
}

// pub fn blockchain_parser(data: String) {
//     /*
//         Blockchain is setup in sendable format as such in blockchain.rs:
//             let block_string = format!(
//             "222{}%{}%{}%{}%{}%{}$",
//             block.id,
//             block.nonce,
//             block.file_name,
//             block.file_data,
//             block.previous_hash,
//             block.current_hash
//             );
//     */
//     let no_prefix_data: String = data[3..].to_string();
//     let blocks: Vec<&str> = no_prefix_data.split('$').collect::<Vec<&str>>();
//     let mut attr_vec: Vec<Vec<&str>> = vec![];
//     for attr in blocks {
//         attr_vec.push(attr.split('%').collect::<Vec<&str>>());
//     };
//     for block in attr_vec {
//         if block.len() > 1 {
//             println!("{:?}", block);
//             println!("{:?}, {:?}", block[0].parse::<u64>().unwrap(), block[1].parse::<u64>().unwrap())
//         }
//     };
// }