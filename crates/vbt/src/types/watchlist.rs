use std::collections::HashMap;

#[derive(Debug)]
pub struct WatchlistEntry {
    pub name: String,
    pub cover: String,
    pub other: HashMap<String, String>,
}
