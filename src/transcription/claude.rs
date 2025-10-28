use crate::models::TranscriptionResponse;
use crate::transcription::{TranscriptionProvider, TranscriptionService};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
        text: String,
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

pub struct ClaudeTranscriptionService {
    api_key: String,
    client: Client,
}

impl ClaudeTranscriptionService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    fn get_media_type(format: &str) -> &str {
        match format {
            "webm" => "audio/webm",
            "mp3" => "audio/mpeg",
            "wav" => "audio/wav",
            "ogg" => "audio/ogg",
            _ => "audio/webm",
        }
    }
}

#[async_trait(?Send)]
impl TranscriptionService for ClaudeTranscriptionService {
    async fn transcribe_audio(
        &self,
        audio_data: &[u8],
        audio_format: &str,
    ) -> Result<TranscriptionResponse> {
        let audio_base64 = BASE64.encode(audio_data);
        let media_type = Self::get_media_type(audio_format);

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
                            data: audio_base64,
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

The title should capture the main idea. Include 2-5 relevant tags that categorize the content. Make the transcription accurate and well-formatted."#
                            .to_string(),
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
            let text = &content.text;

            // Extract JSON from the response
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
                    let transcription = json["text"].as_str().unwrap_or(text).to_string();
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
                Err(_) => Ok(TranscriptionResponse {
                    text: text.clone(),
                    title: "Voice Note".to_string(),
                    tags: vec![],
                }),
            }
        } else {
            Err(anyhow!("No content in Claude response"))
        }
    }

    fn provider(&self) -> TranscriptionProvider {
        TranscriptionProvider::Claude
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }
}
