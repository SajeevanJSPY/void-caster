use std::collections::{BTreeMap, HashMap};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::debug;

use vc_core::crypto::{Digest, Hash, NodeId};
use vc_types::crypto::PublicKey;

use crate::{
    config::{Authority, Committee},
    messages::Certificate,
    types::Round,
};

type Dag = HashMap<Round, HashMap<PublicKey, (Digest, Certificate)>>;

struct GlobalState {
    pub last_committed_round: Round,
    pub last_committed: HashMap<PublicKey, Round>,
    pub dag: Dag,
}

impl GlobalState {
    pub fn new(genesis: Vec<Certificate>) -> Self {
        let genesis = genesis
            .into_iter()
            .map(|x| (x.origin(), (x.digest(), x)))
            .collect::<HashMap<_, _>>();

        Self {
            last_committed_round: 0,
            last_committed: genesis
                .iter()
                .map(|(x, (_, y))| (x.clone(), y.round()))
                .collect(),
            dag: [(0, genesis)].iter().cloned().collect(),
        }
    }
}

pub struct Consensus {
    pub committee: Committee,
    pub gc_depth: Round,
    pub rx_primary: Receiver<Certificate>,
    pub tx_primary: Sender<Certificate>,
    pub genesis: Vec<Certificate>,
}

impl Default for Consensus {
    fn default() -> Self {
        let mut committee = Committee {
            authorities: BTreeMap::new(),
        };

        committee
            .authorities
            .insert(NodeId::new().public_key(), Authority { stake: 10000 });

        let (tx_primary, rx_primary) = tokio::sync::mpsc::channel(100);

        Self {
            committee,
            gc_depth: 50,
            rx_primary,
            tx_primary,
            genesis: vec![],
        }
    }
}

impl Consensus {
    pub async fn run(&mut self) {
        let mut state = GlobalState::new(self.genesis.clone());

        // TODO: not completed yet
        while let Some(certificate) = self.rx_primary.recv().await {
            debug!("Processing {:?}", certificate);
            let round = certificate.round();

            // Add a new certificate to the local storage.
            state
                .dag
                .entry(round)
                .or_insert_with(HashMap::new)
                .insert(certificate.origin(), (certificate.digest(), certificate));

            // Try to order the dag to commit, Start from the highest round for which we have at
            // least 2f+1 certificates. This is because we need them to reveal the common coin
            let r = round - 1;

            if r % 2 != 0 || r < 4 {
                continue;
            }

            // Get the certificate's digest of the leader of round r-2. If we already ordered this leader,
            // there is nothing to do.
            let leader_round = r - 2;
            if leader_round <= state.last_committed_round {
                continue;
            }
        }
    }
}
