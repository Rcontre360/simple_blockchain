use crossbeam::channel::{bounded, unbounded, Receiver};

use full_blockchain::{
    blockchain::block::Block,
    blockchain::chain::Chain,
    network::{listen, MAIN_CHANNEL},
    server::block::{get_block_by_hash, get_block_by_number, mine_block},
};
use rocket::{launch, routes, serde::json::from_str};

fn start_mining() {}

#[launch]
pub fn rocket() -> _ {
    let chain = Chain::new();
    let (send_sync, receive_sync) = bounded::<bool>(1);
    let sync_handler = chain.sync(send_sync).unwrap();

    listen(
        |channel, message, is_synced| {
            if channel == MAIN_CHANNEL && is_synced {
                let block: Block = from_str(&message).unwrap();
                println!("ADD BLOCK TO SYNCED CHAIN: {:?}", block);
            }
        },
        receive_sync,
    );

    let rocket_res = rocket::build().mount(
        "/",
        routes![get_block_by_hash, get_block_by_number, mine_block],
    );

    sync_handler.join();

    rocket_res
}
