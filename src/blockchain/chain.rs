use crate::blockchain::block::{Block, BlockHash};
use crate::storage::Client;
use anyhow::Result;
use core::panic;
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};

pub const BLOCK_TIME: u32 = 1000 * 5; // 5 seconds
pub const SYNC_NODE_ID: usize = 0;

pub struct Chain {
    client: Client,
    pub hashes: Vec<BlockHash>,
    pub synced: bool,
}

impl Chain {
    fn sync_chain(mut chain: Chain) -> Result<JoinHandle<Chain>> {
        let last_block = chain.client.get_last_block()?;
        let last_block_number = last_block.get_block_number();

        let join_handle = std::thread::spawn(move || {
            let mut cur_block_number = 0;
            let node_id = dotenv::var("NODE_ID").unwrap();

            if node_id == "0" {
                return chain;
            }

            loop {
                let nxt_block = chain
                    .client
                    .get_block_by_number(cur_block_number)
                    .unwrap_or(Block::default());

                let is_valid = chain.add_validate_block(&nxt_block).unwrap();

                if nxt_block.get_block_number() == last_block_number && is_valid {
                    break;
                } else if is_valid {
                    cur_block_number += 1;
                } else {
                    panic!("INVALID CHAIN");
                }

                chain.client.set_node_id(node_id.clone());
                chain.client.save_block(&nxt_block).unwrap();
                chain.client.set_node_id(SYNC_NODE_ID.to_string());
            }

            chain.set_synced(true);

            chain
        });

        Ok(join_handle)
    }

    pub fn new() -> Chain {
        Chain {
            client: Client::new(SYNC_NODE_ID.to_string()).unwrap(),
            hashes: vec![],
            synced: false,
        }
    }

    pub fn sync(mut self) -> Result<JoinHandle<Chain>> {
        let sync_handler = Chain::sync_chain(self)?;

        Ok(sync_handler)
    }

    pub fn add_validate_block(&mut self, block: &Block) -> Result<bool> {
        let timestamp = block.get_timestamp();
        let data = block.get_data();
        let difficulty = block.get_difficulty();
        let nonce = block.get_nonce();
        let last_block = self.get_last_block()?;

        let block_hash_data = [
            &timestamp.to_be_bytes(),
            &data[..],
            &last_block.get_hash()[..],
            &difficulty.to_be_bytes(),
            &nonce.to_be_bytes(),
        ]
        .concat();

        let hash = Block::block_hash(&block_hash_data);

        if hash != block.get_hash() {
            //ERROR
            return Ok(false);
        }

        self.hashes.push(hash);
        Ok(true)
    }

    pub fn mine_block(&mut self, data: Vec<u8>, hash: BlockHash, nonce: u32) -> Result<bool> {
        let start = SystemTime::now();

        if self.hashes.is_empty() {
            return Ok(false);
        }

        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let nxt_block = self.create_next_block(timestamp.as_secs(), nonce, hash, data)?;

        self.client.save_block(&nxt_block)?;
        self.hashes.push(nxt_block.get_hash());
        Ok(true)
    }

    pub fn create_next_block(
        &mut self,
        timestamp: u64,
        nonce: u32,
        hash: BlockHash,
        data: Vec<u8>,
    ) -> Result<Block> {
        let block = self.get_last_block()?;
        let difficulty = self.get_difficulty()?;

        let result = Block {
            block_number: self.hashes.len(),
            timestamp,
            difficulty,
            nonce,
            data: data.clone(),
            hash,
            prev_hash: block.get_hash().clone(),
        };

        Ok(result)
    }

    pub fn set_synced(&mut self, is_synced: bool) {
        self.synced = is_synced;
    }

    pub fn get_block_by_chain_index(&mut self, index: usize) -> Result<Block> {
        let block_hash = &self.hashes[index];
        let block = self.client.get_block_by_hash(block_hash)?;

        Ok(block)
    }

    pub fn get_last_block(&mut self) -> Result<Block> {
        let block = self.get_block_by_chain_index(self.hashes.len() - 1)?;

        Ok(block)
    }

    pub fn generate_next_block_hash(
        &mut self,
        timestamp: u64,
        data: Vec<u8>,
    ) -> JoinHandle<(BlockHash, u32)> {
        let block = self.get_last_block().unwrap_or(Block::default());
        let nxt_difficulty = self.get_difficulty().unwrap_or(0);

        std::thread::spawn(move || loop {
            let mut nonce: u32 = 0;
            let target_zeroes: &[u8] = &vec![0; (nxt_difficulty / 8) as usize];
            let leftover_target = 255 / 2u8.pow((nxt_difficulty % 8) as u32);
            let data = &data;
            let block_hash_data = [
                &timestamp.to_be_bytes(),
                &data[..],
                &block.get_hash()[..],
                &nxt_difficulty.to_be_bytes(),
                &nonce.to_be_bytes(),
            ]
            .concat();
            let hash = Block::block_hash(&block_hash_data);
            let leftover_byte = hash[target_zeroes.len()];

            if hash.starts_with(target_zeroes)
                && (leftover_byte | leftover_target) <= leftover_target
            {
                break (hash, nonce);
            }
            nonce += 1;
        })
    }

