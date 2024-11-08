use crate::{
    types::{book::BookRow, watchlist::WatchlistEntry},
    utils::{
        cache::{generate_cache_key, load_cache, save_cache},
        fs::ensure_parent_dir,
        time::generate_time,
        url::get_mime_type,
    },
};
use std::{collections::HashSet, fs::File, io::Write};

pub fn generate_rss(books: &[BookRow], watchlist_entry: &WatchlistEntry) -> String {
    let now = generate_time();
    let mut cache = load_cache::<String>("date_cache.json");

    let new_titles: HashSet<_> = books
        .iter()
        .map(|book| generate_cache_key(book))
        .filter(|key| !cache.data.contains_key(key))
        .collect();

    let title_build_date = if !new_titles.is_empty() {
        // There are new books update the publication date
        cache
            .title_build_date
            .insert(watchlist_entry.name.clone(), now.clone());
        now.clone()
    } else {
        // No new books, use existing date or current time if existing date not found
        cache
            .title_build_date
            .get(&watchlist_entry.name)
            .cloned()
            .unwrap_or_else(|| now.clone())
    };

    let mut rss = String::with_capacity(books.len() * 500);
    rss.push_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0">
<channel>
    <title>{}</title>
    <link>https://github.com/Irilith/VBT</link>
    <description>VBT feed for: {}</description>
    <image>
        <url>{}</url>
        <title>{}</title>
        <link>https://github.com/Irilith/VBT</link>
    </image>
    <pubDate>{}</pubDate>
    <lastBuildDate>{}</lastBuildDate>
"#,
        watchlist_entry.name,
        watchlist_entry.name,
        watchlist_entry.cover,
        watchlist_entry.name,
        now,
        title_build_date,
    ));

    let alt_titles = {
        let mut titles: Vec<_> = watchlist_entry
            .other
            .iter()
            .map(|(_, value)| value.to_string())
            .collect();
        titles.sort();
        titles.join(", ")
    };

    for book in books {
        let cache_key = generate_cache_key(book);
        let mime_type = get_mime_type(&watchlist_entry.cover);
        let pub_date = cache
            .data
            .entry(cache_key)
            .or_insert_with(|| now.clone())
            .clone();

        rss.push_str(&format!(
            r#"    <item>
        <title>{}</title>
        <link>https://github.com/Irilith/VBT</link>
        <description>Author: {} | Translator: {} | ISBN: {} | Alternative Titles: {}</description>
        <enclosure url="{}" length="0" type="{}"></enclosure>
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
            mime_type,
            format!(
                "https://ppdvn.gov.vn/web/guest/ke-hoach-xuat-ban?query={}",
                book.title.trim_end().replace(" ", "+")
            ),
            pub_date,
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
