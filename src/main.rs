use bytes::Bytes;
use full_blockchain::block::*;
use full_blockchain::chain::*;

fn main() {
    let mut chain: Vec<Block> = vec![Block::default()];
    let timestamp = 5;
    let data = Bytes::from_static(b"some data");

    let (nxt_block_hash, nonce) = generate_next_block_hash(&chain, timestamp, &data);
    println!("nxt_block_hash: {:2x?}", nxt_block_hash.as_ref());
}
