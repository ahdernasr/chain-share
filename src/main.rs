mod args;
mod blockchain;
use args::CLIArgs;
use args::handle_args;
use clap::Parser;
use std::io::{stdout, Write};
use termion::clear;
fn main() {

    //TODO, add Windows support
    // Clear the command line (Unix-based Systems)
    print!("{}", clear::All);
    stdout().flush().unwrap();

    //TODO, add COLOR and ASCI-ART 
    //CLI begins here
    let args = CLIArgs::parse();
    handle_args(args)

}

// let mut BC: blockchain::BlockChain = blockchain::BlockChain::new();
// let mined_block: blockchain::Block = blockchain::Block::new(2, "test file".to_string(), "none".to_string(),BC.blocks[0].current_hash.to_owned());
// println!("{:?}", mined_block);
// BC.add_block(mined_block);
// println!("{:?}", BC);
// BC.chain_is_valid(&BC.blocks);
