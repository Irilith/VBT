use reqwest::Client;
use serde_json::json;
use std::error::Error;
#[derive(Default)]
pub struct DiscordEmbed {
    pub title: String,
    pub description: String,
    pub fields: Option<Vec<EmbedField>>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
}

#[derive(Default)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Default)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Default)]
pub struct EmbedImage {
    pub url: String,
}

#[derive(Default)]
pub struct EmbedThumbnail {
    pub url: String,
}

pub async fn send(
    url: String,
    content: String,
    embed: Option<DiscordEmbed>,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let mut payload = json!({
        "content": content
    });

    if let Some(embed) = embed {
        let mut embed_json = json!({
            "title": embed.title,
            "description": embed.description,
            "color": 0x00ff00
        });

        if let Some(fields) = embed.fields {
            embed_json["fields"] = json!(fields
                .iter()
                .map(|f| {
                    json!({
                        "name": f.name,
                        "value": f.value,
                        "inline": f.inline
                    })
                })
                .collect::<Vec<_>>());
        }

        if let Some(footer) = embed.footer {
            let mut footer_json = json!({
                "text": footer.text
            });
            if let Some(icon_url) = footer.icon_url {
                footer_json["icon_url"] = json!(icon_url);
            }
            embed_json["footer"] = footer_json;
        }

        if let Some(image) = embed.image {
            embed_json["image"] = json!({
                "url": image.url
            });
        }

        if let Some(thumbnail) = embed.thumbnail {
            embed_json["thumbnail"] = json!({
                "url": thumbnail.url
            });
        }
        payload["embeds"] = json!([embed_json]);
    }

    client.post(&url).json(&payload).send().await?;

    Ok(())
}
