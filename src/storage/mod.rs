use anyhow::Result;
use hex::{decode, encode};
use redis::{from_redis_value, Client as RedisClient, Connection, JsonCommands, Value};
use rocket::serde::json::from_str;

use crate::blockchain::block::{Block, BlockHash};

pub static DB_ENDPOINT: &str = "redis://127.0.0.1:6379";
pub static COUNT_KEY: &str = "block_count";

fn get_count_key() -> String {
    COUNT_KEY.to_string()
}

pub struct Client {
    pub connection_instance: Connection,
    node_id: String,
}

impl Client {
    pub fn new(node_id: String) -> Result<Client> {
        let client = RedisClient::open(DB_ENDPOINT)?;
        let connection_instance = client.get_connection()?;

        Ok(Client {
            connection_instance,
            node_id,
        })
    }

    fn get_node_id(&self) -> String {
        self.node_id
    }

    fn block_key(hash: &BlockHash) -> Result<String> {
        let mut prefix = get_node_id();
        let vec_hash = &encode(hash);

        prefix.push_str("::block::0x");
        prefix.push_str(vec_hash);
        Ok(prefix)
    }

    fn hash_key(number: usize) -> Result<String> {
        let mut prefix = get_node_id();
        let vec_number = &encode(number.to_be_bytes());

        prefix.push_str("::bock_hash::0x");
        prefix.push_str(vec_number);
        Ok(prefix)
    }

    fn get_data(&mut self, key: &String) -> Result<String> {
        let res: Value = self.connection_instance.json_get(key, ".")?;
        let str_value: String = from_redis_value(&res)?;
        Ok(str_value)
    }

    pub fn get_block_count(&mut self) -> usize {
        let count_str = Client::get_data(self, &get_count_key()).unwrap_or("0".to_string());
        count_str.parse().unwrap_or(0)
    }

    pub fn save_block(&mut self, block: &Block) -> Result<bool> {
        let block_key = Client::block_key(&block.get_hash())?;
        let hash_key = Client::hash_key(block.get_block_number())?;

        self.connection_instance
            .json_set(COUNT_KEY, ".", &block.get_block_number())?;
        self.connection_instance.json_set(block_key, ".", block)?;
        self.connection_instance
            .json_set(hash_key, ".", &block.get_hash())?;

        Ok(true)
    }

    pub fn get_block_by_str(&mut self, block_hash: &String) -> Result<Block> {
        let hash = decode(block_hash).unwrap();
        self.get_block_by_vec(&hash)
    }

    pub fn get_block_by_vec(&mut self, block_hash: &Vec<u8>) -> Result<Block> {
        let hash: &[u8; 32] = &block_hash[..32].try_into().unwrap();
        self.get_block_by_hash(hash)
    }

    pub fn get_block_by_hash(&mut self, block_hash: &BlockHash) -> Result<Block> {
        let hash_key = &Client::block_key(block_hash)?;
        let raw_block = self.get_data(hash_key)?;
        let block: Block = from_str(&raw_block)?;

        Ok(block)
    }

    pub fn get_block_by_number(&mut self, block_number: usize) -> Result<Block> {
        let num_key = &Client::hash_key(block_number)?;
        let raw_hash = self.get_data(num_key)?;
        let hash = from_str(&raw_hash)?;

        let hash_key = Client::block_key(&hash)?;
        let raw_block = self.get_data(&hash_key)?;
        let block: Block = from_str(&raw_block)?;

        Ok(block)
    }

    pub fn get_last_block(&mut self) -> Result<Block> {
        let last_block_number = self.get_block_count();
        self.get_block_by_number(last_block_number)
    }

    pub fn delete_block(&mut self, block_hash: &BlockHash) -> Result<bool> {
        let block = self.get_block_by_hash(block_hash)?;
        let block_key = Client::block_key(&block.get_hash())?;
        let hash_key = Client::hash_key(block.get_block_number())?;

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
