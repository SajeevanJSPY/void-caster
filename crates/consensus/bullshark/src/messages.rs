use sha2::{Digest as _, Sha512};
use std::collections::{BTreeMap, BTreeSet};

use vc_core::crypto::{Digest, Hash, Signature};
use vc_types::crypto::PublicKey;

use crate::types::{Round, WorkerId};

#[derive(Debug, Clone)]
pub struct Header {
    pub author: PublicKey,
    pub round: Round,
    pub payload: BTreeMap<Digest, WorkerId>,
    pub parents: BTreeSet<Digest>,
    pub id: Digest,
    pub signature: Signature,
}

impl Hash for Header {
    fn digest(&self) -> Digest {
        let mut hasher = Sha512::new();
        hasher.update(&self.author.to_bytes());
        hasher.update(self.round.to_le_bytes());
        for (x, y) in &self.payload {
            hasher.update(x);
            hasher.update(y.to_le_bytes());
        }
        for x in &self.parents {
            hasher.update(x);
        }

        Digest(hasher.finalize().as_slice()[..32].try_into().unwrap())
    }
}

impl Header {
    pub fn new(round: Round, author: PublicKey) -> Self {
        let header = Self {
            author,
            round,
            payload: BTreeMap::new(),
            parents: BTreeSet::new(),
            id: Digest::default(),
            signature: Signature::default(),
        };

        let id = header.digest();

        Self { id, ..header }
    }
}

#[derive(Debug, Clone)]
pub struct Vote {
    pub id: Digest,
    pub round: Round,
    pub origin: PublicKey,
    pub author: PublicKey,
    pub signature: Signature,
}

impl Hash for Vote {
    fn digest(&self) -> Digest {
        let mut hasher = Sha512::new();
        hasher.update(&self.id);
        hasher.update(self.round.to_le_bytes());
        hasher.update(&self.origin.to_bytes());
        Digest(hasher.finalize().as_slice()[..32].try_into().unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Certificate {
    pub header: Header,
    pub votes: Vec<(PublicKey, Signature)>,
}

impl Certificate {
    pub fn new(round: Round, author: PublicKey) -> Self {
        Self {
            header: Header::new(round, author),
            votes: vec![],
        }
    }

    pub fn round(&self) -> Round {
        self.header.round
    }

    pub fn origin(&self) -> PublicKey {
        self.header.author.clone()
    }
}

impl Hash for Certificate {
    fn digest(&self) -> Digest {
        let mut hasher = Sha512::new();
        hasher.update(&self.header.id);
        hasher.update(self.round().to_le_bytes());
        hasher.update(&self.origin().to_bytes());
        Digest(hasher.finalize().as_slice()[..32].try_into().unwrap())
    }
}
