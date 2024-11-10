use std::fmt::format;

use chrono::{Duration, Utc};
use dotenv::dotenv;
use vbt_lib::{
    config::flag::{EnvFlag, FT_CONFIG},
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

async fn send_webhook_message(
    url: String,
    mention: String,
    embed: DiscordEmbed,
    config: &EnvFlag,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.ft_webhook {
        send(url, mention, Some(embed))
            .await
            .map_err(Box::<dyn std::error::Error>::from)?;
    }
    Ok(())
}

async fn process_watchlist(id: i64, config: &EnvFlag) -> Result<(), Box<dyn std::error::Error>> {
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

        if config.ft_webhook {
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

            send_webhook_message(url, "<@&1304123731442012220>".to_string(), embed, config).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();

    let future_start_time = (Utc::now() + Duration::hours(24)).timestamp();
    let config = FT_CONFIG.read().unwrap();
    println!("{:?}", config);
    let id = generate_unix_timestamp();

    if config.ft_webhook {
        let url = wh::daily_url().map_err(Box::<dyn std::error::Error>::from)?;
        let title = "A daily worker has started".to_string();
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

        send_webhook_message(
            url.clone(),
            "<@&1304134014734434315>".to_string(),
            embed,
            &config,
        )
        .await?;
    }

    match process_watchlist(id, &config).await {
        Ok(_) => {
            if config.ft_webhook {
                let url = wh::daily_url().map_err(Box::<dyn std::error::Error>::from)?;
                let title = "Finished".to_string();
                // <t:TIME:R> Relative
                let description = format!("Next automatic worker is: <t:{}:R>", future_start_time);
                let embed = DiscordEmbed {
                    title,
                    description,
                    footer: Some(EmbedFooter {
                        text: format!("VBT - {}", generate_time()),
                        ..Default::default()
                    }),
                    ..Default::default()
                };

                send_webhook_message(url, "".to_string(), embed, &config).await?;
            }
            Ok(())
        }
        Err(e) => Err(e),
    }?;

    Ok(())
}
