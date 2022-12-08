use anyhow::Result;
use bytes::Bytes;
use redis::{Client as RedisClient, Connection, JsonCommands};

use crate::blockchain::block::Block;

pub struct Client {
    connection: &'static str,
    pub connection_instance: Connection,
}

impl Client {
    pub fn new(connection: &'static str) -> Result<Client> {
        let mut client = RedisClient::open(connection)?;
        let connection_instance = client.get_connection()?;

        Ok(Client {
            connection,
            connection_instance,
        })
    }

    pub fn save_block(&mut self, block: &Block) -> Result<()> {
        self.connection_instance
            .json_set(block.get_hash().to_vec(), ".", block)?;
        Ok(())
    }

    pub fn get_block(&mut self, block_hash: Bytes) -> Result<()> {
        let res = self
            .connection_instance
            .json_get(block_hash.to_vec(), ".")?;

        Ok(())
    }
}
