pub mod common;
pub mod sdo;

pub fn main() {
    let attack_pattern_json = "{\n\t\"type\": \"attack-pattern\",\n\t\"external_references\": null,\n\t\"name\": \"Some Attack Pattern\",\n\t\"description\": null,\n\t\"aliases\": null,\n\t\"kill_chain_phase\": null\n}";
    let campaign_json = "{ \"type\": \"campaign\", \"spec_version\": \"2.1\", \"id\": \"campaign--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f\", \"created_by_ref\": \"identity--f431f809-377b-45e0-aa1c-6a4751cae5ff\", \"created\": \"2016-04-06T20:03:00.000Z\", \"modified\": \"2016-04-06T20:03:00.000Z\", \"name\": \"Green Group Attacks Against Finance\", \"description\": \"Campaign by Green Group against a series of targets in the financial services sector.\" }";
    let grouping_json = r#"{
  "type": "grouping",
  "spec_version": "2.1",
  "id": "grouping--84e4d88f-44ea-4bcd-bbf3-b2c1c320bcb3",
  "created_by_ref": "identity--a463ffb3-1bd9-4d94-b02d-74e4f1658283",
  "created": "2015-12-21T19:59:11.000Z",
  "modified": "2015-12-21T19:59:11.000Z",
  "name": "The Black Vine Cyberespionage Group",
  "description": "A simple collection of Black Vine Cyberespionage Group attributed intel",
  "context": "suspicious-activity",
  "object_refs": [
    "indicator--26ffb872-1dd9-446e-b6f5-d58527e5b5d2",
    "campaign--83422c77-904c-4dc1-aff5-5c38f3a2c55c",
    "relationship--f82356ae-fe6c-437c-9c24-6b64314ae68a",
    "file--0203b5c8-f8b6-4ddb-9ad0-527d727f968b"
  ]
}"#;

    let course_of_action = r#"{
  "type": "course-of-action",
  "spec_version": "2.1",
  "id": "course-of-action--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
  "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
  "created": "2016-04-06T20:03:48.000Z",
  "modified": "2016-04-06T20:03:48.000Z",
  "name": "mitigation-poison-ivy-firewall",
  "description": "This action points to a recommended set of steps to respond to the Poison Ivy malware on a Cisco firewall device",
  "action_type": "cisco:ios",
  "action_reference":
    { "source_name": "internet",
      "url": "hxxps://www.stopthebad.com/poisonivyresponse.asa"
    }
}"#;

    println!(
        "{:?}",
        serde_json::from_str::<sdo::STIXDomainObject>(attack_pattern_json)
    );
    println!(
        "{:?}",
        serde_json::from_str::<sdo::STIXDomainObject>(campaign_json)
    );
    println!(
        "{:?}",
        serde_json::from_str::<sdo::STIXDomainObject>(grouping_json)
    );

    println!(
        "{:?}",
        serde_json::from_str::<sdo::STIXDomainObject>(course_of_action)
    );
}
