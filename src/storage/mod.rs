use anyhow::Result;
use hex::encode;
use redis::{from_redis_value, Client as RedisClient, Connection, JsonCommands, Value};
use rocket::serde::json::from_str;

use crate::blockchain::block::{Block, BlockHash};

pub static DB_ENDPOINT: &str = "redis://127.0.0.1:6379";

pub struct Client {
    pub connection_instance: Connection,
}

impl Client {
    fn block_key(hash: &BlockHash) -> String {
        let mut prefix = String::from("block::0x");
        let vec_hash = &encode(hash);

        prefix.push_str(vec_hash);
        prefix
    }

    fn hash_key(number: u32) -> String {
        let mut prefix = String::from("block_hash::0x");
        let vec_number = &encode(number.to_be_bytes());

        prefix.push_str(vec_number);
        prefix
    }

    fn get_data(&mut self, key: &String) -> Result<String> {
        let res: Value = self.connection_instance.json_get(key, ".")?;
        let str_value: String = from_redis_value(&res)?;
        Ok(str_value)
    }

    pub fn new() -> Result<Client> {
        let client = RedisClient::open(DB_ENDPOINT)?;
        let connection_instance = client.get_connection()?;

        Ok(Client {
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

    pub fn get_block_by_hash(&mut self, block_hash: &BlockHash) -> Result<Block> {
        let hash_key = &Client::block_key(block_hash);
        let raw_block = self.get_data(hash_key)?;
        let block: Block = from_str(&raw_block)?;

        Ok(block)
    }

    pub fn get_block_by_number(&mut self, block_number: u32) -> Result<Block> {
        let num_key = &Client::hash_key(block_number);
        let raw_hash = self.get_data(num_key)?;
        let hash = from_str(&raw_hash)?;

        let hash_key = Client::block_key(&hash);
        let raw_block = self.get_data(&hash_key)?;
        let block: Block = from_str(&raw_block)?;

        Ok(block)
    }

    pub fn delete_block(&mut self, block_hash: &BlockHash) -> Result<bool> {
        let block = self.get_block_by_hash(block_hash)?;
        let block_key = Client::block_key(&block.get_hash());
        let hash_key = Client::hash_key(block.get_block_number());

        self.connection_instance.json_del(block_key, ".")?;
        self.connection_instance.json_del(hash_key, ".")?;
        Ok(true)
    }
}

mod test {
    use crate::blockchain::block::*;
    use crate::storage::*;

    #[allow(dead_code)]
    fn remove(client: &mut Client, blocks: Vec<&Block>) -> Result<()> {
        for block in blocks.iter() {
            client.delete_block(&block.get_hash())?;
        }

        Ok(())
    }

    #[test]
    #[allow(dead_code)]
    fn save_and_get_block() -> Result<()> {
        let mut db = Client::new()?;
        let block = Block::default();

        db.save_block(&block)?;

        let block_by_hash = db.get_block_by_hash(&block.get_hash())?;
        let block_by_number = db.get_block_by_number(block.get_block_number())?;

        assert!(block == block_by_hash, "block by hash is not equal");
        assert!(block == block_by_number, "block by number is not equal");

        remove(&mut db, [&block].to_vec())?;

        Ok(())
    }
}
