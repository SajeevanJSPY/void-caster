use tendermint_proto::types::{Validator, ValidatorSet};

use vc_types::crypto::{Keypair, PublicKey};

use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct Committee(ValidatorSet);

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

        Self(validator_set)
    }

    fn validator(pub_key: &PublicKey, voting_power: i64) -> Validator {
        let mut hasher = DefaultHasher::new();
        pub_key.hash(&mut hasher);
        let hash = hasher.finish();
        let address = hash.to_be_bytes().to_vec();

        let pub_key = tendermint_proto::crypto::PublicKey {
            sum: Some(tendermint_proto::crypto::public_key::Sum::Ed25519(
                pub_key.to_bytes().to_vec(),
            )),
        };

        Validator {
            address,
            pub_key: Some(pub_key),
            voting_power,
            proposer_priority: 0,
        }
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
