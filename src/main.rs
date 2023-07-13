mod blockchain;

fn main() {
    let mut BC: blockchain::BlockChain = blockchain::BlockChain::new();
    let mined_block: blockchain::Block = blockchain::Block::new(2, "test file".to_string(), "none".to_string(),BC.blocks[0].current_hash.to_owned());
    println!("{:?}", mined_block);
    BC.add_block(mined_block);
    println!("{:?}", BC);
    BC.chain_is_valid(&BC.blocks);
}
