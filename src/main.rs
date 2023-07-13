mod blockchain;

fn main() {
    let blockchain: blockchain::BlockChain = blockchain::BlockChain::new();
    println!("{:?}", blockchain);
    let mined_block: blockchain::Block = blockchain::Block::new(2, "test file".to_string(), "none".to_string(),blockchain.blocks[0].current_hash.to_owned());
    println!("{:?}", mined_block);
}
