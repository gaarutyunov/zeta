use crate::models::TranscriptionResponse;
use crate::transcription::{TranscriptionProvider, TranscriptionService};
use anyhow::{anyhow, Result};
use async_trait::async_trait;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::{SpeechRecognition, SpeechRecognitionEvent, SpeechRecognitionErrorEvent};

pub struct BrowserSpeechService {
    #[cfg(feature = "web")]
    is_supported: bool,
}

impl BrowserSpeechService {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "web")]
            is_supported: Self::check_support(),
        }
    }

    #[cfg(feature = "web")]
    fn check_support() -> bool {
        use wasm_bindgen::JsValue;

        if let Some(window) = web_sys::window() {
            // Check for both standard and webkit-prefixed versions
            js_sys::Reflect::has(&window, &JsValue::from_str("SpeechRecognition"))
                .unwrap_or(false)
                || js_sys::Reflect::has(&window, &JsValue::from_str("webkitSpeechRecognition"))
                    .unwrap_or(false)
        } else {
            false
        }
    }

    #[cfg(feature = "web")]
    fn create_recognition() -> Result<SpeechRecognition> {
        use wasm_bindgen::JsValue;

        let window = web_sys::window().ok_or_else(|| anyhow!("No window object"))?;

        // Try standard API first
        if let Ok(recognition) = js_sys::Reflect::get(&window, &JsValue::from_str("SpeechRecognition"))
        {
            if let Ok(constructor) = recognition.dyn_into::<js_sys::Function>() {
                if let Ok(instance) = constructor.construct_with_args(&js_sys::Array::new()) {
                    if let Ok(speech_rec) = instance.dyn_into::<SpeechRecognition>() {
                        return Ok(speech_rec);
                    }
                }
            }
        }

        // Try webkit-prefixed version
        if let Ok(recognition) =
            js_sys::Reflect::get(&window, &JsValue::from_str("webkitSpeechRecognition"))
        {
            if let Ok(constructor) = recognition.dyn_into::<js_sys::Function>() {
                if let Ok(instance) = constructor.construct_with_args(&js_sys::Array::new()) {
                    if let Ok(speech_rec) = instance.dyn_into::<SpeechRecognition>() {
                        return Ok(speech_rec);
                    }
                }
            }
        }

        Err(anyhow!("SpeechRecognition API not available"))
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
        let tags = vec!["voice-note".to_string(), "browser".to_string()];

        (title, tags)
    }
}

#[async_trait(?Send)]
impl TranscriptionService for BrowserSpeechService {
    async fn transcribe_audio(
        &self,
        _audio_data: &[u8],
        _audio_format: &str,
    ) -> Result<TranscriptionResponse> {
        #[cfg(feature = "web")]
        {
            if !self.is_supported {
                return Err(anyhow!(
                    "Browser Speech Recognition is not supported in this browser. Try Chrome, Edge, or Safari."
                ));
            }

            use wasm_bindgen_futures::JsFuture;
            use std::sync::{Arc, Mutex};

            let recognition = Self::create_recognition()?;

            // Configure recognition
            recognition.set_lang("en-US");
            recognition.set_continuous(false);
            recognition.set_interim_results(false);
            recognition.set_max_alternatives(1);

            // Create shared result
            let result_text = Arc::new(Mutex::new(String::new()));
            let result_text_clone = result_text.clone();
            let error_occurred = Arc::new(Mutex::new(false));
            let error_occurred_clone = error_occurred.clone();

            // Set up result callback
            let onresult = Closure::wrap(Box::new(move |event: SpeechRecognitionEvent| {
                if let Some(results) = event.results() {
                    let mut transcript = String::new();
                    for i in 0..results.length() {
                        if let Some(result) = results.get(i) {
                            if let Some(alternative) = result.get(0) {
                                transcript.push_str(&alternative.transcript());
                                transcript.push(' ');
                            }
                        }
                    }
                    *result_text_clone.lock().unwrap() = transcript.trim().to_string();
                }
            }) as Box<dyn FnMut(_)>);

            recognition.set_onresult(Some(onresult.as_ref().unchecked_ref()));
            onresult.forget();

            // Set up error callback
            let onerror = Closure::wrap(Box::new(move |_event: SpeechRecognitionErrorEvent| {
                *error_occurred_clone.lock().unwrap() = true;
            }) as Box<dyn FnMut(_)>);

            recognition.set_onerror(Some(onerror.as_ref().unchecked_ref()));
            onerror.forget();

            // Start recognition
            recognition.start()?;

            // Note: In a real implementation, this would need to wait for the recognition
            // to complete. This would typically be done with a Promise or event loop.
            // For now, return an error indicating this is a stub implementation.

            return Err(anyhow!(
                "Browser Speech Recognition requires microphone streaming and is not yet fully integrated with the audio recording flow. This will be implemented in a future update."
            ));
        }

        #[cfg(not(feature = "web"))]
        {
            Err(anyhow!(
                "Browser Speech Recognition is only available in web mode"
            ))
        }
    }

    fn provider(&self) -> TranscriptionProvider {
        TranscriptionProvider::BrowserSpeech
    }

    fn is_available(&self) -> bool {
        #[cfg(feature = "web")]
        {
            self.is_supported
        }

        #[cfg(not(feature = "web"))]
        {
            false
        }
    }
}

impl Default for BrowserSpeechService {
    fn default() -> Self {
        Self::new()
    }
}
