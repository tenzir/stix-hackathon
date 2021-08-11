pub mod attack_pattern;
pub mod campaign;
pub mod course_of_action;
pub mod grouping;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum STIXDomainObject {
    #[serde(rename = "attack-pattern")]
    AttackPattern(attack_pattern::AttackPattern),
    #[serde(rename = "campaign")]
    Campain(campaign::Campaign),
    #[serde(rename = "course-of-action")]
    CourseOfAction(course_of_action::CourseOfAction),
    #[serde(rename = "grouping")]
    Grouping(grouping::Grouping),
}
