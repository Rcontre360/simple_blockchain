#[macro_use]
extern crate rocket;

use anyhow::Result;
use full_blockchain::{blockchain::block::Block, storage::Client};

//#[launch]
//pub fn rocket() -> _ {
//rocket::build().mount("/", routes![get_block])
//}

fn main() -> Result<()> {
    let mut db = Client::new("redis://127.0.0.1:6379")?;
    let block = Block::default();

    db.save_block(&block)?;
    let block = db.get_block(block.get_block_number(), &block.get_hash())?;

    println!("block: {:?}", block);

    Ok(())
}
