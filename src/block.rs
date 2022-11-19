use bytes::Bytes;
use sha2::{Digest, Sha256};

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Block {
    pub timestamp: u32,
    pub data: Bytes,
    pub hash: Bytes,
    pub prev_hash: Bytes,
    pub difficulty: u32,
    pub nonce: u32,
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
}

#[cfg(test)]
mod test {
    use crate::block::*;

    #[test]
    #[allow(dead_code)]
    fn block_creation_test() {
        let timestamp = 5;
        let data: Bytes = Bytes::new();
        let hash: Bytes = Bytes::new();
        let prev_hash = Bytes::new();
        let block = Block {
            timestamp,
            data: data.clone(),
            hash: hash.clone(),
            prev_hash: prev_hash.clone(),
            difficulty: 4,
            nonce: 4,
        };

        assert_eq!(timestamp, block.get_timestamp());
        assert_eq!(data, block.get_data());
        assert_eq!(hash, block.get_hash());
        assert_eq!(prev_hash, block.get_prev_hash());
    }
}