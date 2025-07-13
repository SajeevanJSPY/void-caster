use std::{
    collections::{HashMap, HashSet},
    ops::Div,
    sync::{Arc, Mutex},
};

use void_proto::tendermint::{
    consensus::Vote,
    types::{BlockId, Validator},
};

use crate::{
    committee::Committee,
    state::{Height, Round},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoteType {
    PreVote,
    Precommit,
}

#[derive(Debug, Default)]
pub struct VoteTracker {
    // votes[height][round][vote_type][block_id]
    votes: Arc<
        Mutex<
            HashMap<Height, HashMap<Round, HashMap<VoteType, HashMap<BlockId, HashSet<Vec<u8>>>>>>,
        >,
    >,
}

impl VoteTracker {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_vote(
        &self,
        h: Height,
        r: Round,
        vote_type: VoteType,
        block_id: BlockId,
        address: Vec<u8>,
    ) {
        let mut lock = self.votes.lock().unwrap();

        let mut round_entry = lock
            .entry(h)
            .or_default()
            .entry(r)
            .or_default()
            .entry(vote_type)
            .or_default()
            .entry(block_id.clone())
            .or_default();

        round_entry.insert(address);
    }

    pub fn has_super_majority(
        &self,
        commitee: &Committee,
        h: Height,
        r: Round,
        vote_type: VoteType,
        block_id: BlockId,
    ) -> bool {
        let total_voting_power = commitee.total_voting_power();

        let mut lock = self.votes.lock().unwrap();
        let validators = commitee.validators().clone();

        if let Some(voted_validators) = lock
            .get(&h)
            .and_then(|rounds| rounds.get(&r))
            .and_then(|vt| vt.get(&vote_type))
            .and_then(|b| b.get(&block_id))
        {
            let voted_power = voted_validators
                .iter()
                .filter_map(|address| {
                    validators
                        .iter()
                        .find(|v| v.address == **address)
                        .map(|v| v.voting_power)
                })
                .sum();

            let threshold = (total_voting_power * 2) / 3 + 1;
            threshold <= voted_power
        } else {
            false
        }
    }
}

#[cfg(test)]
mod vote_tests {
    use crate::committee;

    use super::*;
    use vc_test_utils::test_tracing;
    use void_proto::tendermint::types::{BlockId, Validator};

    #[test]
    fn test_vote() {
        let vote = VoteTracker::new();

        let h = 1;
        let r = 1;
        let vote_type = VoteType::PreVote;
        let block_id = BlockId {
            hash: vec![32, 34, 64, 12, 54],
            part_set_header: None,
        };

        let committee = Committee::random_committee(5);
        let validators = committee.validators().clone();

        // voting
        vote.add_vote(
            h,
            r,
            vote_type,
            block_id.clone(),
            validators[0].address.clone(),
        );
        vote.add_vote(
            h,
            r,
            vote_type,
            block_id.clone(),
            validators[1].address.clone(),
        );
        vote.add_vote(
            h,
            r,
            vote_type,
            block_id.clone(),
            validators[2].address.clone(),
        );
        vote.add_vote(
            h,
            r,
            vote_type,
            block_id.clone(),
            validators[3].address.clone(),
        );

        assert!(vote.has_super_majority(&committee, h, r, vote_type, block_id));
    }

    fn test_duplicate_vote() {
        test_tracing();

        let vote = VoteTracker::new();

        let h = 1;
        let r = 1;
        let vote_type = VoteType::PreVote;
        let block_id = BlockId {
            hash: vec![32, 34, 64, 12, 54],
            part_set_header: None,
        };

        let committee = Committee::random_committee(5);
        let validators = committee.validators().clone();

        // voting
        vote.add_vote(
            h,
            r,
            vote_type,
            block_id.clone(),
            validators[0].address.clone(),
        );
        // again voting with the same validator
        vote.add_vote(
            h,
            r,
            vote_type,
            block_id.clone(),
            validators[0].address.clone(),
        );
        vote.add_vote(
            h,
            r,
            vote_type,
            block_id.clone(),
            validators[2].address.clone(),
        );
        vote.add_vote(
            h,
            r,
            vote_type,
            block_id.clone(),
            validators[3].address.clone(),
        );

        assert!(!vote.has_super_majority(&committee, h, r, vote_type, block_id));
    }
}
