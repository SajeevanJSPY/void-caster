use prost::Message;

use crate::types::Height;
use void_proto::{
    google::Timestamp,
    tendermint::types::{Block as TmBlock, Data, Header as BlockHeader},
};

use super::{
    Block,
    error::{BlockBuilderError, BlockBuilderResult},
};

// TODO: this structure should have access to global state
//      verify the Block before creating a new Block
#[derive(Default)]
pub struct BlockBuilder {
    height: Height,
    chain_id: String,
    validators_hash: Vec<u8>,
    transactions: Vec<Vec<u8>>,
}

impl Clone for BlockBuilder {
    fn clone(&self) -> Self {
        Self {
            chain_id: self.chain_id.clone(),
            validators_hash: self.validators_hash.clone(),
            ..Default::default()
        }
    }
}

impl BlockBuilder {
    pub fn new(chain_id: String, validators_hash: Vec<u8>) -> Self {
        Self {
            chain_id,
            validators_hash,
            ..Default::default()
        }
    }

    pub fn with_transactions(mut self, transactions: Vec<Vec<u8>>) -> Self {
        self.transactions = transactions;
        self
    }

    pub fn with_height(mut self, height: Height) -> Self {
        self.height = height;
        self
    }

    pub fn with_validators_hash(mut self, validators_hash: Vec<u8>) -> Self {
        self.validators_hash = validators_hash;
        self
    }

    pub fn with_chain_id(mut self, chain_id: String) -> Self {
        self.chain_id = chain_id;
        self
    }

    pub fn build(self, time: Timestamp, proposer_address: Vec<u8>) -> BlockBuilderResult {
        if self.validators_hash.clone().is_empty() {
            return Err(BlockBuilderError::InvalidValidatorsHash);
        }

        let block_header = BlockHeader {
            version: None,
            chain_id: "void-caster-devnet".to_string(),
            height: self.height,
            time: Some(time),
            proposer_address,
            validators_hash: self.validators_hash,
            ..Default::default()
        };

        let block_data = Data {
            txs: self.transactions,
        };

        Ok(Block::new(block_header, block_data))
    }
}
