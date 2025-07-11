use tendermint_proto::mempool::Message as MempoolMessage;
use tendermint_proto::{
    state::{State as TmState, Version},
    types::Validator,
};

use crate::messages::Message;
use crate::{
    state::{Block, ConsensusState, Step},
    types::Round,
};

const CHAIN_ID: &str = "void-caster";

#[derive(Debug)]
pub struct Consensus {
    pub authority: Validator,
    pub state: ConsensusState,
    pub meta_state: State,
    pub mempool: MempoolMessage,
}

impl Consensus {
    pub async fn run(&mut self) {
        // start the first round for the first height
        self.start_round(0).await;
    }

    /// start the round for the new height
    async fn start_round(&mut self, r: Round) {
        // set the current round and step for the new height
        self.state.set_current_round(r);
        self.state.set_current_step(Step::Propose);

        // if the node chosen to propose the value
        if self.get_round_proposer(r) == self.authority {
            let proposal = self.state.get_valid_value().unwrap_or(self.get_value());

            self.broadcast(Message::proposal(
                self.state.get_current_height(),
                self.state.get_current_round(),
                -1,
            ));
        }
    }

    fn broadcast(&self, m: Message) {}

    fn get_value(&self) -> Block {
        todo!()
    }

    /// get the Proposer for the current round
    fn get_round_proposer(&self, r: Round) -> Validator {
        todo!()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct State(pub TmState);

impl State {
    pub fn new() -> Self {
        let state = TmState {
            version: Some(Version {
                software: "void-caster".to_string(),
                consensus: None,
            }),
            chain_id: CHAIN_ID.to_string(),
            initial_height: 0,
            ..Default::default()
        };

        Self(state)
    }
}
