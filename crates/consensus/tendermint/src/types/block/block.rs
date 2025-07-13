use prost::Message;

use crate::types::Height;
use void_proto::{
    google::Timestamp,
    tendermint::types::{Block as TmBlock, Data as BlockData, Header as BlockHeader},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block(TmBlock);

impl Block {
    pub fn encode(&self) -> Vec<u8> {
        self.0.encode_to_vec()
    }

    pub(crate) fn new(header: BlockHeader, data: BlockData) -> Self {
        Self(TmBlock {
            header: Some(header),
            data: Some(data),
            evidence: None,
            last_commit: None,
        })
    }

    pub fn header(&self) -> BlockHeader {
        self.0.header.clone().unwrap()
    }

    pub fn transactions(&self) -> Vec<Vec<u8>> {
        self.0.data.clone().unwrap().txs.clone()
    }

    pub fn time(&self) -> Timestamp {
        self.header()
            .time
            .expect("time is not set for the Block")
            .clone()
    }

    pub fn height(&self) -> Height {
        self.header().height
    }

    pub fn proposer_address(&self) -> Vec<u8> {
        self.header().proposer_address
    }

    pub fn validators_hash(&self) -> Vec<u8> {
        self.header().validators_hash
    }
}
