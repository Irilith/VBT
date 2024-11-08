use crate::types::book::BookRow;
use reqwest::Client;
use scraper::{Html, Selector};

async fn get_data_fetch(query: &str, page: u8) -> Result<String, Box<dyn std::error::Error>> {
    let query_clean = query.trim_end().replace(" ", "+");
    // https://ppdvn.gov.vn/web/guest/ke-hoach-xuat-ban?query=H%C3%A0nh+Tr%C3%ACnh+C%E1%BB%A7a+Elaina&p=1
    let url = format!(
        "https://ppdvn.gov.vn/web/guest/ke-hoach-xuat-ban?query={}&p={}",
        query_clean, page
    );
    let client = Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}

pub async fn extract_table_data(query: &str) -> Result<Vec<BookRow>, Box<dyn std::error::Error>> {
    let html = get_data_fetch(query, 1).await?;
    let document = Html::parse_document(&html);

    let selectors = (
        Selector::parse("#list_data_return table")
            .map_err(|e| format!("Invalid table selector: {}", e))?,
        Selector::parse("tbody tr").map_err(|e| format!("Invalid row selector: {}", e))?,
        Selector::parse("td").map_err(|e| format!("Invalid cell selector: {}", e))?,
    );

    let table = document
        .select(&selectors.0)
        .next()
        .ok_or("Table not found")?;

    let rows = table
        .select(&selectors.1)
        .filter_map(|row| {
            let cells: Vec<String> = row
                .select(&selectors.2)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();

            if cells.len() == 9 {
                Some(BookRow {
                    stt: cells[0].clone(),
                    isbn: cells[1].clone(),
                    title: cells[2].clone(),
                    author: cells[3].clone(),
                    translator: cells[4].clone(),
                    quantity: cells[5].clone(),
                    self_published: cells[6].clone(),
                    partner: cells[7].clone(),
                    registration_number: cells[8].clone(),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(rows)
}
