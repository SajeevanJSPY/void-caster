//! Maintains the consensus state of a node actively participating in Tendermint rounds.
//! Includes local view of height, round, step, locked and valid values.

pub use crate::types::{Height, Round};

// TODO: change with the real Block structure
pub type Block = Vec<Vec<u8>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    Propose,
    Prevote,
    Precommit,
}

#[derive(Debug)]
pub struct ConsensusState {
    pub current_height: Height,
    pub current_round: Round,
    pub current_step: Step,
    pub locked_value: Option<Block>,
    pub locked_round: Option<Round>,
    pub valid_value: Option<Block>,
    pub valid_round: Option<Round>,
    pub decisions: Vec<Block>,
}

impl ConsensusState {
    pub(crate) fn get_current_height(&self) -> Height {
        self.current_height
    }

    pub(crate) fn get_current_round(&self) -> Round {
        self.current_round
    }

    pub(crate) fn get_current_step(&self) -> Step {
        self.current_step
    }

    pub(crate) fn get_valid_round(&self) -> Option<Round> {
        self.valid_round
    }

    pub(crate) fn get_valid_value(&self) -> Option<Block> {
        self.valid_value.clone()
    }

    pub(crate) fn get_locked_round(&self) -> Option<Round> {
        self.valid_round
    }

    pub(crate) fn get_locked_value(&self) -> Option<Block> {
        self.valid_value.clone()
    }

    pub(crate) fn set_current_height(&mut self, h: Height) {
        self.current_height = h;
    }

    pub(crate) fn set_current_round(&mut self, r: Round) {
        self.current_round = r;
    }

    pub(crate) fn set_current_step(&mut self, s: Step) {
        self.current_step = s;
    }

    pub(crate) fn set_valid_round(&mut self, r: Option<Round>) {
        self.valid_round = r;
    }

    pub(crate) fn set_valid_value(&mut self, value: Option<Block>) {
        self.valid_value = value;
    }

    pub(crate) fn set_locked_round(&mut self, r: Option<Round>) {
        self.locked_round = r;
    }

    pub(crate) fn set_locked_value(&mut self, value: Option<Block>) {
        self.locked_value = value;
    }
}
