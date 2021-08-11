use crate::common::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GroupingUnvalidated {
    // Required Common Properties
    spec_version: String,
    id: Identifier,
    created: Timestamp,
    modified: Timestamp,

    // Optional Common Properties
    created_by_ref: Option<Identifier>,
    revoked: Option<Boolean>,
    labels: Option<List<String>>,
    confidence: Option<Integer>,
    lang: Option<String>,
    external_references: Option<List<ExternalReference>>,
    object_marking_refs: Option<List<Identifier>>,
    granular_markings: Option<List<GranularMarking>>,
    defanged: Option<Boolean>,
    extensions: Option<Dictionary>,

    // Grouping Specific Properties
    name: Option<String>,
    description: Option<String>,
    context: OpenVocab,
    object_refs: List<Identifier>,
}

// The 'grouping-context-ov' vocabulary.
const GROUPING_CONTEXT_VOCABULARY: &'static [&'static str] =
    &["suspicious-activity", "malware-analysis", "unspecified"];

#[derive(Debug, Deserialize, Serialize)]
#[serde(try_from = "GroupingUnvalidated")]
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
    confidence: Option<Integer>,
    lang: Option<String>,
    external_references: Option<List<ExternalReference>>,
    object_marking_refs: Option<List<Identifier>>,
    granular_markings: Option<List<GranularMarking>>,
    defanged: Option<Boolean>,
    extensions: Option<Dictionary>,

    // Grouping Specific Properties
    name: Option<String>,
    description: Option<String>,
    context: OpenVocab,
    object_refs: List<Identifier>,
}

pub struct Stix2ValidationError {
    msg: String,
}

// The error type has to implement Display
impl std::fmt::Display for Stix2ValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(formatter, "Invalid STIX2 object: {}", self.msg);
    }
}

impl std::convert::TryFrom<GroupingUnvalidated> for Grouping {
    type Error = Stix2ValidationError;
    fn try_from(grouping: GroupingUnvalidated) -> Result<Self, Self::Error> {
        let GroupingUnvalidated {
            spec_version,
            id,
            created,
            modified,
            created_by_ref,
            revoked,
            labels,
            confidence,
            lang,
            external_references,
            object_marking_refs,
            granular_markings,
            defanged,
            extensions,
            name,
            description,
            context,
            object_refs,
        } = grouping;

        if !defanged.is_none() {
            return Err(Stix2ValidationError {
                msg: "Common property 'defanged' is not applicable to 'Grouping' object"
                    .to_string(),
            });
        }

        // if !GROUPING_CONTEXT_VOCABULARY.iter().find(|&&x| x == &context) {
        //     return Err(Stix2ValidationError {msg: "Invalid context value"});
        // }

        Ok(Grouping {
            spec_version,
            id,
            created,
            modified,
            created_by_ref,
            revoked,
            labels,
            confidence,
            lang,
            external_references,
            object_marking_refs,
            granular_markings,
            defanged,
            extensions,
            name,
            description,
            context,
            object_refs,
        })
    }
}
