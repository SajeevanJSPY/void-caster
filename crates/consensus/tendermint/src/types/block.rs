//! Block should be compatible with the Ethereum and Tendermint, for the execution

use void_proto::tendermint::types::Block as TmBlock;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block(TmBlock);
