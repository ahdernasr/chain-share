mod blockchain;
mod cli;
use cli::cli_task;
mod p2p;
use p2p::p2p_task;
use std::io::{stdout, Write};
use std::error::Error;
use termion::clear;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Clear the command line (Unix-based Systems) **TODO ADD WINDOWS SUPPORT**
    print!("{}", clear::All);
    stdout().flush().unwrap();

    // Run the CLI input stream and P2P network asynchronously
    let (_p2p_result, _cli_result) = futures::join!(p2p_task(), cli_task());

    Ok(())
}

// let mut BC: blockchain::BlockChain = blockchain::BlockChain::new();
// let mined_block: blockchain::Block = blockchain::Block::new(2, "test file".to_string(), "none".to_string(),BC.blocks[0].current_hash.to_owned());
// println!("{:?}", mined_block);
// BC.add_block(mined_block);
// println!("{:?}", BC);
// BC.chain_is_valid(&BC.blocks);