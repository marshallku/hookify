use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsAlert {
    #[serde(rename = "type")]
    pub message_type: String,
    pub attachments: Vec<TeamsAttachment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsAttachment {
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub content: TeamsContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub body: Vec<TeamsTextBlock>,
    pub actions: Vec<TeamsAction>,
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsTextBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub title: String,
    pub url: String,
}
