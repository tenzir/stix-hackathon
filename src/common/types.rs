use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Identifier = String;
pub type List<T> = Vec<T>;
pub type Hashes = HashMap<String, String>;
pub type KillChainPhase = String; // TODO
pub type OpenVocab = String; // TODO
pub type Timestamp = String; // TODO
pub type GranularMarking = String; // TODO
pub type Integer = i64;
pub type Binary = String;
pub type Boolean = bool;
pub type Dictionary = String; // TODO

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalReference {
    source_name: String,
    description: Option<String>,
    url: Option<String>,
    hashes: Option<Hashes>,
    external_id: Option<String>,
}
