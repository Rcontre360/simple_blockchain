use crate::chain_block::Block;
use bytes::Bytes;

pub type Chain = Vec<Block>;

pub fn is_valid_chain(chain: &Chain) -> bool {
    let mut cur_block = Block::default();
    for i in 0..chain.len() - 1 {
        let block = &chain[i];
        let nxt_block = &chain[i + 1];

        if *cur_block.get_hash() != *block.get_hash() {
            return false;
        }

        cur_block =
            Block::create_next_block(&cur_block, nxt_block.get_timestamp(), nxt_block.get_data());
    }
    true
}

pub fn add_block(chain: &mut Chain, timestamp: u32, data: &Bytes) {
    if chain.is_empty() {
        return;
    }

    let prev_block = &chain[chain.len() - 1];
    chain.push(Block::create_next_block(
        prev_block,
        timestamp,
        &data.clone(),
    ));
}

#[allow(dead_code, unused_variables, unused_mut, clippy::unused_unit)]
pub fn replace_chain(mut target: &Chain, copy: &Chain) -> () {}

#[cfg(test)]
mod test {
    use crate::chain::*;
    use crate::chain_block::*;

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
}
