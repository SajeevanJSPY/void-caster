use std::collections::BTreeMap;

use crate::types::Stake;
use vc_types::crypto::PublicKey;

#[derive(Debug, Clone)]
pub struct Authority {
    pub stake: Stake,
}

#[derive(Debug, Clone)]
pub struct Committee {
    pub authorities: BTreeMap<PublicKey, Authority>,
}
