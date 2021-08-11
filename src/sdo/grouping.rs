use crate::common::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Grouping {
    // Required Common Properties
    spec_version: String,
    id: Identifier,
    created: Timestamp,
    modified: Timestamp,

    // Optional Common Properties
    created_by_ref: Option<Identifier>,
    revoked: Option<Boolean>,
    labels: Option<List<String>>,
    confidence: Integer,
    lang: String,
    external_references: List<ExternalReference>,
    object_marking_refs: List<Identifier>,
    granular_markings: List<GranularMarking>,
    extensions: Dictionary,

    // Grouping Specific Properties
    name: Option<String>,
    description: Option<String>,
    context: OpenVocab,
    object_refs: List<Identifier>,
}
