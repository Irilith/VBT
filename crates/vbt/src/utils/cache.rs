use crate::types::book::BookRow;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

#[derive(Serialize, Deserialize)]
pub struct Cache<T> {
    pub data: HashMap<String, T>,
    pub title_build_date: HashMap<String, T>,
}

pub fn generate_cache_key(book: &BookRow) -> String {
    format!("{}-{}", book.isbn, book.title)
}

pub fn load_cache<T: for<'de> Deserialize<'de>>(path: &str) -> Cache<T> {
    if let Ok(file) = File::open(path) {
        if let Ok(cache) = serde_json::from_reader(file) {
            return cache;
        }
    }
    Cache {
        data: HashMap::new(),
        title_build_date: HashMap::new(),
    }
}

pub fn save_cache<T: Serialize>(cache: &Cache<T>, path: &str) -> Result<(), String> {
    let file = File::create(path).map_err(|e| format!("Failed to create cache file: {}", e))?;

    serde_json::to_writer_pretty(file, cache)
        .map_err(|e| format!("Failed to write cache: {}", e))?;

    Ok(())
}
