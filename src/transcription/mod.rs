mod apple_speech;
mod browser_speech;
mod claude;
mod google;
mod openai;
mod whisper_web;

pub use apple_speech::AppleSpeechService;
pub use browser_speech::BrowserSpeechService;
pub use claude::ClaudeTranscriptionService;
pub use google::GoogleTranscriptionService;
pub use openai::OpenAITranscriptionService;
pub use whisper_web::WhisperWebService;

use crate::models::TranscriptionResponse;
use anyhow::{anyhow, Result};
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
pub enum TranscriptionProvider {
    Claude,
    OpenAI,
    GoogleCloud,
    BrowserSpeech,
    WhisperWeb,
    AppleSpeech,
}

impl TranscriptionProvider {
    pub fn name(&self) -> &str {
        match self {
            Self::Claude => "Claude AI",
            Self::OpenAI => "OpenAI Whisper",
            Self::GoogleCloud => "Google Cloud",
            Self::BrowserSpeech => "Browser Speech Recognition",
            Self::WhisperWeb => "Local Whisper (WebGPU)",
            Self::AppleSpeech => "Apple Speech (iOS/macOS)",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Claude => "Best for structured notes with auto-generated titles and tags",
            Self::OpenAI => "High-quality transcription with GPT-powered note structuring",
            Self::GoogleCloud => "Fast and accurate Google Speech-to-Text API",
            Self::BrowserSpeech => "Free browser-based recognition (Chrome, Edge, Safari)",
            Self::WhisperWeb => "Privacy-first local transcription using WebGPU (experimental)",
            Self::AppleSpeech => "On-device Apple Speech framework for iOS and macOS",
        }
    }

    pub fn requires_api_key(&self) -> bool {
        match self {
            Self::Claude | Self::OpenAI | Self::GoogleCloud => true,
            Self::BrowserSpeech | Self::WhisperWeb | Self::AppleSpeech => false,
        }
    }

    pub fn supports_web(&self) -> bool {
        match self {
            Self::BrowserSpeech | Self::WhisperWeb => true,
            Self::Claude | Self::OpenAI | Self::GoogleCloud => true,
            Self::AppleSpeech => false,
        }
    }

    pub fn supports_ios(&self) -> bool {
        match self {
            Self::AppleSpeech => true,
            Self::Claude | Self::OpenAI | Self::GoogleCloud => true,
            Self::BrowserSpeech | Self::WhisperWeb => false,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::BrowserSpeech,
            Self::Claude,
            Self::OpenAI,
            Self::GoogleCloud,
            Self::WhisperWeb,
            Self::AppleSpeech,
        ]
    }

    pub fn web_providers() -> Vec<Self> {
        vec![
            Self::BrowserSpeech,
            Self::Claude,
            Self::OpenAI,
            Self::GoogleCloud,
            Self::WhisperWeb,
        ]
    }

    pub fn ios_providers() -> Vec<Self> {
        vec![
            Self::AppleSpeech,
            Self::Claude,
            Self::OpenAI,
            Self::GoogleCloud,
        ]
    }
}

#[async_trait(?Send)]
pub trait TranscriptionService {
    async fn transcribe_audio(
        &self,
        audio_data: &[u8],
        audio_format: &str,
    ) -> Result<TranscriptionResponse>;

    fn provider(&self) -> TranscriptionProvider;
    fn is_available(&self) -> bool;
}

pub struct TranscriptionConfig {
    pub provider: TranscriptionProvider,
    pub claude_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    pub google_api_key: Option<String>,
}

impl TranscriptionConfig {
    pub fn new() -> Self {
        Self {
            provider: TranscriptionProvider::Claude,
            claude_api_key: None,
            openai_api_key: None,
            google_api_key: None,
        }
    }

    pub fn get_service(&self) -> Result<Box<dyn TranscriptionService>> {
        match self.provider {
            TranscriptionProvider::Claude => {
                if let Some(key) = &self.claude_api_key {
                    Ok(Box::new(ClaudeTranscriptionService::new(key.clone())))
                } else {
                    Err(anyhow!("Claude API key not set"))
                }
            }
            TranscriptionProvider::OpenAI => {
                if let Some(key) = &self.openai_api_key {
                    Ok(Box::new(OpenAITranscriptionService::new(key.clone())))
                } else {
                    Err(anyhow!("OpenAI API key not set"))
                }
            }
            TranscriptionProvider::GoogleCloud => {
                if let Some(key) = &self.google_api_key {
                    Ok(Box::new(GoogleTranscriptionService::new(key.clone())))
                } else {
                    Err(anyhow!("Google Cloud API key not set"))
                }
            }
            TranscriptionProvider::BrowserSpeech => Ok(Box::new(BrowserSpeechService::new())),
            TranscriptionProvider::WhisperWeb => Ok(Box::new(WhisperWebService::new())),
            TranscriptionProvider::AppleSpeech => Ok(Box::new(AppleSpeechService::new())),
        }
    }
}

impl Default for TranscriptionConfig {
    fn default() -> Self {
        Self::new()
    }
}
