use serde::{Deserialize, Serialize};

pub type Identifier = String;
pub type List<T> = Vec<T>;
pub type KillChainPhase = String;
pub type OpenVocab = String;
pub type Timestamp = String;
pub type Boolean = bool;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalReference {
    source_name: String,
    description: Option<String>,
    url: Option<String>,
    hashes: Option<String>,
    external_id: Option<String>
}

pub type GranularMarking = String;
pub type Integer = i64;
pub type Dictionary = String;
