mod p2p;
use colored::*;
use p2p::P2P as p_2_p;
use std::error::Error;
use std::io::{ stdout, Write };

// Auto-join publishing solution: tell user to manually request blockchain,
// and dissallow any blockchain commands until the user requests the blockchain
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Clear the command line (Unix-based Systems) **TODO ADD WINDOWS SUPPORT**
    // print!("{}", clear::All);
    stdout().flush().unwrap();

    println!("{}", "Welcome to ChainShare".red().bold());

    // Run the Blockchain Builder and P2P network asynchronously
    let mut p2p: p_2_p = p_2_p::new();
    let _ = p2p.run_task().await;
    //NOTHING UNDER HERE WILL RUN

    // let (_, __) = futures::join!(p2p.run_task(), blockchain_task());
    // let blockChain = initialiseBlockChain();

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
