use super::*;
use tench_document_core::TenchDocument;

impl StoryEngine {
    // ----- metadata -----

    pub fn set_title(&mut self, title: &str) {
        self.push_undo();
        self.document.metadata.title = title.to_string();
        self.dirty = true;
    }

    pub fn set_author(&mut self, author: &str) {
        self.push_undo();
        self.document.metadata.author = Some(author.to_string());
        self.dirty = true;
    }

    pub fn set_genre(&mut self, genre: &str) {
        self.push_undo();
        self.document.metadata.genre = Some(genre.to_string());
        self.dirty = true;
    }

    pub fn set_language(&mut self, language: &str) {
        self.push_undo();
        self.document.metadata.language = Some(language.to_string());
        self.dirty = true;
    }

    // ----- chapter CRUD -----

    /// Add a new chapter with the given title. Returns the chapter ID.
    pub fn add_chapter(&mut self, title: &str) -> String {
        self.push_undo();
        let id = self.next_id();
        let order = self.document.chapters.len();
        let chapter = Chapter::new(&id, title, order);
        self.document.chapters.push(chapter);
        self.dirty = true;
        id
    }

    /// Insert a chapter at a specific position.
    pub fn insert_chapter(&mut self, title: &str, position: usize) -> String {
        self.push_undo();
        let id = self.next_id();
        let chapter = Chapter::new(&id, title, position);
        let pos = position.min(self.document.chapters.len());
        self.document.chapters.insert(pos, chapter);
        self.reindex_chapters();
        self.dirty = true;
        id
    }

    /// Delete a chapter by ID.
    pub fn delete_chapter(&mut self, chapter_id: &str) -> bool {
        self.push_undo();
        let before = self.document.chapters.len();
        self.document.chapters.retain(|c| c.id != chapter_id);
        if self.document.chapters.len() < before {
            self.reindex_chapters();
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Rename a chapter.
    pub fn rename_chapter(&mut self, chapter_id: &str, new_title: &str) -> bool {
        self.push_undo();
        if let Some(ch) = self
            .document
            .chapters
            .iter_mut()
            .find(|c| c.id == chapter_id)
        {
            ch.title = new_title.to_string();
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Update chapter notes.
    pub fn set_chapter_notes(&mut self, chapter_id: &str, notes: &str) -> bool {
        self.push_undo();
        if let Some(ch) = self
            .document
            .chapters
            .iter_mut()
            .find(|c| c.id == chapter_id)
        {
            ch.notes = notes.to_string();
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Update chapter status.
    pub fn set_chapter_status(&mut self, chapter_id: &str, status: ChapterStatus) -> bool {
        self.push_undo();
        if let Some(ch) = self
            .document
            .chapters
            .iter_mut()
            .find(|c| c.id == chapter_id)
        {
            ch.status = status;
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Set chapter section grouping.
    pub fn set_chapter_section(&mut self, chapter_id: &str, section: Option<String>) -> bool {
        self.push_undo();
        if let Some(ch) = self
            .document
            .chapters
            .iter_mut()
            .find(|c| c.id == chapter_id)
        {
            ch.section = section;
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Move a chapter to a new position.
    pub fn move_chapter(&mut self, chapter_id: &str, new_position: usize) -> bool {
        self.push_undo();
        let current_pos = self
            .document
            .chapters
            .iter()
            .position(|c| c.id == chapter_id);
        let Some(pos) = current_pos else {
            return false;
        };
        let new_pos = new_position.min(self.document.chapters.len().saturating_sub(1));
        if pos == new_pos {
            return false;
        }
        let chapter = self.document.chapters.remove(pos);
        self.document.chapters.insert(new_pos, chapter);
        self.reindex_chapters();
        self.dirty = true;
        true
    }

    /// Get a reference to a chapter by ID.
    pub fn get_chapter(&self, chapter_id: &str) -> Option<&Chapter> {
        self.document.chapters.iter().find(|c| c.id == chapter_id)
    }

    /// Get a mutable reference to a chapter by ID.
    pub fn get_chapter_mut(&mut self, chapter_id: &str) -> Option<&mut Chapter> {
        self.dirty = true;
        self.document
            .chapters
            .iter_mut()
            .find(|c| c.id == chapter_id)
    }

    /// Replace a chapter's content document.
    pub fn set_chapter_content(&mut self, chapter_id: &str, content: TenchDocument) -> bool {
        self.push_undo();
        if let Some(ch) = self
            .document
            .chapters
            .iter_mut()
            .find(|c| c.id == chapter_id)
        {
            ch.content = content;
            self.dirty = true;
            true
        } else {
            false
        }
    }

    fn reindex_chapters(&mut self) {
        for (i, ch) in self.document.chapters.iter_mut().enumerate() {
            ch.order = i;
        }
    }
}
