#[macro_use]
extern crate rocket;

use full_blockchain::server::block::*;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![get_block])
}
