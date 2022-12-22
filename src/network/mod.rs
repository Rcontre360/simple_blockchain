use std::{sync::Arc, thread::JoinHandle};

use anyhow::Result;
use crossbeam::channel::Receiver;
use redis::{Client as RedisClient, Commands, Connection, ControlFlow, Msg, PubSubCommands};
use rocket::serde::json::serde_json::to_string;

use crate::blockchain::block::Block;

pub static MAIN_CHANNEL: &str = "BLOCKCHAIN";

fn get_connection() -> Connection {
    let endpoint = dotenv::var("DB").unwrap();
    let client = RedisClient::open(endpoint).unwrap();
    client.get_connection().unwrap()
}

pub fn listen<T>(
    mut callback: impl FnMut(String, String, T) + Send + Sync + 'static,
    context_receiver: Receiver<T>,
) -> JoinHandle<()>
where
    T: Send + 'static,
{
    let mut con = get_connection();
    std::thread::spawn(move || {
        con.subscribe(&[MAIN_CHANNEL], |msg: Msg| {
            let channel = msg.get_channel_name().to_string();
            let payload: String = msg.get_payload().unwrap();
            let context = context_receiver.recv().unwrap();
            callback(channel, payload, context);

            ControlFlow::<()>::Continue
        })
        .unwrap();
    })
}

pub fn broadcast(message: String) -> Result<bool> {
    let mut con = get_connection();
    let res: String = con.publish(MAIN_CHANNEL, message)?;

    Ok(true)
}

pub fn broadcast_block(block: &Block) -> Result<bool> {
    let message = to_string(block)?;
    broadcast(message)
}

mod test {
    use crate::blockchain::block::*;
    use crate::storage::*;
}
