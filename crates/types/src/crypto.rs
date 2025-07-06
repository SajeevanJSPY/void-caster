use libp2p::identity::{self, ed25519};

pub type Keypair = ed25519::Keypair;
pub type PublicKey = ed25519::PublicKey;
pub type PeerId = libp2p::PeerId;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("failed to decode the public key from the bytes")]
    DecodingError(#[from] identity::DecodingError),
    #[error("An error during signing of a message")]
    SigningError(#[from] identity::SigningError),
}
