pub mod crypto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Consensus {
    Tendermint,
    Bullshark,
}
