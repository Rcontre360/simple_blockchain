#[macro_use]
extern crate rocket;
use full_blockchain::server::block::{get_block_by_hash, get_block_by_number, mine_block};

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![get_block_by_hash, get_block_by_number, mine_block],
    )
}
