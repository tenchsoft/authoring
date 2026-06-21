//! Story editing engine — high-level CRUD operations for the story document.
//!
//! The `StoryEngine` owns a [`StoryDocument`] and exposes operations for
//! chapters, characters, world entries, timeline events, glossary, search,
//! undo/redo, and statistics.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::*;
mod chapters;
mod history;
mod lore;
mod search;
mod support;
mod updates;

#[cfg(test)]
mod tests;

pub use search::{StorySearchMatch, StoryStatistics};
pub use updates::{CharacterUpdate, GlossaryEntryUpdate, TimelineEventUpdate, WorldEntryUpdate};

const UNDO_LIMIT: usize = 100;

// ---------------------------------------------------------------------------
// Undo snapshot
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct UndoSnapshot {
    document: StoryDocument,
}

// ---------------------------------------------------------------------------
// Engine
// ---------------------------------------------------------------------------

/// The core engine for a Story project session.
pub struct StoryEngine {
    document: StoryDocument,
    undo_stack: Vec<UndoSnapshot>,
    redo_stack: Vec<UndoSnapshot>,
    dirty: bool,
}

impl StoryEngine {
    // ----- construction -----

    pub fn new(document: StoryDocument) -> Self {
        StoryEngine {
            document,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            dirty: false,
        }
    }

    /// Create a new empty story with a title.
    pub fn new_empty(title: &str) -> Self {
        Self::new(StoryDocument::new(title))
    }

    // ----- read access -----

    pub fn get_document(&self) -> &StoryDocument {
        &self.document
    }

    pub fn get_document_mut(&mut self) -> &mut StoryDocument {
        self.dirty = true;
        &mut self.document
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    // ----- helpers -----

    fn push_undo(&mut self) {
        let snapshot = UndoSnapshot {
            document: self.document.clone(),
        };
        if self.undo_stack.len() >= UNDO_LIMIT {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(snapshot);
        self.redo_stack.clear();
    }

    fn next_id(&self) -> String {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        format!("id_{ts}")
    }
}
