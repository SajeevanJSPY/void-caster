//! Block should be compatible with the Ethereum and Tendermint, for the execution

use tendermint_proto::types::Block as TmBlock;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block(TmBlock);
