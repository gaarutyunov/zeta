use crate::models::TranscriptionResponse;
use crate::transcription::{TranscriptionProvider, TranscriptionService};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::{multipart, Client};
use serde::{Deserialize, Serialize};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/audio/transcriptions";
const OPENAI_CHAT_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Serialize, Deserialize)]
struct TranscriptionResult {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    response_format: ResponseFormat,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

pub struct OpenAITranscriptionService {
    api_key: String,
    client: Client,
}

impl OpenAITranscriptionService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    fn get_file_extension(format: &str) -> &str {
        match format {
            "webm" => "webm",
            "mp3" => "mp3",
            "wav" => "wav",
            "ogg" => "ogg",
            "m4a" => "m4a",
            _ => "webm",
        }
    }

    async fn structure_with_gpt(&self, transcription: &str) -> Result<TranscriptionResponse> {
        let request = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: format!(
                    r#"Given this transcription, create a Zettelkasten note.

Transcription: {}

Respond with a JSON object with this exact structure:
{{
  "title": "A concise title (3-7 words) for the note",
  "text": "The full transcription",
  "tags": ["tag1", "tag2", "tag3"]
}}

The title should capture the main idea. Include 2-5 relevant tags."#,
                    transcription
                ),
            }],
            response_format: ResponseFormat {
                type_: "json_object".to_string(),
            },
        };

        let response = self
            .client
            .post(OPENAI_CHAT_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("OpenAI Chat API error: {}", error_text));
        }

        let chat_response: ChatResponse = response.json().await?;

        if let Some(choice) = chat_response.choices.first() {
            let content = &choice.message.content;
            match serde_json::from_str::<serde_json::Value>(content) {
                Ok(json) => {
                    let title = json["title"]
                        .as_str()
                        .unwrap_or("Voice Note")
                        .to_string();
                    let text = json["text"].as_str().unwrap_or(transcription).to_string();
                    let tags = json["tags"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_else(Vec::new);

                    Ok(TranscriptionResponse { text, title, tags })
                }
                Err(_) => Ok(TranscriptionResponse {
                    text: transcription.to_string(),
                    title: "Voice Note".to_string(),
                    tags: vec![],
                }),
            }
        } else {
            Err(anyhow!("No response from GPT"))
        }
    }
}

#[async_trait(?Send)]
impl TranscriptionService for OpenAITranscriptionService {
    async fn transcribe_audio(
        &self,
        audio_data: &[u8],
        audio_format: &str,
    ) -> Result<TranscriptionResponse> {
        let file_ext = Self::get_file_extension(audio_format);
        let filename = format!("audio.{}", file_ext);

        // Create multipart form
        let file_part = multipart::Part::bytes(audio_data.to_vec())
            .file_name(filename)
            .mime_str(&format!("audio/{}", audio_format))?;

        let form = multipart::Form::new()
            .part("file", file_part)
            .text("model", "whisper-1")
            .text("response_format", "text");

        let response = self
            .client
            .post(OPENAI_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("OpenAI Whisper API error: {}", error_text));
        }

        let transcription_text = response.text().await?;

        // Use GPT to structure the transcription
        self.structure_with_gpt(&transcription_text).await
    }

    fn provider(&self) -> TranscriptionProvider {
        TranscriptionProvider::OpenAI
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }
}
