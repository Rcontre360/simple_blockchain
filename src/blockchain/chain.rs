use crate::blockchain::block::{Block, BlockHash};
use crate::storage::Client;
use anyhow::Result;

pub const BLOCK_TIME: u32 = 1000 * 5; // 5 seconds

pub struct Chain {
    client: Client,
    pub hashes: Vec<BlockHash>,
}

impl Chain {
    pub fn new() -> Result<Chain> {
        let mut chain = Chain {
            client: Client::new()?,
            hashes: vec![],
        };
        let genesis_block = &Block::default();
        chain.hashes.push(genesis_block.get_hash());
        chain.client.save_block(genesis_block)?;

        Ok(chain)
    }

    pub fn add_block(&mut self, timestamp: u32, data: &Vec<u8>) -> Result<bool> {
        if self.hashes.is_empty() {
            return Ok(false);
        }

        let nxt_block = self.create_next_block(timestamp, &data.clone())?;
        self.hashes.push(nxt_block);

        Ok(true)
    }

    pub fn create_next_block(&mut self, timestamp: u32, data: &Vec<u8>) -> Result<BlockHash> {
        let block = self.get_last_block()?;
        let difficulty = self.get_difficulty()?;
        let (hash, nonce) = self.generate_next_block_hash(timestamp, data)?;

        let result = Block {
            block_number: 0,
            timestamp,
            difficulty,
            nonce,
            data: data.clone(),
            hash,
            prev_hash: block.get_hash().clone(),
        };

        Ok(result.get_hash())
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
        timestamp: u32,
        data: &Vec<u8>,
    ) -> Result<(BlockHash, u32)> {
        let block = self.get_last_block()?;
        let nxt_difficulty = self.get_difficulty()?;

        let mut nonce: u32 = 0;
        let target_zeroes: &[u8] = &vec![0; (nxt_difficulty / 8) as usize];
        let leftover_target = 255 / 2u8.pow((nxt_difficulty % 8) as u32);

        loop {
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
                return Ok((hash, nonce));
            }
            nonce += 1;
        }
    }

    pub fn get_difficulty(&mut self) -> Result<u32> {
        let last_block = self.get_last_block()?;

        let prev_timestamp: u32 = if self.hashes.len() > 1 {
            let prev_block = self
                .client
                .get_block_by_hash(&self.hashes[self.hashes.len() - 2])?;
            prev_block.get_timestamp()
        } else {
            0
        };

        let res = if last_block.get_timestamp() - prev_timestamp > BLOCK_TIME {
            last_block.get_timestamp() - 1
        } else {
            last_block.get_timestamp() + 1
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
        let mut chain = Chain::new()?;
        chain.add_block(0, &b"first block data".to_vec());
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

        //chain.add_block(timestamp, &b"first block data".to_vec());

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

        //chain.add_block(timestamp, &b"first block data".to_vec());

        //chain.add_block(Block {
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

        //add_block(&mut chain, timestamp, &b"first block data".to_vec());

        //let block = &chain[chain.len() - 1];
        //let nxt_block = create_next_block(&chain, nxt_timestamp, &b"some data".to_vec());
        //let (nxt_hash, nonce) =
        //generate_next_block_hash(&chain, nxt_timestamp, &b"some data".to_vec());

        //assert_eq!(nxt_block.get_timestamp(), nxt_timestamp);
        //assert_eq!(nxt_block.get_prev_hash(), block.get_hash());
        //assert_eq!(nxt_block.get_hash(), nxt_hash);
    }
}
