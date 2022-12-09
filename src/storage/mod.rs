use anyhow::Result;
use redis::{from_redis_value, Client as RedisClient, Connection, JsonCommands};
use rocket::serde::json::from_str;

use crate::blockchain::block::Block;

pub struct Client {
    connection: &'static str,
    pub connection_instance: Connection,
}

impl Client {
    fn block_key(number: u32, hash: &[u8; 32]) -> Vec<u8> {
        let mut prefix = b"block::".to_vec();
        let mut vec_hash = hash.to_vec();
        let mut vec_number = number.to_be_bytes().to_vec();

        prefix.append(&mut vec_number);
        prefix.append(&mut b"::".to_vec());
        prefix.append(&mut vec_hash);

        prefix
    }

    pub fn new(connection: &'static str) -> Result<Client> {
        let client = RedisClient::open(connection)?;
        let connection_instance = client.get_connection()?;

        Ok(Client {
            connection,
            connection_instance,
        })
    }

    pub fn save_block(&mut self, block: &Block) -> Result<bool> {
        self.connection_instance.json_set(
            Client::block_key(block.get_block_number(), &block.get_hash()),
            ".",
            block,
        )?;
        Ok(true)
    }

    pub fn get_block(&mut self, block_number: u32, block_hash: &[u8; 32]) -> Result<Block> {
        let res = self
            .connection_instance
            .json_get(Client::block_key(block_number, block_hash), ".")?;

        let test = from_redis_value::<String>(&res)?;
        let block: Block = from_str(&test)?;

        Ok(block)
    }
}
