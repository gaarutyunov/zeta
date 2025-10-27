use crate::models::Note;
use anyhow::Result;
use std::sync::{Arc, Mutex};

#[cfg(feature = "desktop")]
use surrealdb::{engine::local::Mem, Surreal};

pub struct Database {
    #[cfg(feature = "desktop")]
    db: Surreal<surrealdb::engine::local::Db>,

    #[cfg(feature = "web")]
    notes: Arc<Mutex<Vec<Note>>>,
}

impl Database {
    #[cfg(feature = "desktop")]
    pub async fn new() -> Result<Self> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("zeta").use_db("notes").await?;
        Ok(Self { db })
    }

    #[cfg(feature = "web")]
    pub async fn new() -> Result<Self> {
        Ok(Self {
            notes: Arc::new(Mutex::new(Vec::new())),
        })
    }

    #[cfg(feature = "desktop")]
    pub async fn create_note(&self, note: Note) -> Result<Note> {
        let created: Vec<Note> = self.db.create("notes").content(&note).await?;
        Ok(created.into_iter().next().unwrap_or(note))
    }

    #[cfg(feature = "web")]
    pub async fn create_note(&self, note: Note) -> Result<Note> {
        let mut notes = self.notes.lock().unwrap();
        notes.push(note.clone());
        Ok(note)
    }

    #[cfg(feature = "desktop")]
    pub async fn get_note(&self, id: &str) -> Result<Option<Note>> {
        let note: Option<Note> = self.db.select(("notes", id)).await?;
        Ok(note)
    }

    #[cfg(feature = "web")]
    pub async fn get_note(&self, id: &str) -> Result<Option<Note>> {
        let notes = self.notes.lock().unwrap();
        Ok(notes.iter().find(|n| n.id == id).cloned())
    }

    #[cfg(feature = "desktop")]
    pub async fn get_all_notes(&self) -> Result<Vec<Note>> {
        let notes: Vec<Note> = self.db.select("notes").await?;
        Ok(notes)
    }

    #[cfg(feature = "web")]
    pub async fn get_all_notes(&self) -> Result<Vec<Note>> {
        let notes = self.notes.lock().unwrap();
        Ok(notes.clone())
    }

    #[cfg(feature = "desktop")]
    pub async fn update_note(&self, note: Note) -> Result<Note> {
        let updated: Option<Note> = self.db.update(("notes", &note.id)).content(&note).await?;
        Ok(updated.unwrap_or(note))
    }

    #[cfg(feature = "web")]
    pub async fn update_note(&self, note: Note) -> Result<Note> {
        let mut notes = self.notes.lock().unwrap();
        if let Some(pos) = notes.iter().position(|n| n.id == note.id) {
            notes[pos] = note.clone();
        }
        Ok(note)
    }

    #[cfg(feature = "desktop")]
    pub async fn delete_note(&self, id: &str) -> Result<()> {
        let _: Option<Note> = self.db.delete(("notes", id)).await?;
        Ok(())
    }

    #[cfg(feature = "web")]
    pub async fn delete_note(&self, id: &str) -> Result<()> {
        let mut notes = self.notes.lock().unwrap();
        notes.retain(|n| n.id != id);
        Ok(())
    }

    #[cfg(feature = "desktop")]
    pub async fn search_notes(&self, query: &str) -> Result<Vec<Note>> {
        let all_notes: Vec<Note> = self.db.select("notes").await?;
        let query_lower = query.to_lowercase();
        Ok(all_notes
            .into_iter()
            .filter(|note| {
                note.title.to_lowercase().contains(&query_lower)
                    || note.content.to_lowercase().contains(&query_lower)
                    || note.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect())
    }

    #[cfg(feature = "web")]
    pub async fn search_notes(&self, query: &str) -> Result<Vec<Note>> {
        let notes = self.notes.lock().unwrap();
        let query_lower = query.to_lowercase();
        Ok(notes
            .iter()
            .filter(|note| {
                note.title.to_lowercase().contains(&query_lower)
                    || note.content.to_lowercase().contains(&query_lower)
                    || note.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect())
    }
}
