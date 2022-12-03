use bytes::Bytes;
use rocket::get;
use rocket::serde::json::Json;

use crate::blockchain::block::Block;

#[get("/block/<id>")]
pub fn get_block(id: usize) -> Json<Block> {
    Json(Block {
        timestamp: 1,
        data: Bytes::from_static(b"test"),
        hash: Bytes::from_static(b"test"),
        prev_hash: Bytes::from_static(b"test"),
        difficulty: 4,
        nonce: 4,
    })
}
