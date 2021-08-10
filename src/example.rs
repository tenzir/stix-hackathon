use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Nationality {
    NL,
    DE
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
    nationality: Nationality,
    age: u8,
    phones: Vec<String>,
}
