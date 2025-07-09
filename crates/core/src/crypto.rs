use serde::{Deserialize, Serialize};
use vc_types::crypto::{CryptoError, Keypair, PeerId, PublicKey};

#[derive(Debug, Hash, PartialEq, Default, Eq, Clone, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Digest(pub [u8; 32]);

impl Digest {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub struct NodeId {
    keypair: Keypair,
}

impl NodeId {
    pub fn new() -> Self {
        let keypair = Keypair::generate();
        Self { keypair }
    }

    pub fn from_bytes(kp: &mut [u8]) -> Result<Self, CryptoError> {
        let keypair = Keypair::try_from_bytes(kp)?;
        Ok(Self { keypair })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.keypair.to_bytes().into()
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.keypair.sign(message)
    }

    pub fn public_key(&self) -> PublicKey {
        self.keypair.public()
    }

    pub fn peer_id(&self) -> PeerId {
        PeerId::from_public_key(&self.public_key().into())
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Signature {
    part1: [u8; 32],
    part2: [u8; 32],
}

impl Signature {
    pub fn new(digest: &Digest, keypair: &Keypair) -> Self {
        let sig = keypair.sign(&digest.0);
        let part1 = sig[..32].try_into().expect("Unexpected signature length");
        let part2 = sig[32..64].try_into().expect("Unexpected signature length");
        Self { part1, part2 }
    }

    pub fn flatten(&self) -> [u8; 64] {
        [self.part1, self.part2]
            .concat()
            .try_into()
            .expect("Unexpected signature length")
    }

    pub fn verify(&self, digest: &Digest, public_key: &PublicKey) -> Result<bool, CryptoError> {
        let signature = self.flatten().clone();
        Ok(public_key.verify(&digest.0, &signature))
    }
}

pub trait Hash {
    fn digest(&self) -> Digest;
}
