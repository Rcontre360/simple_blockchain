use bytes::Bytes;
use rocket::serde::ser::{Serialize, SerializeStruct, Serializer};
use sha2::{Digest, Sha256};
use std::fmt;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Block {
    pub timestamp: u32,
    pub difficulty: u32,
    pub nonce: u32,
    pub data: Bytes,
    pub hash: Bytes,
    pub prev_hash: Bytes,
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Block", 3)?;
        s.serialize_field("timestamp", &self.timestamp)?;
        s.serialize_field("difficulty", &self.difficulty)?;
        s.serialize_field("nonce", &self.nonce)?;
        s.serialize_field("data", &self.data.to_vec())?;
        s.serialize_field("hash", &self.hash.to_vec())?;
        s.serialize_field("prev_hash", &self.prev_hash.to_vec())?;
        s.end()
    }
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
    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn get_nonce(&self) -> u32 {
        self.nonce
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
    use crate::blockchain::block::*;

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
