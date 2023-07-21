mod blockchain;
use blockchain::BlockChain;
mod cli;
use cli::cli_task;
mod p2p;
// use core::num::dec2flt::number;
// use p2p::{number_of_peers, p2p_task};
use p2p::P2P;
use std::sync::Arc;
use std::sync::Mutex;
use std::error::Error;
use std::io::{stdout, Write};
use termion::clear;

// Auto-join publishing solution: tell user to manually request blockchain,
// and dissallow any blockchain commands until the user requests the blockchain
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let P2P: P2P = P2P::new();
    let swarm_instance = &P2P.swarm;
    // let (_p2p_result, _cli_result) = futures::join!(P2P.run_task(), cli_task());

    // let blockChain = initialiseBlockChain();

    // Clear the command line (Unix-based Systems) **TODO ADD WINDOWS SUPPORT**
    print!("{}", clear::All);
    stdout().flush().unwrap();

    // Run the CLI input stream and P2P network asynchronously
    let (_p2p_result, _cli_result) = futures::join!(P2P.run_task(), cli_task());

    Ok(())
}

// pub fn publish_message(swarm: , message: String) {
//     let topic = gossipsub::IdentTopic::new("test-net");
//     if let Err(e) = self.swarm
//         .behaviour_mut()
//         .gossipsub
//         .publish(topic.clone(), message.as_bytes())
//     {
//         println!("Publish error: {e:?}");
//     }
// }

// #[async_std::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Create a new P2P instance wrapped in an Arc and a Mutex
//     let p2p: Arc<Mutex<P2P>> = Arc::new(Mutex::new(P2P::new()));

//     // Clear the command line (Unix-based Systems) **TODO ADD WINDOWS SUPPORT**
//     print!("{}", clear::All);
//     stdout().flush().unwrap();

//     // Clone the Arc to be moved into each task
//     let p2p_task = p2p.clone();
//     let cli_task = cli_task();

//     // Run the P2P and CLI tasks asynchronously
//     let (p2p_result, cli_result) = futures::join!(run_p2p_task(p2p_task), cli_task);


//     // Handle results or errors if necessary
//     // ...

//     Ok(())
// }

// // Define a new function to run the P2P task with access to the shared P2P instance
// async fn run_p2p_task(p2p: Arc<Mutex<P2P>>) {
//     let p2p = p2p.lock(); // Acquire the lock on P2P
//     p2p.unwrap().run_task(); // Access and run the P2P task
// }


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
