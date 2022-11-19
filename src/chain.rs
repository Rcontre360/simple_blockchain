use crate::block::Block;
use bytes::Bytes;

pub type Chain = Vec<Block>;

pub fn is_valid_chain(chain: &Chain) -> bool {
    for i in 0..chain.len() - 1 {
        let block = &chain[i];
        let nxt_block = &chain[i + 1];

        let hash1 = block.get_hash();
        let hash2 = nxt_block.get_hash();
        println!("debug: {} & {}", hash1.len(), hash2.len());
        for j in 0..std::cmp::min(hash1.len(), hash2.len()) {
            println!("equal: {} & {}", hash1[j], hash2[j]);
        }

        if !block.get_hash().eq(nxt_block.get_hash()) {
            return false;
        }
    }
    true
}

pub fn add_block(chain: &mut Chain, timestamp: u32, data: &Bytes) {
    if chain.is_empty() {
        return;
    }

    chain.push(create_next_block(chain, timestamp, &data.clone()));
}

pub fn create_next_block(chain: &Chain, timestamp: u32, data: &Bytes) -> Block {
    let block = &chain[chain.len() - 1];
    let block_hash_data = [&timestamp.to_be_bytes(), &data[..], &block.hash[..]].concat();
    let hash = Block::block_hash(block_hash_data);

    Block {
        timestamp,
        hash,
        data: data.clone(),
        prev_hash: block.hash.clone(),
        nonce: 4,
        difficulty: 5,
    }
}

pub fn generate_next_block_hash(chain: &Chain, timestamp: u32, data: &Bytes) -> Bytes {
    let block = &chain[chain.len() - 1];
    let nonce: u32 = 0;
    let nxt_difficulty: usize = 17;
    let target_zeroes: &[u8] = &vec![0; nxt_difficulty / 8];
    let leftover_target = 2u8.pow((nxt_difficulty % 8 + 1) as u32) - 1;

    loop {
        let block_hash_data = [
            &timestamp.to_be_bytes(),
            &data[..],
            &block.hash[..],
            &nxt_difficulty.to_be_bytes(),
            &nonce.to_be_bytes(),
        ]
        .concat();
        let hash = Block::block_hash(block_hash_data);

        //check if paddleft is 0 for each bite
        if hash.starts_with(target_zeroes) && hash[target_zeroes.len()] ^ leftover_target == 0 {
            return hash;
        }
    }
}

#[allow(dead_code, unused_variables, unused_mut, clippy::unused_unit)]
pub fn replace_chain(mut target: &Chain, copy: &Chain) -> () {}

#[cfg(test)]
mod test {
    use crate::block::*;
    use crate::chain::*;

    fn create_chain() -> Chain {
        let chain: Vec<Block> = vec![Block::default()];
        chain
    }

    #[test]
    #[allow(dead_code)]
    fn add_block_test() {
        let mut chain = create_chain();
        let timestamp = 5;

        add_block(
            &mut chain,
            timestamp,
            &Bytes::from_static(b"first block data"),
        );

        let first_block = &chain[0];
        let second_block = &chain[chain.len() - 1];
        let nxt_hash = Block::block_hash(
            [
                &second_block.get_timestamp().to_be_bytes(),
                &second_block.get_data()[..],
                &first_block.get_hash()[..],
            ]
            .concat(),
        );

        assert_eq!(chain.len(), 2);
        assert_eq!(*second_block.get_hash(), nxt_hash);
    }

    #[test]
    #[allow(dead_code)]
    fn is_valid_chain_test() {
        let mut chain = create_chain();
        let timestamp = 5;

        add_block(
            &mut chain,
            timestamp,
            &Bytes::from_static(b"first block data"),
        );

        chain.push(Block {
            timestamp: 10,
            data: Bytes::from_static(b"invalid data"),
            hash: Bytes::from_static(b"invalid hash"),
            prev_hash: Bytes::from_static(b"invalid hash 2"),
        });

        assert!(!is_valid_chain(&chain));
    }

    #[test]
    #[allow(dead_code)]
    fn create_next_block_test() {
        let mut chain = create_chain();
        let timestamp = 5;
        let nxt_timestamp = timestamp + 5;

        add_block(
            &mut chain,
            timestamp,
            &Bytes::from_static(b"first block data"),
        );

        let block = &chain[chain.len() - 1];
        let nxt_block = create_next_block(&chain, nxt_timestamp, &Bytes::new());
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
