use crate::types::book::BookRow;
use serde_json::json;

pub fn return_json(
    query: &str,
    bookrow: &Vec<BookRow>,
) -> Result<String, Box<dyn std::error::Error>> {
    let data = json!({
        "query": query,
        "rows": bookrow,
    });

    let json_str = serde_json::to_string_pretty(&data)?;
    Ok(json_str)
}
