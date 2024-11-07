use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRow {
    pub stt: String,
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub translator: String,
    pub quantity: String,
    pub self_published: String,
    pub partner: String,
    pub registration_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct BookEntry {
    pub pub_date: String,
    pub queries: Vec<String>,
}