    pub fn get_difficulty(&mut self) -> Result<u32> {
        let last_block = self.get_last_block()?;

        let prev_timestamp: u32 = if self.hashes.len() > 1 {
            let prev_block = self
                .client
                .get_block_by_hash(&self.hashes[self.hashes.len() - 2])?;
            prev_block.get_difficulty()
        } else {
            0
        };

        let res = if last_block.get_difficulty() - prev_timestamp > BLOCK_TIME {
            last_block.get_difficulty() - 1
        } else {
            last_block.get_difficulty() + 1
        };

        Ok(res)
    }

    pub fn print_chain(&self) {
        for (i, block) in self.hashes.iter().enumerate() {
            println!("block #{}: {:#?}", i, block);
            println!();
        }
    }

    pub fn is_valid_chain(&mut self) -> Result<bool> {
        for i in 0..self.hashes.len() - 1 {
            let block = self.get_block_by_chain_index(i)?;
            let nxt_block = self.get_block_by_chain_index(i + 1)?;

            let hash1 = block.get_hash();
            let hash2 = nxt_block.get_hash();

            if !block.get_hash().eq(&nxt_block.get_hash()) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[allow(dead_code, unused_variables, unused_mut, clippy::unused_unit)]
pub fn replace_chain(mut target: &Chain, copy: &Chain) -> () {}

#[cfg(test)]
mod test {
    use crate::blockchain::block::*;
    use crate::blockchain::chain::*;

    fn create_chain() -> Result<Chain> {
        let mut chain = Chain::default()?;
        chain.mine_block(0, &b"first block data".to_vec());
        Ok(chain)
    }

    fn get_difficulty_test() {
        let mut chain = create_chain();
    }

    #[test]
    #[allow(dead_code)]
    fn add_block_test() {
        //let mut chain = create_chain();
        //let mut chain_copy = chain.to_vec();
        //let timestamp = 5;
        //let original_len = chain.len();

        //chain.mine_block(timestamp, &b"first block data".to_vec());

        //let nxt_block = get_last_block(&chain);
        //let (nxt_hash, nonce) =
        //generate_next_block_hash(&chain_copy, nxt_block.get_timestamp(), nxt_block.get_data());

        //assert_eq!(chain.len(), original_len + 1);
        //assert_eq!(nxt_block.get_hash(), nxt_hash);
        //assert_eq!(nxt_block.get_nonce(), nonce);
    }

    #[test]
    #[allow(dead_code)]
    fn is_valid_chain_test() -> Result<()> {
        //let mut chain = create_chain()?;
        //let timestamp = 5;

        //chain.mine_block(timestamp, &b"first block data".to_vec());

        //chain.mine_block(Block {
        //timestamp: 10,
        //block_number: 0,
        //data: b"invalid data".to_vec(),
        //hash: Block::block_hash(&b"invalid hash".to_vec()),
        //prev_hash: Block::block_hash(&b"invalid hash 2".to_vec()),
        //difficulty: 4,
        //nonce: 3,
        //});

        //assert!(!is_valid_chain(&chain));
        Ok(())
    }

    #[test]
    #[allow(dead_code)]
    fn generate_next_block_hash_test() {
        //let chain = create_chain();
        //let timestamp = 5;
        //let data = b"some data".to_vec();
        //let difficulty = get_difficulty(&chain);

        //let (nxt_block_hash, nonce) = generate_next_block_hash(&chain, timestamp, &data);
        //let block_hash_data = [
        //&timestamp.to_be_bytes(),
        //&data[..],
        //&get_last_block(&chain).get_hash()[..],
        //&difficulty.to_be_bytes(),
        //&nonce.to_be_bytes(),
        //]
        //.concat();

        //let target_zeroes: &[u8] = &vec![0; (difficulty / 8) as usize];
        //let leftover_target = 255 / 2u8.pow((difficulty % 8) as u32);
        //let verify_hash = Block::block_hash(&block_hash_data);

        //assert!(nxt_block_hash.starts_with(target_zeroes));
        //assert!((nxt_block_hash[target_zeroes.len()] | leftover_target) <= leftover_target);
        //assert_eq!(nxt_block_hash, verify_hash);
    }

    #[test]
    #[allow(dead_code)]
    fn create_next_block_test() {
        //let mut chain = create_chain();
        //let timestamp = 5;
        //let nxt_timestamp = timestamp + 5;

        //mine_block(&mut chain, timestamp, &b"first block data".to_vec());

        //let block = &chain[chain.len() - 1];
        //let nxt_block = create_next_block(&chain, nxt_timestamp, &b"some data".to_vec());
        //let (nxt_hash, nonce) =
        //generate_next_block_hash(&chain, nxt_timestamp, &b"some data".to_vec());

        //assert_eq!(nxt_block.get_timestamp(), nxt_timestamp);
        //assert_eq!(nxt_block.get_prev_hash(), block.get_hash());
        //assert_eq!(nxt_block.get_hash(), nxt_hash);
    }
}
