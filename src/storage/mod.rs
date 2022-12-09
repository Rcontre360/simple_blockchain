use anyhow::Result;
use redis::{from_redis_value, Client as RedisClient, Connection, JsonCommands, Value};
use rocket::serde::json::from_str;

use crate::blockchain::block::Block;

pub static DB_ENDPOINT: &str = "redis://127.0.0.1:6379";

pub struct Client {
    connection: &'static str,
    pub connection_instance: Connection,
}

impl Client {
    fn block_key(hash: &[u8; 32]) -> Vec<u8> {
        let mut prefix = b"block::".to_vec();
        let mut vec_hash = hash.to_vec();
        prefix.append(&mut vec_hash);

        prefix
    }

    fn hash_key(number: u32) -> Vec<u8> {
        let mut prefix = b"block_hash::".to_vec();
        let mut vec_number = number.to_be_bytes().to_vec();
        prefix.append(&mut vec_number);

        prefix
    }

    fn get_data(&mut self, key: &Vec<u8>) -> Result<String> {
        let res = self.connection_instance.json_get(key, ".")?;
        Ok(from_redis_value::<String>(&res)?)
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
        let block_key = Client::block_key(&block.get_hash());
        let hash_key = Client::hash_key(block.get_block_number());

        self.connection_instance.json_set(block_key, ".", block)?;
        self.connection_instance
            .json_set(hash_key, ".", &block.get_hash())?;

        Ok(true)
    }

    pub fn get_block_by_hash(&mut self, block_hash: &[u8; 32]) -> Result<Block> {
        let hash_key = &Client::block_key(block_hash);
        let json = self.get_data(hash_key)?;
        let block: Block = from_str(&json)?;

        Ok(block)
    }

    pub fn get_block_by_number(&mut self, block_number: u32) -> Result<Block> {
        let num_key = &block_number.to_be_bytes().to_vec();
        let hash = self.get_data(num_key)?;
        let block_string = self.get_data(&hash.as_bytes().to_vec())?;
        let block: Block = from_str(&block_string)?;

        Ok(block)
    }

    pub fn delete_block(&mut self, block_hash: &[u8; 32]) -> Result<bool> {
        Ok(true)
    }
}

mod test {
    use crate::blockchain::block::*;
    use crate::storage::*;

    #[test]
    #[allow(dead_code)]
    fn save_and_get_block() -> Result<()> {
        let mut db = Client::new(DB_ENDPOINT)?;
        let block = Block::default();

        db.save_block(&block)?;

        let block_by_hash = db.get_block_by_hash(&block.get_hash())?;
        let block_by_number = db.get_block_by_number(block.get_block_number())?;
        println!("BLOCK by number");

        assert!(block == block_by_hash, "block by hash is not equal");
        assert!(block == block_by_number, "block by number is not equal");

        Ok(())
    }
}
