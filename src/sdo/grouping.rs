use crate::common::types::*;
use crate::common::properties::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Grouping {
    #[serde(flatten)]
    common: CommonProperties,
    name: Option<String>,
    description: Option<String>,
    context: OpenVocab,
    object_refs: List<Identifier>,
}

impl AsRef<CommonProperties> for Grouping {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
