/*

                     Tx
    Initial State ------->

*/

pub type Tx = Vec<u8>;
pub type VotingPower = u128;
pub type Height = i64;
pub type Round = i32;
pub type Nonce = u64;
pub type Amount = u128;

/// value that has been decided by the consensus process
pub struct Block {
    height: Height,
    round: Round,
    /// the transactions needs to be ensured before committing,
    /// which will be performed in value valid check
    transactions: Vec<Tx>
}

/// Global State for performing Consensus
/// These Consensus ultimately return decision values, which will be eventually saved to the disk
///     the changes made in the process, won't be written until the GlobalState is committed
pub struct GlobalState {
	/// current block height (starting from 0), Incremented when a block is committed
    current_height: Height,

	/// current round number in consensus at this height, incremented when consensus fails in one round
    current_round: Round,

	/// current step in the round
    step: Step,

	/// the value the node is locked on (previously committed)
    locked_value: Option<Value>,

	/// the round in which the validator locked its value, None if never locked
    locked_round: Option<Round>,

	/// the latest value known to be potential valid (received 2f+1 prevotes)
    valid_value: Option<Value>,

	/// the round which [GlobalState.valid_value] has seen, if not None
    valid_round: Option<Round>,

    // TODO: replace this with the DB, which stores to the disk
    /// assume the value for the decision is Block
	/// the final decided block value at height h, once consensus is reached
    /// this should be stored to the disk
    decisions: Vec<Block>
}

impl GlobalState {
    /// initialize the GlobalState with the appropriate values and genesis values
    /// some of the values can be get from the Genesis, config files
    pub fn new(genesis: Genesis) -> Self {
        // initialize the values
        Self {
            current_height: 0,
            current_round: 0,
            step: Step::Propose,
            locked_value: None,
            locked_round: None,
            valid_value: None,
            valid_round: None,
            decisions: vec![]
        }
    }

    /// after the initialization, this function should be called, for the node to be participate in
    /// Consensus
    ///     state change triggers:
    ///         timeouts: timeoutPropose, timeoutPrevote, timeoutPrecommit
    ///         messages from the other nodes: broadcast
    pub async fn run(&self) {
        // start the first round for the first height
        self.start_round(0);

        // listen to the upon rules

    }

    /// start the round for the next height
    fn start_round(&self, r: Round) {
        self.current_round = r;
        self.step = Step::Propose;

        // get the proposal value from the `valid_value` or from the `get_value()` function
        let proposal = Block::default();

        // if our node is the leader, propose the Value
        if self.proposer(self.height, r) == self.us() {
            // create a new block for the new height
            // get the transactions from the Mempool, and validate the transactions
            // typical Mempool normally validate the transaction
            proposal = self.valid_value.unwrap_or(self.get_value());

            // broadcast this message to other Node
            self.broadcast(Message::Propose, height, round, proposal, valid_round);
        } else {
            tokio::time::interval(self.timeout_propose(r));
            // this will be executed after the duration `timeout_propose()` ended
            // wait for a period of time to get the propose
            self.on_timeout_propose(self.current_height, self.current_round);
        }
    }

    pub fn us(&self) -> Proposer {}

    // create a new block with transactions, and necessary information required by the chain
    pub fn get_value(&self, h: Height, r: Round) -> Block {
        todo!();
    }


    //=== Upon Rules
    //  Proposal - height, round, value, valid round
    //  Precommit - height, round, hash of the value
    //  Prevote - height, round, hash of the value

    /// this function has to be called, when the proposer broadcasted the proposal
    /// (PROPOSAL, h-p, round-p, v, -1) from proposer(h-p, round-p)
    pub fn on_proposing_proposal(&self, h: Height, r: Round, v: Value, valid_round: Option<Round> /* TODO: valid round is not set yet */) {
        // TODO: validate the proposer signature
        assert!(valid_round.is_none());
        assert!(self.step.is_propose());
        if (self.round != r || self.height != h) {
            return;
        }

        if self.is_valid_value(v) && (self.locked_round.is_none() || self.locked_value == v) {
            broadcast(Message::Prevote, h, r, Some(Value.hash()))
        } else {
            broadcast(Message::Prevote, h, r, None)
        }
        self.step = Step::Prevote;
    }

    /// (PROPOSAL, h-p, round-p, v, vr) from proposer(h-p, round-p) and 2f+1 (PREVOTE, h-p, vr, id(v))
    pub fn on_receive_proposal_with_valid_round(&self, h: Height, r: Round, v: Value, v_r: Option<Round>) {
        // TODO: validate the proposer signature
        // TODO: verify the voting of 2f+1 votes, only execute this function if the votes are 2f+1
        assert!(self.step == Step::Propose);
        assert(v_r.is_some());
        let valid_round = v_r.unwrap();
        assert!(self.locked_round.unwrap() <= v_r || self.locked_value() == v);
        if (self.round != r || self.height != h) {
            return;
        }

        if self.is_valid_value(v) && (self.locked_round <= valid_round || self.locked_value == v) {
            broadcast(Message::Prevote, self.current_height, self.current_round, Some(Value.hash()));
        } else {
            broadcast(Message::Prevote, self.current_height, self.current_round, None);
        }

        self.step = Step::Prevote;
    }

