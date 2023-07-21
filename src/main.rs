mod blockchain;
use blockchain::BlockChain;
mod cli;
use cli::cli_task;
mod p2p;
// use core::num::dec2flt::number;
// use p2p::{number_of_peers, p2p_task};
use p2p::P2P;
use std::error::Error;
use std::io::{stdout, Write};
use termion::clear;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let P2P: P2P = P2P::new();
    // let (_p2p_result, _cli_result) = futures::join!(P2P.run_task(), cli_task());

    // let blockChain = initialiseBlockChain();

    // Clear the command line (Unix-based Systems) **TODO ADD WINDOWS SUPPORT**
    print!("{}", clear::All);
    stdout().flush().unwrap();

    // Run the CLI input stream and P2P network asynchronously
    let (_p2p_result, _cli_result) = futures::join!(P2P.run_task(), cli_task());

    Ok(())
}

// fn initialiseBlockChain() -> BlockChain {
//     // Check peers
//         if get_peers_count() > 1 {
//             // Request Longest Chain
//                // publish message "request chain"
//             // Create local BlockChain object based on chain data recieved
//             return BlockChain;
//         } else {
//             // If only one peer
//                  // FUTURE: check local SQL database for the BlockChain
//                  // If there is an SQL blockchain
//                     // Create new BlockChain object with only genesis block
//                  // If there is none
//                  //Create empty BlockChain object with only genesis block
//                  // FUTURE: Save to SQL
//             return BlockChain;
//         }
// }

// let mut BC: blockchain::BlockChain = blockchain::BlockChain::new();
// let mined_block: blockchain::Block = blockchain::Block::new(2, "test file".to_string(), "none".to_string(),BC.blocks[0].current_hash.to_owned());
// println!("{:?}", mined_block);
// BC.add_block(mined_block);
// println!("{:?}", BC);
// BC.chain_is_valid(&BC.blocks);
