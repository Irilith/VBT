use dotenv::dotenv;
use vbt_lib::discord::{send::*, wh};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = wh::processed_url()?;
    let title = "Embed Title".to_string();
    let description = "This is a description of the embed message.".to_string();

    let embed = DiscordEmbed {
        title,
        description,
        fields: Some(vec![
            EmbedField {
                name: "Field 1".to_string(),
                value: "Value 1".to_string(),
                inline: true,
            },
            EmbedField {
                name: "Field 2".to_string(),
                value: "Value 2".to_string(),
                inline: false,
            },
        ]),
        footer: Some(EmbedFooter {
            text: "VBT".to_string(),
            icon_url: Some("https://s4.anilist.co/file/anilistcdn/media/manga/cover/large/nx98741-O0Xcc89Id03h.jpg".to_string()),
        }),
        thumbnail: Some(EmbedThumbnail {
            url: "https://s4.anilist.co/file/anilistcdn/media/manga/cover/large/nx98741-O0Xcc89Id03h.jpg".to_string(),
        }),
        ..Default::default()
    };

    send(url, "Content test".to_string(), Some(embed)).await?;
    Ok(())
}
