use bytes::Bytes;
use sha2::{Digest, Sha256};

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Block {
    pub timestamp: u32,
    pub data: Bytes,
    pub hash: Bytes,
    pub prev_hash: Bytes,
}

#[allow(dead_code)]
impl Block {
    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn get_data(&self) -> &Bytes {
        &self.data
    }

    pub fn get_hash(&self) -> &Bytes {
        &self.hash
    }

    pub fn get_prev_hash(&self) -> &Bytes {
        &self.prev_hash
    }

    pub fn block_hash(bytes: Vec<u8>) -> Bytes {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        Bytes::from(hasher.finalize().to_vec())
    }

    pub fn create_next_block(block: &Block, timestamp: u32, data: &Bytes) -> Block {
        let block_hash_data = [&timestamp.to_be_bytes(), &data[..], &block.hash[..]].concat();
        let hash = Block::block_hash(block_hash_data);

        Block {
            timestamp,
            hash,
            data: data.clone(),
            prev_hash: block.hash.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::chain_block::*;

    #[test]
    #[allow(dead_code)]
    fn block_creation_test() -> () {
        let timestamp = 5;
        let data: Bytes = Bytes::new();
        let hash: Bytes = Bytes::new();
        let prev_hash = Bytes::new();
        let block = Block {
            timestamp,
            data: data.clone(),
            hash: hash.clone(),
            prev_hash: prev_hash.clone(),
        };

        assert_eq!(timestamp, block.get_timestamp());
        assert_eq!(data, block.get_data());
        assert_eq!(hash, block.get_hash());
        assert_eq!(prev_hash, block.get_prev_hash());
    }

    #[test]
    #[allow(dead_code)]
    fn create_next_block_test() -> () {
        let timestamp = 5;
        let nxt_timestamp = timestamp + 5;
        let block = Block {
            timestamp,
            data: Bytes::new(),
            hash: Bytes::new(),
            prev_hash: Bytes::new(),
        };

        let nxt_block = Block::create_next_block(&block, nxt_timestamp, &Bytes::new());
        let nxt_hash = Block::block_hash(
            [
                &nxt_timestamp.to_be_bytes(),
                &Bytes::new()[..],
                &block.hash[..],
            ]
            .concat(),
        );

        assert_eq!(nxt_block.get_timestamp(), nxt_timestamp);
        assert_eq!(*nxt_block.get_prev_hash(), block.hash);
        assert_eq!(*nxt_block.get_hash(), nxt_hash);
    }
}
