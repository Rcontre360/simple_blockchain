#[macro_use]
extern crate rocket;

use anyhow::Result;
use full_blockchain::{blockchain::block::Block, storage::Client};

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![])
}
