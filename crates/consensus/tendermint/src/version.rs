use void_proto::tendermint::{state::Version as TmVersion, version::Consensus};

const APP_VERSION: &str = "void-caster:0.0.1";
const BLOCK_PROTOCOL: u64 = 11;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Version(TmVersion);

impl Version {
    pub fn new() -> Self {
        Self(TmVersion {
            consensus: Some(Consensus {
                block: BLOCK_PROTOCOL,
                app: 0,
            }),
            software: APP_VERSION.to_string(),
        })
    }

    pub fn get_block_protocol(&self) -> u64 {
        self.0.consensus.unwrap().block
    }

    pub fn app_version(&self) -> u64 {
        self.0.consensus.unwrap().app
    }

    pub fn get_software(&self) -> String {
        self.0.software.clone()
    }

    pub fn inner(&self) -> TmVersion {
        self.0.clone()
    }
}
