pub mod common;
pub mod sdo;

pub fn main() {
    let attack_pattern_json = "{\n\t\"type\": \"attack-pattern\",\n\t\"external_references\": null,\n\t\"name\": \"Some Attack Pattern\",\n\t\"description\": null,\n\t\"aliases\": null,\n\t\"kill_chain_phase\": null\n}";
    let campaign_json = "{ \"type\": \"campaign\", \"spec_version\": \"2.1\", \"id\": \"campaign--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f\", \"created_by_ref\": \"identity--f431f809-377b-45e0-aa1c-6a4751cae5ff\", \"created\": \"2016-04-06T20:03:00.000Z\", \"modified\": \"2016-04-06T20:03:00.000Z\", \"name\": \"Green Group Attacks Against Finance\", \"description\": \"Campaign by Green Group against a series of targets in the financial services sector.\" }";

    println!(
        "{:?}",
        serde_json::from_str::<sdo::STIXDomainObject>(campaign_json)
    );
}

