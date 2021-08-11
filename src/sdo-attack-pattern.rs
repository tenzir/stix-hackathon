use common_types::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AttackPattern {
    type: String,
    external_references: Option<List<ExternalReference>>,
    name: String,
    description: Option<String>,
    aliases: Option<List<String>>,
    kill_chain_phases: Option<List<KillChainPhase>>,
}
