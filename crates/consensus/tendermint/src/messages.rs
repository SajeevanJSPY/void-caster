//! messages that are sent between nodes

use prost::Message as _;
use tendermint_proto::consensus::message::Sum as Messages;
use tendermint_proto::{consensus::Message as TmMessage, google::protobuf::Timestamp};

use crate::types::{Height, Round};

pub struct Message(TmMessage);

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        self.0.encode_to_vec()
    }

    pub fn proposal(height: Height, round: Round, pol_round: Round) -> Self {
        let proposal = tendermint_proto::types::Proposal {
            r#type: tendermint_proto::types::SignedMsgType::Proposal as i32,
            height,
            round,
            pol_round,
            // NOTE: omit the other fields for now
            ..Default::default()
        };

        Self(TmMessage {
            sum: Some(Messages::Proposal(tendermint_proto::consensus::Proposal {
                proposal: Some(proposal),
            })),
        })
    }

    pub fn proposal_pol(height: Height, proposal_pol_round: Round) -> Self {
        let proposal_pol = tendermint_proto::consensus::ProposalPol {
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
        let vote = tendermint_proto::types::Vote {
            r#type: tendermint_proto::types::SignedMsgType::Prevote as i32,
            height,
            round,
            timestamp: Some(timestamp),
            validator_address,
            // NOTE: omit the other fields for now
            ..Default::default()
        };

        Self(TmMessage {
            sum: Some(Messages::Vote(tendermint_proto::consensus::Vote {
                vote: Some(vote),
            })),
        })
    }
}
