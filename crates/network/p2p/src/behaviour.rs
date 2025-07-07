use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use futures::prelude::stream::StreamExt;
use libp2p::{
    StreamProtocol, Swarm, gossipsub, kad, noise,
    request_response::{self, ProtocolSupport},
    swarm::NetworkBehaviour,
    tcp, yamux,
};
use serde::{Deserialize, Serialize};
use vc_core::crypto::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusRequest(String);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusResponse(Vec<u8>);

#[derive(NetworkBehaviour)]
struct ConsenusBehaviour {
    kad: kad::Behaviour<kad::store::MemoryStore>,
    gossipsub: gossipsub::Behaviour,
    request_response: request_response::cbor::Behaviour<ConsensusRequest, ConsensusResponse>,
}

pub struct VoidCasterP2p {
    swarm: Swarm<ConsenusBehaviour>,
}

impl VoidCasterP2p {
    pub fn new(node_id: NodeId) -> eyre::Result<Self> {
        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_behaviour(|key| {
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(1))
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .message_id_fn(message_id_fn)
                    .build()
                    .expect("failed to configure gossipsub");

                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )
                .expect("failed to create gossippub behaviour");

                let request_response = request_response::cbor::Behaviour::new(
                    [(
                        StreamProtocol::new("/void-caster/p2p"),
                        ProtocolSupport::Full,
                    )],
                    request_response::Config::default(),
                );

                ConsenusBehaviour {
                    kad: kad::Behaviour::new(
                        node_id.peer_id(),
                        kad::store::MemoryStore::new(key.public().to_peer_id()),
                    ),
                    gossipsub,
                    request_response,
                }
            })?
            .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        swarm.behaviour_mut().kad.set_mode(Some(kad::Mode::Server));

        let topic = gossipsub::IdentTopic::new("void-caster");
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        Ok(Self { swarm })
    }

    pub async fn run(&mut self) -> eyre::Result<()> {
        use libp2p::gossipsub::Event::*;
        use libp2p::swarm::SwarmEvent::*;

        // Listen on all interfaces and whatever port the OS assigns
        self.swarm
            .listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => match event {
                    Behaviour(ConsenusBehaviourEvent::Gossipsub(Subscribed { .. })) => {
                        println!("subscribed");
                    },
                    NewListenAddr { listener_id, .. } => {
                        println!("listening with the id: {listener_id}");
                    }
                    _ => {}
                }

            } // select
        }
    }
}
