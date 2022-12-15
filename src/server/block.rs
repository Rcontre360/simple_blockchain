use anyhow::Error;
use hex::decode;
use rocket::get;
use rocket::serde::json::Json;

use crate::blockchain::block::{Block, BlockHash};
use crate::storage::Client;

type Result<T, E = rocket::response::Debug<Error>> = std::result::Result<T, E>;

#[get("/block/number/<block_number>")]
pub fn get_block_by_number(block_number: u32) -> Result<Json<Block>> {
    let mut client = Client::new()?;
    let block = client.get_block_by_number(block_number)?;

    Ok(Json(block))
}

#[get("/block/hash/<block_hash>")]
pub fn get_block_by_hash(block_hash: String) -> Result<Json<Block>> {
    let mut client = Client::new()?;
    let mut final_hash = block_hash.clone();

    if (block_hash.chars().nth(1).unwrap() == 'x') {
        let parts: Vec<&str> = block_hash.split('x').collect();
        final_hash = String::from(parts[1]).to_lowercase();
    }
    let block = client.get_block_by_str(&final_hash)?;

    Ok(Json(block))
}

#[get("/mine")]
pub fn mine_block() -> Result<Json<Block>> {
    let mut db = Client::new()?;
    let block = Block::default();

    db.save_block(&block)?;

    Ok(Json(block))
}
