use dotenv::dotenv;
use vbt_lib::{
    discord::{
        send::{send, DiscordEmbed, EmbedFooter, EmbedThumbnail},
        wh,
    },
    file_ops::{json_ops, rss_ops},
    json::return_json::return_json,
    requests::get_data,
    services::watchlist::load_watchlist,
    utils::time::{generate_time, generate_unix_timestamp},
};

async fn process_watchlist(id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let watchlist = load_watchlist()?;

    for entry in watchlist {
        // Fetch data using name in the watchlist
        let rows = get_data::extract_table_data(&entry.name)
            .await
            .map_err(|e| {
                Box::<dyn std::error::Error>::from(format!(
                    "Failed to fetch data for {}: {}",
                    entry.name, e
                ))
            })?;

        // Generate JSON for json file
        let json_data = return_json(&entry.name, &rows).map_err(|e| {
            Box::<dyn std::error::Error>::from(format!(
                "Failed to generate JSON for {}: {}",
                entry.name, e
            ))
        })?;

        // Get romaji filename or use "name" if not available
        let filename_base = entry
            .other
            .get("romaji")
            .unwrap_or(&entry.name)
            .replace(" ", "_");

        json_ops::save_json(&json_data, &format!("feed/json/{}.json", filename_base))?;

        let rss_content = rss_ops::generate_rss(&rows, &entry);
        rss_ops::save_rss(&rss_content, &format!("feed/rss/{}.rss", filename_base))?;

        println!("Processed: {}", entry.name);
        let url = wh::processed_url().map_err(Box::<dyn std::error::Error>::from)?;
        let title = entry.name;
        let description = format!("Id: **{}**", id);

        let embed = DiscordEmbed {
            title,
            description,
            footer: Some(EmbedFooter {
                text: format!("VBT - {}", generate_time()),
                ..Default::default()
            }),
            thumbnail: Some(EmbedThumbnail { url: entry.cover }),
            ..Default::default()
        };

        send(url, "<@&1304123731442012220>".to_string(), Some(embed))
            .await
            .map_err(Box::<dyn std::error::Error>::from)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();
    let id = generate_unix_timestamp();
    let url = wh::daily_url().map_err(Box::<dyn std::error::Error>::from)?;
    let title = "A daily run has started".to_string();
    let description = format!("Id: {}", id);

    let embed = DiscordEmbed {
        title,
        description,
        footer: Some(EmbedFooter {
            text: format!("VBT - {}", generate_time()),
            ..Default::default()
        }),
        ..Default::default()
    };

    send(
        url.to_string(),
        "<@&1304134014734434315>".to_string(),
        Some(embed),
    )
    .await
    .map_err(Box::<dyn std::error::Error>::from)?;

    match process_watchlist(id).await {
        Ok(_) => {
            let title = "Finished".to_string();
            let embed = DiscordEmbed {
                title,
                footer: Some(EmbedFooter {
                    text: format!("VBT - {}", generate_time()),
                    ..Default::default()
                }),
                ..Default::default()
            };
            send(url.clone().to_string(), "".to_string(), Some(embed))
                .await
                .map_err(Box::<dyn std::error::Error>::from)?;
            Ok(())
        }
        Err(e) => Err(e),
    }?;

    Ok(())
}
