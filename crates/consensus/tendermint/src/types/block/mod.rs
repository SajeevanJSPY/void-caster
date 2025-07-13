//! Block should be compatible with the Ethereum and Tendermint, for the execution

mod block;
mod builder;
mod error;

pub use block::*;
pub use builder::*;
pub use error::*;

#[cfg(test)]
mod block_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use void_proto::google::Timestamp;

    const CHAIN_ID: &str = "void-caster-devnet";

    fn time() -> Timestamp {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Timestamp {
            seconds: now.as_secs() as i64,
            nanos: now.subsec_nanos() as i32,
        }
    }

    #[test]
    fn test_block_builder() {
        // get the current time
        let time = time();

        let dummy_txs = vec![vec![45, 54, 35, 53, 41], vec![65, 73, 21, 54, 231, 65]];
        let proposer_address = vec![0; 20];
        let validators_hash = vec![1; 32];

        let block = BlockBuilder::new(CHAIN_ID.to_string(), validators_hash.clone());
        // make a cheap copy the BlockBuilder
        let block1 = block
            .clone()
            .with_height(1)
            .with_transactions(dummy_txs.clone())
            .build(time.clone(), proposer_address.clone())
            .unwrap();

        assert_eq!(block1.height(), 1);
        assert_eq!(block1.proposer_address(), proposer_address.clone());
        assert_eq!(block1.validators_hash(), validators_hash.clone());
        assert_eq!(block1.transactions(), dummy_txs.clone());
        assert_eq!(block1.time(), time.clone());

        // make another cheap copy of the BlockBuilder
        let block2 = block
            .clone()
            .with_height(2)
            .with_transactions(dummy_txs.clone())
            .build(time.clone(), proposer_address.clone())
            .unwrap();

        assert_eq!(block2.height(), 2);
        assert_eq!(block2.validators_hash(), validators_hash.clone());
        assert_eq!(block2.proposer_address(), proposer_address);
    }

    #[test]
    fn test_invalid_validators_hash() {
        let time = time();

        // try to create with empty proposer address
        let block = BlockBuilder::new(CHAIN_ID.to_string(), vec![])
            .build(time.clone(), vec![])
            .unwrap_err();

        assert_eq!(block, BlockBuilderError::InvalidValidatorsHash);
    }
}
