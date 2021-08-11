use crate::common::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Campaign {
    name: String,
    description: Option<String>,
    aliases: Option<List<String>>,
    first_seen: Option<Timestamp>,
    last_seen: Option<Timestamp>,
    objective: Option<String>,
}
