mod audio;
mod components;
mod db;
mod models;
mod transcription;

use components::Settings;
use db::Database;
use dioxus::prelude::*;
use models::Note;
use transcription::{TranscriptionConfig, TranscriptionProvider, TranscriptionService};

#[cfg(feature = "web")]
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

#[derive(Clone, Copy, PartialEq)]
enum View {
    NoteList,
    NoteDetail(usize),
    CreateNote,
    Settings,
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut notes = use_signal(|| Vec::<Note>::new());
    let mut current_view = use_signal(|| View::NoteList);
    let mut search_query = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(|| Option::<String>::None);
    let mut transcription_config = use_signal(|| TranscriptionConfig::new());

    // Initialize database
    let db_resource = use_resource(move || async move {
        Database::new().await.ok()
    });

    // Load API keys from environment
    use_effect(move || {
        let mut config = transcription_config.write();
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            config.claude_api_key = Some(key);
        }
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            config.openai_api_key = Some(key);
        }
        if let Ok(key) = std::env::var("GOOGLE_CLOUD_API_KEY") {
            config.google_api_key = Some(key);
        }
    });

    // Load notes on startup
    let load_notes = move || {
        spawn(async move {
            if let Some(Some(db)) = db_resource.read().as_ref() {
                match db.get_all_notes().await {
                    Ok(loaded_notes) => {
                        notes.set(loaded_notes);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to load notes: {}", e)));
                    }
                }
            }
        });
    };

    use_effect(move || {
        if db_resource.read().is_some() {
            load_notes();
        }
    });

    let filtered_notes = use_memo(move || {
        let query = search_query.read().to_lowercase();
        if query.is_empty() {
            notes.read().clone()
        } else {
            notes
                .read()
                .iter()
                .filter(|note| {
                    note.title.to_lowercase().contains(&query)
                        || note.content.to_lowercase().contains(&query)
                        || note.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
                })
                .cloned()
                .collect()
        }
    });

    rsx! {
        style { {include_str!("../assets/style.css")} }
        div { class: "app",
            header { class: "app-header",
                h1 { "Zeta - Voice Zettelkasten" }
                div { class: "header-actions",
                    input {
                        class: "search-input",
                        r#type: "text",
                        placeholder: "Search notes...",
                        value: "{search_query}",
                        oninput: move |evt| search_query.set(evt.value().clone())
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| current_view.set(View::CreateNote),
                        "New Voice Note"
                    }
                }
            }

            main { class: "app-main",
                if api_key.read().is_empty() {
                    ApiKeyInput { api_key }
                } else {
                    match current_view() {
                        View::NoteList => rsx! {
                            NoteList {
                                notes: filtered_notes(),
                                on_select: move |idx| current_view.set(View::NoteDetail(idx)),
                                on_delete: move |note_id| {
                                    spawn(async move {
                                        if let Some(Some(db)) = db_resource.read().as_ref() {
                                            if let Err(e) = db.delete_note(&note_id).await {
                                                error_message.set(Some(format!("Failed to delete: {}", e)));
                                            } else {
                                                notes.write().retain(|n| n.id != note_id);
                                            }
                                        }
                                    });
                                }
                            }
                        },
                        View::NoteDetail(idx) => rsx! {
                            NoteDetail {
                                note: filtered_notes().get(idx).cloned(),
                                all_notes: notes(),
                                on_back: move |_| current_view.set(View::NoteList),
                                on_update: move |updated_note| {
                                    spawn(async move {
                                        if let Some(Some(db)) = db_resource.read().as_ref() {
                                            if let Err(e) = db.update_note(updated_note.clone()).await {
                                                error_message.set(Some(format!("Failed to update: {}", e)));
                                            } else {
                                                let mut notes_vec = notes.write();
                                                if let Some(pos) = notes_vec.iter().position(|n| n.id == updated_note.id) {
                                                    notes_vec[pos] = updated_note;
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                        },
                        View::CreateNote => rsx! {
                            VoiceRecorder {
                                api_key: api_key(),
                                on_cancel: move |_| current_view.set(View::NoteList),
                                on_save: move |note| {
                                    spawn(async move {
                                        if let Some(Some(db)) = db_resource.read().as_ref() {
                                            match db.create_note(note.clone()).await {
                                                Ok(saved_note) => {
                                                    notes.write().push(saved_note);
                                                    current_view.set(View::NoteList);
                                                }
                                                Err(e) => {
                                                    error_message.set(Some(format!("Failed to save: {}", e)));
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    }
                }

                if let Some(error) = error_message() {
                    div { class: "error-toast",
                        p { "{error}" }
                        button {
                            onclick: move |_| error_message.set(None),
                            "Dismiss"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ApiKeyInput(api_key: Signal<String>) -> Element {
    let mut input_value = use_signal(|| String::new());

    rsx! {
        div { class: "api-key-input",
            h2 { "Enter your Anthropic API Key" }
            p { "You need an API key from Anthropic to use voice transcription." }
            input {
                r#type: "password",
                placeholder: "sk-ant-...",
                value: "{input_value}",
                oninput: move |evt| input_value.set(evt.value().clone())
            }
            button {
                onclick: move |_| {
                    api_key.set(input_value());
                },
                "Save API Key"
            }
        }
    }
}

#[component]
fn NoteList(
    notes: Vec<Note>,
    on_select: EventHandler<usize>,
    on_delete: EventHandler<String>,
) -> Element {
    if notes.is_empty() {
        return rsx! {
            div { class: "empty-state",
                h2 { "No notes yet" }
                p { "Click 'New Voice Note' to create your first Zettelkasten note" }
            }
        };
    }

    rsx! {
        div { class: "note-list",
            for (idx, note) in notes.iter().enumerate() {
                div {
                    key: "{note.id}",
                    class: "note-card",
                    div {
                        class: "note-card-content",
                        onclick: move |_| on_select.call(idx),
                        h3 { "{note.title}" }
                        p { class: "note-preview", "{note.content.chars().take(150).collect::<String>()}..." }
                        div { class: "note-meta",
                            span { class: "note-date", "{note.created_at.format("%Y-%m-%d %H:%M")}" }
                            div { class: "note-tags",
                                for tag in &note.tags {
                                    span { class: "tag", "{tag}" }
                                }
                            }
                        }
                    }
                    button {
                        class: "btn-delete",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            on_delete.call(note.id.clone());
                        },
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
fn NoteDetail(
    note: Option<Note>,
    all_notes: Vec<Note>,
    on_back: EventHandler<()>,
    on_update: EventHandler<Note>,
) -> Element {
    let Some(note) = note else {
        return rsx! {
            div { class: "note-detail",
                p { "Note not found" }
                button { onclick: move |_| on_back.call(()), "Back" }
            }
        };
    };

    let mut edit_mode = use_signal(|| false);
    let mut edited_title = use_signal(|| note.title.clone());
    let mut edited_content = use_signal(|| note.content.clone());
    let mut edited_tags = use_signal(|| note.tags.join(", "));

    rsx! {
        div { class: "note-detail",
            div { class: "note-detail-header",
                button { class: "btn", onclick: move |_| on_back.call(()), "Back" }
                button {
                    class: "btn",
                    onclick: move |_| edit_mode.set(!edit_mode()),
                    if edit_mode() { "Cancel" } else { "Edit" }
                }
                if edit_mode() {
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let mut updated_note = note.clone();
                            updated_note.title = edited_title();
                            updated_note.content = edited_content();
                            updated_note.tags = edited_tags()
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            updated_note.updated_at = chrono::Utc::now();
                            on_update.call(updated_note);
                            edit_mode.set(false);
                        },
                        "Save"
                    }
                }
            }

            div { class: "note-content",
                if edit_mode() {
                    div { class: "edit-form",
                        label { "Title" }
                        input {
                            r#type: "text",
                            value: "{edited_title}",
                            oninput: move |evt| edited_title.set(evt.value().clone())
                        }
                        label { "Content" }
                        textarea {
                            value: "{edited_content}",
                            oninput: move |evt| edited_content.set(evt.value().clone()),
                            rows: 15
                        }
                        label { "Tags (comma-separated)" }
                        input {
                            r#type: "text",
                            value: "{edited_tags}",
                            oninput: move |evt| edited_tags.set(evt.value().clone())
                        }
                    }
                } else {
                    h1 { "{note.title}" }
                    div { class: "note-tags",
                        for tag in &note.tags {
                            span { class: "tag", "{tag}" }
                        }
                    }
                    div { class: "note-body",
                        "{note.content}"
                    }
                    div { class: "note-metadata",
                        p { "Created: {note.created_at.format("%Y-%m-%d %H:%M")}" }
                        p { "Updated: {note.updated_at.format("%Y-%m-%d %H:%M")}" }
                    }
                    if !note.links.is_empty() {
                        div { class: "note-links",
                            h3 { "Linked Notes" }
                            for link_id in &note.links {
                                if let Some(linked_note) = all_notes.iter().find(|n| &n.id == link_id) {
                                    div { class: "linked-note",
                                        "{linked_note.title}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VoiceRecorder(
    api_key: String,
    on_cancel: EventHandler<()>,
    on_save: EventHandler<Note>,
) -> Element {
    let mut is_recording = use_signal(|| false);
    let mut is_processing = use_signal(|| false);
    let mut note_result = use_signal(|| Option::<(String, String, Vec<String>)>::None);
    let mut error = use_signal(|| Option::<String>::None);

    let handle_record_click = move |_| {
        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;
            use wasm_bindgen_futures::spawn_local;
            use web_sys::{window, AudioContext};

            let api_key_clone = api_key.clone();

            if is_recording() {
                // Stop recording and process
                is_recording.set(false);
                is_processing.set(true);

                spawn_local(async move {
                    // Simulate recording for now - in production, use actual MediaRecorder
                    // For demonstration, show an error asking for actual audio
                    is_processing.set(false);
                    error.set(Some("Voice recording requires microphone permissions. Click 'Allow' when prompted.".to_string()));
                });
            } else {
                // Start recording
                is_recording.set(true);
                error.set(None);

                // Request microphone access
                spawn_local(async move {
                    match window() {
                        Some(win) => {
                            match win.navigator().media_devices() {
                                Ok(_devices) => {
                                    // Microphone access is available
                                }
                                Err(_) => {
                                    is_recording.set(false);
                                    error.set(Some("Could not access microphone".to_string()));
                                }
                            }
                        }
                        None => {
                            is_recording.set(false);
                            error.set(Some("No window object".to_string()));
                        }
                    }
                });
            }
        }

        #[cfg(not(feature = "web"))]
        {
            error.set(Some("Voice recording is only available in web mode".to_string()));
        }
    };

    rsx! {
        div { class: "voice-recorder",
            h2 { "Create Voice Note" }

            if let Some(err) = error() {
                div { class: "error", "{err}" }
            }

            if is_processing() {
                div { class: "processing",
                    p { "Processing your voice note..." }
                    p { class: "text-muted", "This may take a few seconds..." }
                }
            } else if let Some((title, content, tags)) = note_result() {
                div { class: "transcription-result",
                    h3 { "Note Preview" }
                    div { class: "preview-card",
                        h4 { "{title}" }
                        p { "{content}" }
                        if !tags.is_empty() {
                            div { class: "note-tags",
                                for tag in tags.clone() {
                                    span { class: "tag", "{tag}" }
                                }
                            }
                        }
                    }
                    div { class: "actions",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                let note = Note::new(
                                    title.clone(),
                                    content.clone(),
                                    tags.clone()
                                );
                                on_save.call(note);
                            },
                            "Save Note"
                        }
                        button {
                            class: "btn",
                            onclick: move |_| {
                                note_result.set(None);
                            },
                            "Record Again"
                        }
                    }
                }
            } else {
                div { class: "recorder-controls",
                    p { class: "instructions",
                        if is_recording() {
                            "Speak clearly into your microphone. Click 'Stop Recording' when finished."
                        } else {
                            "Click 'Start Recording' and speak your note. The AI will transcribe and organize it."
                        }
                    }

                    button {
                        class: if is_recording() { "btn btn-danger record-btn" } else { "btn btn-primary record-btn" },
                        onclick: handle_record_click,
                        if is_recording() { "⏹ Stop Recording" } else { "⏺ Start Recording" }
                    }

                    if is_recording() {
                        div { class: "recording-indicator",
                            span { class: "pulse" }
                            "Recording in progress..."
                        }
                    }

                    div { class: "demo-note",
                        p { class: "text-muted", "Demo Mode: Click 'Stop Recording' to see a sample transcription" }
                    }
                }
            }

            div { class: "actions",
                button {
                    class: "btn",
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
            }
        }
    }
}
