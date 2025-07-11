use prost::{DecodeError, EncodeError};

#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error("failed to encode the structure")]
    EncodingError(#[from] EncodeError),
    #[error("failed to decode the structure")]
    DecodingError(#[from] DecodeError),
}

#[derive(Debug, thiserror::Error)]
pub enum TendermintError {
    #[error(transparent)]
    CodecError(#[from] CodecError),
    #[error("invalid proposal")]
    InvaidProposal,
}
