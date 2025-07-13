use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use sha3::{Digest, Keccak256};

use vc_types::crypto::{Keypair, PublicKey};
use void_proto::tendermint::types::{Validator, ValidatorSet};

#[derive(Debug)]
pub struct Committee(Arc<Mutex<ValidatorSet>>);

impl Committee {
    /// get the initial validators from the genesis file
    pub fn genesis(validators: impl Iterator<Item = (PublicKey, i64)>) -> Self {
        let mut total_voting_power = 0;

        let validators = validators
            .into_iter()
            .map(|(pub_key, voting_power)| {
                total_voting_power += voting_power;
                Self::validator(&pub_key, voting_power)
            })
            .collect();

        // set the initial proposer as None, implement a Algorithm to find out the proposer
        let validator_set = ValidatorSet {
            validators,
            proposer: None,
            total_voting_power,
        };

        Self(Arc::new(Mutex::new(validator_set)))
    }

    fn validator(pub_key: &PublicKey, voting_power: i64) -> Validator {
        let hash = Keccak256::digest(pub_key.to_bytes().as_slice());
        // TODO: change the type with the [void_proto::void_proto::types::Address]
        //      also change in the proto definitions
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..]);

        let pub_key = void_proto::tendermint::crypto::PublicKey {
            sum: Some(void_proto::tendermint::crypto::public_key::Sum::Ed25519(
                pub_key.to_bytes().to_vec(),
            )),
        };

        Validator {
            address: address.to_vec(),
            pub_key: Some(pub_key),
            voting_power,
            proposer_priority: 0,
        }
    }

    /// get all the validators
    pub fn validators(&self) -> Vec<Validator> {
        self.0.lock().unwrap().validators.clone()
    }

    /// get total voting power for the Committee
    pub fn total_voting_power(&self) -> i64 {
        self.0.lock().unwrap().total_voting_power
    }

    // get the current proposer
    pub fn proposer(&self) -> Option<Validator> {
        self.0.lock().unwrap().proposer.clone()
    }

    #[cfg(test)]
    pub fn random_committee(size: usize) -> Self {
        const DEFAULT_VOTING_POWER: i64 = 100000;
        let mut validators = Vec::new();

        for _ in 0..size {
            let keypair = Keypair::generate();
            let pub_key = keypair.public();
            validators.push((pub_key, DEFAULT_VOTING_POWER));
        }

        Self::genesis(validators.into_iter())
    }
}

#[cfg(test)]
mod committee_tests {
    use super::*;

    #[test]
    fn test_total_power_calculation() {
        const DEFAULT_VOTING_POWER: i64 = 100000;
        // random commitee
        // each validator have DEFAULT_VOTING_POWER
        let committee = Committee::random_committee(3);

        assert_eq!(committee.total_voting_power(), 3 * DEFAULT_VOTING_POWER);
    }
}
