use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub links: Vec<String>, // IDs of linked notes
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub audio_url: Option<String>, // Optional: store original audio
}

impl Note {
    pub fn new(title: String, content: String, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            tags,
            links: Vec::new(),
            created_at: now,
            updated_at: now,
            audio_url: None,
        }
    }

    pub fn add_link(&mut self, note_id: String) {
        if !self.links.contains(&note_id) {
            self.links.push(note_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_link(&mut self, note_id: &str) {
        self.links.retain(|id| id != note_id);
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNoteRequest {
    pub audio_data: String, // Base64 encoded audio
    pub audio_format: String, // e.g., "webm", "mp3", "wav"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResponse {
    pub text: String,
    pub title: String,
    pub tags: Vec<String>,
}
