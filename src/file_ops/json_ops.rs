use crate::utils::fs::ensure_parent_dir;
use std::{
    fs::File,
    io::{Read, Write},
};

pub fn save_json(data: &str, path: &str) -> Result<(), String> {
    ensure_parent_dir(path)?;
    File::create(path)
        .map_err(|e| format!("Failed to create {}: {}", path, e))?
        .write_all(data.as_bytes())
        .map_err(|e| format!("Failed to write to {}: {}", path, e))?;

    Ok(())
}

pub fn read_json(path: &str) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| format!("Failed to open {}: {}", path, e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    Ok(contents)
}
