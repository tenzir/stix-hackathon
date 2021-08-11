pub mod attack_pattern;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum STIXDomainObject {
    #[serde(rename = "attack-pattern")]
    AttackPattern(attack_pattern::AttackPattern),
}
