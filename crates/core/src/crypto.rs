use vc_types::crypto::{CryptoError, Keypair, PeerId, PublicKey};

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
