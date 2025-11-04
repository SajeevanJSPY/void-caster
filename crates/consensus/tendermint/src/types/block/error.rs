use super::Block;

pub type BlockBuilderResult = Result<Block, BlockBuilderError>;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum BlockBuilderError {
    #[error("transactions cannot be empty")]
    TransactionEmpty,
    #[error("invalid proposer")]
    InvalidProposer,
    #[error("invalid validator hash")]
    InvalidValidatorsHash,
}
