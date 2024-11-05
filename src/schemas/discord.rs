use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscordAlert {
    #[serde(rename = "embeds")]
    pub embeds: Vec<DiscordEmbed>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordEmbed {
    #[serde(rename = "type")]
    pub embed_type: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "color")]
    pub color: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<DiscordField>,
    #[serde(rename = "footer")]
    pub footer: DiscordFooter,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordField {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordFooter {
    #[serde(rename = "text")]
    pub text: String,
}
