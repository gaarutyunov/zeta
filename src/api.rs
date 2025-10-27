use crate::models::TranscriptionResponse;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

const CLAUDE_API_URL: &str = "https://api.anthropic.com/v1/messages";

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeMessage {
    role: String,
    content: Vec<ContentBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ContentBlock {
    Text {
        #[serde(rename = "type")]
        type_: String,
        text: String
    },
    Document {
        #[serde(rename = "type")]
        type_: String,
        source: DocumentSource,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentSource {
    #[serde(rename = "type")]
    type_: String,
    media_type: String,
    data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeContent {
    text: String,
}

pub struct ClaudeClient {
    api_key: String,
    client: Client,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn transcribe_audio(
        &self,
        audio_data: &str,
        audio_format: &str,
    ) -> Result<TranscriptionResponse> {
        // Determine media type based on format
        let media_type = match audio_format {
            "webm" => "audio/webm",
            "mp3" => "audio/mpeg",
            "wav" => "audio/wav",
            "ogg" => "audio/ogg",
            _ => "audio/webm",
        };

        let request = ClaudeRequest {
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 4096,
            messages: vec![ClaudeMessage {
                role: "user".to_string(),
                content: vec![
                    ContentBlock::Document {
                        type_: "document".to_string(),
                        source: DocumentSource {
                            type_: "base64".to_string(),
                            media_type: media_type.to_string(),
                            data: audio_data.to_string(),
                        },
                    },
                    ContentBlock::Text {
                        type_: "text".to_string(),
                        text: r#"Please transcribe this audio recording and create a Zettelkasten note.

Respond with a JSON object with this exact structure:
{
  "title": "A concise title (3-7 words) for the note",
  "text": "The full transcription of the audio",
  "tags": ["tag1", "tag2", "tag3"]
}

The title should capture the main idea. Include 2-5 relevant tags that categorize the content. Make the transcription accurate and well-formatted."#.to_string(),
                    },
                ],
            }],
        };

        let response = self
            .client
            .post(CLAUDE_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Claude API error: {}", error_text));
        }

        let claude_response: ClaudeResponse = response.json().await?;

        if let Some(content) = claude_response.content.first() {
            // Parse the JSON response from Claude
            let text = &content.text;

            // Try to extract JSON from the response
            let json_str = if let Some(start) = text.find('{') {
                if let Some(end) = text.rfind('}') {
                    &text[start..=end]
                } else {
                    text
                }
            } else {
                text
            };

            match serde_json::from_str::<serde_json::Value>(json_str) {
                Ok(json) => {
                    let title = json["title"]
                        .as_str()
                        .unwrap_or("Untitled Note")
                        .to_string();
                    let transcription = json["text"]
                        .as_str()
                        .unwrap_or(text)
                        .to_string();
                    let tags = json["tags"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_else(Vec::new);

                    Ok(TranscriptionResponse {
                        text: transcription,
                        title,
                        tags,
                    })
                }
                Err(_) => {
                    // If JSON parsing fails, use the raw text
                    Ok(TranscriptionResponse {
                        text: text.clone(),
                        title: "Voice Note".to_string(),
                        tags: vec![],
                    })
                }
            }
        } else {
            Err(anyhow!("No content in Claude response"))
        }
    }
}
