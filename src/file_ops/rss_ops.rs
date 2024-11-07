use crate::{
    types::{book::BookRow, watchlist::WatchlistEntry},
    utils::{
        cache::{generate_cache_key, load_cache, save_cache},
        fs::ensure_parent_dir,
        time::generate_time,
    },
};
use std::{fs::File, io::Write};

pub fn generate_rss(books: &Vec<BookRow>, watchlist_entry: &WatchlistEntry) -> String {
    let now = generate_time();
    let mut cache = load_cache::<String>("date_cache.json");

    let mut rss = format!(
        r#"<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0">
<channel>
    <title>{}</title>
    <link>https://ppdvn.gov.vn</link>
    <description>VBT feed for: {}</description>
    <image>
        <url>{}</url>
        <title>{}</title>
        <link>https://ppdvn.gov.vn</link>
    </image>
    <pubDate>{}</pubDate>
"#,
        watchlist_entry.name,
        watchlist_entry.name,
        watchlist_entry.cover,
        watchlist_entry.name,
        now
    );

    for book in books {
        let cache_key = generate_cache_key(book);
        let pub_date = cache
            .data
            .entry(cache_key)
            .or_insert_with(|| now.clone())
            .clone();

        let alt_titles = watchlist_entry
            .other
            .iter()
            .map(|(_, value)| format!("{}", value))
            .collect::<Vec<_>>()
            .join(", ");

        rss.push_str(&format!(
            r#"    <item>
        <title>{}</title>
        <description>Author: {} | Translator: {} | ISBN: {} | Alternative Titles: {}</description>
        <enclosure url="{}" length="0"></enclosure>
        <guid>{}</guid>
        <pubDate>{}</pubDate>
    </item>
"#,
            book.title,
            book.author,
            book.translator,
            book.isbn,
            alt_titles,
            watchlist_entry.cover,
            format!(
                "https://ppdvn.gov.vn/web/guest/ke-hoach-xuat-ban?query={}",
                book.title.trim_end().replace(" ", "+")
            ),
            pub_date
        ));
    }

    rss.push_str("</channel>\n</rss>");

    if let Err(e) = save_cache(&cache, "date_cache.json") {
        eprintln!("Failed to save date cache: {}", e);
    }

    rss
}

pub fn save_rss(content: &str, path: &str) -> Result<(), String> {
    ensure_parent_dir(path)?;

    File::create(path)
        .map_err(|e| format!("Failed to create RSS file: {}", e))?
        .write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write RSS content: {}", e))?;

    Ok(())
}
