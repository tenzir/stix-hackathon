use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum STIXDomainObject {
    AttackPattern,
}
