use crate::blockchain::block::Block;

pub type Chain = Vec<Block>;

pub const BLOCK_TIME: u32 = 1000 * 5; // 5 seconds

pub fn add_block(chain: &mut Chain, timestamp: u32, data: &Vec<u8>) {
    if chain.is_empty() {
        return;
    }

    chain.push(create_next_block(chain, timestamp, &data.clone()));
}

pub fn create_next_block(chain: &Chain, timestamp: u32, data: &Vec<u8>) -> Block {
    let block = &chain[chain.len() - 1];
    let difficulty = get_difficulty(chain);
    let (hash, nonce) = generate_next_block_hash(chain, timestamp, data);

    Block {
        timestamp,
        difficulty,
        nonce,
        data: data.clone(),
        hash,
        prev_hash: block.get_hash().clone(),
    }
}

pub fn generate_next_block_hash(chain: &Chain, timestamp: u32, data: &Vec<u8>) -> ([u8; 32], u32) {
    let block = &chain[chain.len() - 1];
    let nxt_difficulty = get_difficulty(chain);

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

        if hash.starts_with(target_zeroes) && (leftover_byte | leftover_target) <= leftover_target {
            return (hash, nonce);
        }
        nonce += 1;
    }
}

pub fn get_difficulty(chain: &Chain) -> u32 {
    let last_block = get_last_block(chain);
    let prev_timestamp: u32 = if chain.len() > 1 {
        chain[chain.len() - 2].get_timestamp()
    } else {
        0
    };

    if last_block.get_timestamp() - prev_timestamp > BLOCK_TIME {
        last_block.get_timestamp() - 1
    } else {
        last_block.get_timestamp() + 1
    }
}

#[allow(dead_code, unused_variables, unused_mut, clippy::unused_unit)]
pub fn replace_chain(mut target: &Chain, copy: &Chain) -> () {}

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

        if !block.get_hash().eq(&nxt_block.get_hash()) {
            return false;
        }
    }
    true
}

pub fn print_chain(chain: &Chain) {
    for (i, block) in chain.iter().enumerate() {
        println!("block #{}: {:#?}", i, block);
        println!();
    }
}

pub fn get_last_block(chain: &Chain) -> &Block {
    &chain[chain.len() - 1]
}

#[cfg(test)]
mod test {
    use crate::blockchain::block::*;
    use crate::blockchain::chain::*;

    fn create_chain() -> Chain {
        let mut chain: Vec<Block> = vec![Block::default()];
        add_block(&mut chain, 0, &b"first block data".to_vec());
        chain
    }

    fn get_difficulty_test() {
        let mut chain = create_chain();
    }

    #[test]
    #[allow(dead_code)]
    fn add_block_test() {
        let mut chain = create_chain();
        let mut chain_copy = chain.to_vec();
        let timestamp = 5;
        let original_len = chain.len();

        add_block(&mut chain, timestamp, &b"first block data".to_vec());

        let nxt_block = get_last_block(&chain);
        let (nxt_hash, nonce) =
            generate_next_block_hash(&chain_copy, nxt_block.get_timestamp(), nxt_block.get_data());

        assert_eq!(chain.len(), original_len + 1);
        assert_eq!(nxt_block.get_hash(), nxt_hash);
        assert_eq!(nxt_block.get_nonce(), nonce);
    }

    #[test]
    #[allow(dead_code)]
    fn is_valid_chain_test() {
        let mut chain = create_chain();
        let timestamp = 5;

        add_block(&mut chain, timestamp, &b"first block data".to_vec());

        chain.push(Block {
            timestamp: 10,
            data: b"invalid data".to_vec(),
            hash: Block::block_hash(&b"invalid hash".to_vec()),
            prev_hash: Block::block_hash(&b"invalid hash 2".to_vec()),
            difficulty: 4,
            nonce: 3,
        });

        assert!(!is_valid_chain(&chain));
    }

    #[test]
    #[allow(dead_code)]
    fn generate_next_block_hash_test() {
        let chain = create_chain();
        let timestamp = 5;
        let data = b"some data".to_vec();
        let difficulty = get_difficulty(&chain);

        let (nxt_block_hash, nonce) = generate_next_block_hash(&chain, timestamp, &data);
        let block_hash_data = [
            &timestamp.to_be_bytes(),
            &data[..],
            &get_last_block(&chain).get_hash()[..],
            &difficulty.to_be_bytes(),
            &nonce.to_be_bytes(),
        ]
        .concat();

        let target_zeroes: &[u8] = &vec![0; (difficulty / 8) as usize];
        let leftover_target = 255 / 2u8.pow((difficulty % 8) as u32);
        let verify_hash = Block::block_hash(&block_hash_data);

        assert!(nxt_block_hash.starts_with(target_zeroes));
        assert!((nxt_block_hash[target_zeroes.len()] | leftover_target) <= leftover_target);
        assert_eq!(nxt_block_hash, verify_hash);
    }

    #[test]
    #[allow(dead_code)]
    fn create_next_block_test() {
        let mut chain = create_chain();
        let timestamp = 5;
        let nxt_timestamp = timestamp + 5;

        add_block(&mut chain, timestamp, &b"first block data".to_vec());

        let block = &chain[chain.len() - 1];
        let nxt_block = create_next_block(&chain, nxt_timestamp, &b"some data".to_vec());
        let (nxt_hash, nonce) =
            generate_next_block_hash(&chain, nxt_timestamp, &b"some data".to_vec());

        assert_eq!(nxt_block.get_timestamp(), nxt_timestamp);
        assert_eq!(nxt_block.get_prev_hash(), block.get_hash());
        assert_eq!(nxt_block.get_hash(), nxt_hash);
    }
}