    /// call this function for the first time after step == Prevote
    /// this function should be called after the timeout period of prevote has ended `timeoutPrevote`
    /// 2f+1 (PREVOTE, h-p, round-p, *) for the first time
    pub fn on_prevote_firsttime(&self, h: Height, r: Round) {
        assert!(self.step == Step::Prevote);
        // TODO: verify the voting of 2f+1 votes, and execute this function
        tokio::time::Sleep(Self::timeout_prevote());
        self.on_timeout_prevote(h, r);
    }

    /// (PROPOSAL, h-p, round-p, v, *) from proposer and 2f+1 (PREVOTE, h-p, round-p, id(v))
    pub fn on_proposal_and_prevote(&self, h: Height, r: Round, v: Value) {
        // check this proposal came from the proposer for this height
        // and also check the votes

        assert!(self.is_valid_value(v) && self.step >= Step::Prevote /* which means Prevote or Precommit */);

        if self.step == Step::Prevote {
            self.locked_value = Some(v);
            self.locked_round = Some(r);
            broadcast(Message::Precommit, h, r, id(v));
            self.step = Step::Precommit;
        }
        self.valid_value = v;
        self.valid_round = r;
    }

    /// 2f+1 (PREVOTE, h-p, round-p, nil)
    pub fn on_prevote(&self, h: Height, r: Round, what: Option<Value>) {
        assert!(self.step == Step::Prevote);
        self.broadcast(Message::Precommit, h, r, None);
        self.step = Step::Precommit;
    }

    /// call this function for the first time after step == Precommit
    /// this function should be called after the timeout period of prevote has ended `timeoutPrecommit`
    /// 2f+1 (PRECOMMIT, h-p, round-p, *)
    pub fn on_precommit(&self, h: Height, r: Round) {
        // TODO: verify the voting of 2f+1 votes, and execute this function
        tokio::time::Sleep(Self::timeout_precommit());
        self.on_timeout_precommit(h, r);
    }

    /// (PROPOSAL, h-p, r, v, *) from proposer(h-p, r) and 2f+1 (PRECOMMIT, h-p, r, id(v))
    pub fn on_proposal_and_precommit(&self, h: Height, r: Round, v: Value) {
        assert!(self.decisions[h].is_none());

        if self.is_valid_value(v) {
            decisions.insert(h, v);
            self.current_height = h + 1;
            // reset locked_round, locked_value, valid_round, valid_value to initial values
            self.locked_round = None;
            self.locked_value = None;
            self.valid_round = None;
            self.valid_value = None;
        }
    }

    /// f+1 (*, h-p, round, *, *)
    pub fn on_vote_failures(h: Height, r: Round) {
        assert!(r > self.round);
        self.start_round(r);
    }

    pub fn on_timeout_propose(&mut self, h: Height, r: Round) {
        if self.current_height != h || self.current_round != r || self.step != Step::Propose {
            return;
        }
        self.broadcast(Message::Prevote, h, r, None);
        self.step = Step::Prevote;
    }

    pub fn on_timeout_prevote(&mut self, h: Height, r: Round) {
        if self.current_height != h || self.current_round != r || self.step == Step::Prevote {
            return;
        }

        self.broadcast(Message::Precommit, h, r, None);
        self.step == Step::Precommit;
    }

    pub fn on_timeout_precommit(&self, h: Height, r: Round) {
        if self.current_height != h || self.current_round != r {
            return;
        }

        self.start_round(r + 1);
    }

    /// validate the values that are sent through the proposal
    /// normally it would be block of transactions
    pub fn is_valid_value(&self, v: Value) -> bool {
    }

    /// returning the proposer for the round in the consensus instance h
    pub fn proposer(&self, h: Height, r: Round) -> Proposer {}

    // voting related functionalities
    pub fn total_power(validator_set: ValidatorSet) -> u64 {
        validator_set.total_voting_power
    }

    pub fn quorum_threshold() -> u64 {
        todo!();
    }

    pub fn validity_threshold() -> u64 {
        todo!();
    }
}

pub struct Consensus {
    state: GlobalState
}

#[derive(Debug, Clone)]
pub struct Proposer {}


#[derive(Debug, Clone)]
pub enum Message {
    Proposal,
    Prevote,
    Precommit
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Propose,
    Prevote,
    Precommit
}

