use bytes::Bytes;
use rocket::get;
use rocket::serde::json::Json;

use crate::blockchain::block::Block;

#[get("/block/<id>")]
pub fn get_block(id: usize) -> Json<Block> {
    Json(Block {
        timestamp: 1,
        block_number: 0,
        data: b"test".to_vec(),
        hash: Block::block_hash(&b"test".to_vec()),
        prev_hash: Block::block_hash(&b"test".to_vec()),
        difficulty: 4,
        nonce: 4,
    })
}
