use crate::common::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Grouping {
    // Common
    spec_version: String,
    id: Identifier,
    created: Timestamp,
    modified: Timestamp,
    // Concrete
    name: Option<String>,
    description: Option<String>,
    context: OpenVocab,
    object_refs: List<Identifier>,
}
