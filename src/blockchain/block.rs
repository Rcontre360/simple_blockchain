use rocket::serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

#[allow(dead_code)]
#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Block {
    pub timestamp: u32,
    pub difficulty: u32,
    pub nonce: u32,
    pub data: Vec<u8>,
    pub hash: [u8; 32],
    pub prev_hash: [u8; 32],
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Block")
            .field("nonce", &self.nonce)
            .field("difficulty", &self.difficulty)
            .field("timestamp", &self.timestamp)
            .field("data", &self.data.to_vec())
            .field("hash", &self.hash.to_vec())
            .field("prev_hash", &self.prev_hash.to_vec())
            .finish()
    }
}

#[allow(dead_code)]
impl Block {
    pub fn default() -> Block {
        Block {
            timestamp: 0,
            difficulty: 0,
            nonce: 0,
            data: Vec::from([]),
            hash: Block::block_hash(&b"".to_vec()),
            prev_hash: Block::block_hash(&b"".to_vec()),
        }
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn get_nonce(&self) -> u32 {
        self.nonce
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_hash(&self) -> [u8; 32] {
        self.hash.clone()
    }

    pub fn get_prev_hash(&self) -> [u8; 32] {
        self.prev_hash.clone()
    }

    pub fn block_hash(bytes: &Vec<u8>) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let res: [u8; 32] = hasher
            .finalize()
            .to_vec()
            .try_into()
            .unwrap_or_else(|v: Vec<u8>| {
                panic!("Expected a Vec of length {} but it was {}", 32, v.len());
            });
        res
    }
}

#[cfg(test)]
mod test {
    use crate::blockchain::block::*;

    #[test]
    #[allow(dead_code)]
    fn block_creation_test() {
        let timestamp = 5;
        let data = b"".to_vec();
        let hash = Block::block_hash(&b"".to_vec());
        let prev_hash = Block::block_hash(&b"".to_vec());
        let block = Block {
            timestamp,
            data: data.clone(),
            hash: hash.clone(),
            prev_hash: prev_hash.clone(),
            difficulty: 4,
            nonce: 4,
        };

        assert_eq!(timestamp, block.get_timestamp());
        assert_eq!(data, *block.get_data());
        assert_eq!(hash, block.get_hash());
        assert_eq!(prev_hash, block.get_prev_hash());
    }
}
