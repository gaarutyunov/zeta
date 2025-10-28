use crate::models::TranscriptionResponse;
use crate::transcription::{TranscriptionProvider, TranscriptionService};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const GOOGLE_API_URL: &str = "https://speech.googleapis.com/v1/speech:recognize";

#[derive(Debug, Serialize)]
struct RecognitionConfig {
    encoding: String,
    sample_rate_hertz: u32,
    language_code: String,
    enable_automatic_punctuation: bool,
}

#[derive(Debug, Serialize)]
struct RecognitionAudio {
    content: String,
}

#[derive(Debug, Serialize)]
struct RecognizeRequest {
    config: RecognitionConfig,
    audio: RecognitionAudio,
}

#[derive(Debug, Deserialize)]
struct RecognizeResponse {
    results: Vec<RecognitionResult>,
}

#[derive(Debug, Deserialize)]
struct RecognitionResult {
    alternatives: Vec<SpeechRecognitionAlternative>,
}

#[derive(Debug, Deserialize)]
struct SpeechRecognitionAlternative {
    transcript: String,
    confidence: Option<f64>,
}

pub struct GoogleTranscriptionService {
    api_key: String,
    client: Client,
}

impl GoogleTranscriptionService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    fn get_encoding(format: &str) -> &str {
        match format {
            "webm" | "ogg" => "OGG_OPUS",
            "mp3" => "MP3",
            "wav" => "LINEAR16",
            "flac" => "FLAC",
            _ => "OGG_OPUS",
        }
    }

    fn get_sample_rate(format: &str) -> u32 {
        match format {
            "webm" | "ogg" => 48000,
            "mp3" => 44100,
            "wav" => 16000,
            _ => 48000,
        }
    }

    fn extract_title_and_tags(text: &str) -> (String, Vec<String>) {
        // Simple heuristic: use first sentence as title
        let title = text
            .split('.')
            .next()
            .unwrap_or(text)
            .chars()
            .take(60)
            .collect::<String>();

        // Extract simple tags from common words
        let tags = vec!["voice-note".to_string()];

        (title, tags)
    }
}

#[async_trait(?Send)]
impl TranscriptionService for GoogleTranscriptionService {
    async fn transcribe_audio(
        &self,
        audio_data: &[u8],
        audio_format: &str,
    ) -> Result<TranscriptionResponse> {
        let audio_base64 = BASE64.encode(audio_data);
        let encoding = Self::get_encoding(audio_format);
        let sample_rate = Self::get_sample_rate(audio_format);

        let request = RecognizeRequest {
            config: RecognitionConfig {
                encoding: encoding.to_string(),
                sample_rate_hertz: sample_rate,
                language_code: "en-US".to_string(),
                enable_automatic_punctuation: true,
            },
            audio: RecognitionAudio {
                content: audio_base64,
            },
        };

        let response = self
            .client
            .post(format!("{}?key={}", GOOGLE_API_URL, self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Google Cloud Speech API error: {}", error_text));
        }

        let recognition_response: RecognizeResponse = response.json().await?;

        // Combine all transcriptions
        let transcription = recognition_response
            .results
            .iter()
            .filter_map(|result| {
                result
                    .alternatives
                    .first()
                    .map(|alt| alt.transcript.clone())
            })
            .collect::<Vec<String>>()
            .join(" ");

        if transcription.is_empty() {
            return Err(anyhow!("No transcription received from Google Cloud"));
        }

        let (title, tags) = Self::extract_title_and_tags(&transcription);

        Ok(TranscriptionResponse {
            text: transcription,
            title,
            tags,
        })
    }

    fn provider(&self) -> TranscriptionProvider {
        TranscriptionProvider::GoogleCloud
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }
}
