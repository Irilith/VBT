use crate::types::watchlist::WatchlistEntry;
use serde_json::Value;
use std::{collections::HashMap, fs::File};

pub fn load_watchlist() -> Result<Vec<WatchlistEntry>, String> {
    let file = File::open("watchlist.json")
        .map_err(|e| format!("Failed to open watchlist.json: {}", e))?;

    let json: Value = serde_json::from_reader(file)
        .map_err(|e| format!("Failed to parse watchlist.json: {}", e))?;

    let mut entries = Vec::new();

    for entry in json.as_array().unwrap() {
        let mut other_titles = HashMap::new();
        if let Some(other) = entry["other"].as_array() {
            for title in other {
                for key in &["native", "romaji", "english"] {
                    if let Some(value) = title[key].as_str() {
                        other_titles.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }

        entries.push(WatchlistEntry {
            name: entry["name"].as_str().unwrap().to_string(),
            cover: entry["cover"].as_str().unwrap().to_string(),
            other: other_titles,
        });
    }

    Ok(entries)
}
