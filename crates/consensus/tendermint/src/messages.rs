//! messages that are sent between nodes

use prost::Message as _;
use void_proto::google::Timestamp;
use void_proto::tendermint::consensus::Message as TmMessage;
use void_proto::tendermint::consensus::message::Sum as Messages;

use crate::types::{Height, Round};

pub struct Message(TmMessage);

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        self.0.encode_to_vec()
    }

    pub fn inner(self) -> TmMessage {
        self.0
    }

    pub fn proposal(height: Height, round: Round, pol_round: Round) -> Self {
        let proposal = void_proto::tendermint::types::Proposal {
            r#type: void_proto::tendermint::types::SignedMsgType::Proposal as i32,
            height,
            round,
            pol_round,
            // NOTE: omit the other fields for now
            ..Default::default()
        };

        Self(TmMessage {
            sum: Some(Messages::Proposal(
                void_proto::tendermint::consensus::Proposal {
                    proposal: Some(proposal),
                },
            )),
        })
    }

    pub fn proposal_pol(height: Height, proposal_pol_round: Round) -> Self {
        let proposal_pol = void_proto::tendermint::consensus::ProposalPol {
            height,
            proposal_pol_round,
            // NOTE: omit the other fields for now
            ..Default::default()
        };
        Self(TmMessage {
            sum: Some(Messages::ProposalPol(proposal_pol)),
        })
    }

    pub fn vote(
        height: Height,
        round: Round,
        timestamp: Timestamp,
        validator_address: Vec<u8>,
    ) -> Self {
        let vote = void_proto::tendermint::types::Vote {
            r#type: void_proto::tendermint::types::SignedMsgType::Prevote as i32,
            height,
            round,
            timestamp: Some(timestamp),
            validator_address,
            // NOTE: omit the other fields for now
            ..Default::default()
        };

        Self(TmMessage {
            sum: Some(Messages::Vote(void_proto::tendermint::consensus::Vote {
                vote: Some(vote),
            })),
        })
    }

    pub fn vote_not_valid() -> Self {
        Self(TmMessage {
            sum: Some(Messages::Vote(void_proto::tendermint::consensus::Vote {
                vote: None,
            })),
        })
    }
}
