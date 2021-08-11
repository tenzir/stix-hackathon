pub mod common;
pub mod sdo;

pub fn main() {
    let attack_pattern_json = "{\n\t\"type\": \"attack-pattern\",\n\t\"external_references\": null,\n\t\"name\": \"Some Attack Pattern\",\n\t\"description\": null,\n\t\"aliases\": null,\n\t\"kill_chain_phase\": null\n}";

    println!(
        "{:?}",
        serde_json::from_str::<sdo::STIXDomainObject>(attack_pattern_json)
    );
}
