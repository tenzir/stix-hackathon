use crate::common::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseOfAction {
    name: String,
    description: Option<String>,
    action_type: Option<OpenVocab>,
    os_execution_envs: Option<List<String>>,
    action_bin: Option<Binary>,
    action_reference: Option<ExternalReference>,
}
