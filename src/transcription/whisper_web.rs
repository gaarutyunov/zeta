use crate::models::TranscriptionResponse;
use crate::transcription::{TranscriptionProvider, TranscriptionService};
use anyhow::{anyhow, Result};
use async_trait::async_trait;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "web")]
use wasm_bindgen_futures::JsFuture;

pub struct WhisperWebService {
    #[cfg(feature = "web")]
    is_webgpu_supported: bool,
}

impl WhisperWebService {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "web")]
            is_webgpu_supported: Self::check_webgpu_support(),
        }
    }

    #[cfg(feature = "web")]
    fn check_webgpu_support() -> bool {
        use wasm_bindgen::JsCast;

        if let Some(window) = web_sys::window() {
            if let Some(navigator) = window.navigator().dyn_ref::<web_sys::Navigator>() {
                // Check if GPU is available (WebGPU)
                return js_sys::Reflect::has(navigator, &JsValue::from_str("gpu"))
                    .unwrap_or(false);
            }
        }
        false
    }

    fn extract_title_and_tags(text: &str) -> (String, Vec<String>) {
        // Use first sentence or first 50 chars as title
        let title = text
            .split('.')
            .next()
            .unwrap_or(text)
            .chars()
            .take(50)
            .collect::<String>()
            .trim()
            .to_string();

        let title = if title.is_empty() {
            "Voice Note".to_string()
        } else {
            title
        };

        // Simple tag extraction
        let tags = vec!["voice-note".to_string(), "local".to_string()];

        (title, tags)
    }
}

#[cfg(feature = "web")]
#[wasm_bindgen(module = "/src/whisper_worker.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn transcribe_with_whisper(audio_data: &[u8]) -> Result<JsValue, JsValue>;

    #[wasm_bindgen]
    fn is_whisper_loaded() -> bool;
}

#[async_trait(?Send)]
impl TranscriptionService for WhisperWebService {
    async fn transcribe_audio(
        &self,
        audio_data: &[u8],
        _audio_format: &str,
    ) -> Result<TranscriptionResponse> {
        #[cfg(feature = "web")]
        {
            if !self.is_webgpu_supported {
                return Err(anyhow!(
                    "WebGPU is not supported in this browser. Please use Chrome, Edge, or another WebGPU-enabled browser."
                ));
            }

            // Note: In a real implementation, you would:
            // 1. Load the Whisper model from Transformers.js
            // 2. Process the audio with the model
            // 3. Return the transcription
            //
            // For now, we'll return a placeholder error directing users to the implementation

            return Err(anyhow!(
                "Local Whisper Web is not yet fully implemented. This requires loading the Whisper model via Transformers.js. Please use Claude, OpenAI, or Google Cloud for now."
            ));

            // Example of what the real implementation would look like:
            // let result = transcribe_with_whisper(audio_data).await
            //     .map_err(|e| anyhow!("Whisper transcription failed: {:?}", e))?;
            //
            // let transcription = result.as_string()
            //     .ok_or_else(|| anyhow!("Invalid transcription result"))?;
            //
            // let (title, tags) = Self::extract_title_and_tags(&transcription);
            //
            // Ok(TranscriptionResponse {
            //     text: transcription,
            //     title,
            //     tags,
            // })
        }

        #[cfg(not(feature = "web"))]
        {
            Err(anyhow!("Local Whisper is only available in web mode"))
        }
    }

    fn provider(&self) -> TranscriptionProvider {
        TranscriptionProvider::WhisperWeb
    }

    fn is_available(&self) -> bool {
        #[cfg(feature = "web")]
        {
            self.is_webgpu_supported
        }

        #[cfg(not(feature = "web"))]
        {
            false
        }
    }
}

impl Default for WhisperWebService {
    fn default() -> Self {
        Self::new()
    }
}
