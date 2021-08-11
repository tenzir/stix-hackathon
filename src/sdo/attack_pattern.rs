use crate::common::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackPattern {
    external_references: Option<List<ExternalReference>>,
    name: String,
    description: Option<String>,
    aliases: Option<List<String>>,
    kill_chain_phases: Option<List<KillChainPhase>>,
}
