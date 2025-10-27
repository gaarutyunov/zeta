#[cfg(feature = "web")]
pub mod web {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{
        Blob, BlobPropertyBag, MediaRecorder, MediaRecorderOptions, MediaStream,
        MediaStreamConstraints, MediaTrackConstraints,
    };

    pub struct AudioRecorder {
        recorder: Option<MediaRecorder>,
        chunks: Vec<Blob>,
    }

    impl AudioRecorder {
        pub fn new() -> Self {
            Self {
                recorder: None,
                chunks: Vec::new(),
            }
        }

        pub async fn start(&mut self) -> Result<(), JsValue> {
            let window = web_sys::window().ok_or("No window")?;
            let navigator = window.navigator();
            let media_devices = navigator.media_devices()?;

            let mut constraints = MediaStreamConstraints::new();
            let mut audio_constraints = MediaTrackConstraints::new();
            audio_constraints.echo_cancellation(&JsValue::from(true));
            audio_constraints.noise_suppression(&JsValue::from(true));
            constraints.audio(&audio_constraints.into());

            let promise = media_devices.get_user_media_with_constraints(&constraints)?;
            let stream = JsFuture::from(promise).await?;
            let stream: MediaStream = stream.dyn_into()?;

            let mut options = MediaRecorderOptions::new();
            options.mime_type("audio/webm");

            let recorder = MediaRecorder::new_with_media_stream_and_options(&stream, &options)?;

            self.chunks.clear();

            let chunks_ref = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            let chunks_clone = chunks_ref.clone();

            let ondataavailable = Closure::wrap(Box::new(move |e: web_sys::BlobEvent| {
                if let Some(blob) = e.data() {
                    chunks_clone.borrow_mut().push(blob);
                }
            }) as Box<dyn FnMut(_)>);

            recorder.set_ondataavailable(Some(ondataavailable.as_ref().unchecked_ref()));
            ondataavailable.forget();

            recorder.start()?;
            self.recorder = Some(recorder);

            Ok(())
        }

        pub async fn stop(&mut self) -> Result<Vec<u8>, JsValue> {
            if let Some(recorder) = self.recorder.take() {
                let promise = js_sys::Promise::new(&mut |resolve, reject| {
                    let onstop = Closure::once(Box::new(move || {
                        resolve.call0(&JsValue::NULL).unwrap();
                    }) as Box<dyn FnOnce()>);

                    recorder.set_onstop(Some(onstop.as_ref().unchecked_ref()));
                    onstop.forget();

                    recorder.stop().unwrap();
                });

                JsFuture::from(promise).await?;

                // Collect chunks
                if let Some(ondataavailable) = recorder.ondataavailable() {
                    // Chunks were collected via the ondataavailable callback
                }

                // Create a blob from chunks
                let blob_parts = js_sys::Array::new();
                for chunk in &self.chunks {
                    blob_parts.push(chunk);
                }

                let mut blob_props = BlobPropertyBag::new();
                blob_props.type_("audio/webm");

                let blob = Blob::new_with_blob_sequence_and_options(&blob_parts, &blob_props)?;

                // Convert blob to base64
                let array_buffer = JsFuture::from(blob.array_buffer()).await?;
                let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                let bytes = uint8_array.to_vec();

                Ok(bytes)
            } else {
                Err(JsValue::from_str("No active recording"))
            }
        }
    }
}

#[cfg(not(feature = "web"))]
pub mod desktop {
    pub struct AudioRecorder;

    impl AudioRecorder {
        pub fn new() -> Self {
            Self
        }

        pub async fn start(&mut self) -> Result<(), String> {
            Err("Audio recording not supported in desktop mode yet".to_string())
        }

        pub async fn stop(&mut self) -> Result<Vec<u8>, String> {
            Err("Audio recording not supported in desktop mode yet".to_string())
        }
    }
}
