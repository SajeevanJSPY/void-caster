mod behaviour;

use behaviour::VoidCasterP2p;
use vc_core::crypto::NodeId;

pub async fn start_p2p() -> eyre::Result<()> {
    let node_id = NodeId::new();
    let mut p2p = VoidCasterP2p::new(node_id)?;
    let _ = p2p.run().await;

    Ok(())
}
