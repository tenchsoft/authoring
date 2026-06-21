//! Lightweight UI state for the Story app.
//!
//! This module contains only UI-related state (active tab, focus mode, overlay
//! visibility). All document editing logic lives in `tench_story_core::StoryEngine`.

use tench_story_core::StoryEngine;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StoryTab {
    Characters,
    World,
    Timeline,
    Comments,
    Stats,
    Glossary,
    Relationships,
    MindMap,
    AiAssist,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StoryInputFocus {
    Manuscript,
    Search,
}

pub struct StoryState {
    /// The core editing engine — owns the document and all CRUD/search/undo logic.
    pub engine: StoryEngine,
    /// Currently active right-panel tab.
    pub active_tab: StoryTab,
    /// Currently selected chapter index.
    pub selected_chapter_idx: usize,
    /// Text input buffer for the editor.
    pub input_text: String,
    /// Whether the UI is in focus mode (side panels hidden).
    pub focus_mode: bool,
    /// Whether the export modal is open.
    pub show_export: bool,
    /// Whether the command palette is open.
    pub show_command_palette: bool,
    /// Timestamp of last save for status bar display.
    pub saved_at: String,
    /// Search query buffer.
    pub search_query: String,
    /// Whether search is case-sensitive.
    pub search_case_sensitive: bool,
    /// Whether the search panel is open.
    pub show_search: bool,
    /// Currently focused text input surface.
    pub input_focus: StoryInputFocus,
}

impl Default for StoryState {
    fn default() -> Self {
        let mut engine = StoryEngine::new_empty("Untitled Story");
        // Seed with a single empty chapter so the user has something to start with.
        engine.add_chapter("Chapter 1");
        StoryState {
            engine,
            active_tab: StoryTab::Characters,
            selected_chapter_idx: 0,
            input_text: String::new(),
            focus_mode: false,
            show_export: false,
            show_command_palette: false,
            saved_at: String::new(),
            search_query: String::new(),
            search_case_sensitive: false,
            show_search: false,
            input_focus: StoryInputFocus::Manuscript,
        }
    }
}

impl StoryState {
    pub fn example() -> Self {
        let mut state = Self::default();
        state.append_text("Ari opened the sealed archive gate.");
        state.add_chapter("Chapter 2");
        state.append_text("Mira documented the signal and the conflict.");
        state.selected_chapter_idx = 0;

        let ari = state.engine.add_character("Ari");
        let mira = state.engine.add_character("Mira");
        state.engine.add_relationship(&ari, &mira, "allies");
        state
            .engine
            .add_world_entry("Archive Gate", tench_story_core::WorldCategory::Place);
        state
            .engine
            .add_timeline_event("Year 12", "The signal arrives");
        state
            .engine
            .add_glossary_entry("Aster", "A navigation marker used by gatekeepers.");
        let chapter_id = state
            .engine
            .get_document()
            .chapters
            .first()
            .map(|chapter| chapter.id.clone());
        if let Some(chapter_id) = chapter_id {
            state
                .engine
                .add_comment(&chapter_id, "Clarify the gate rules.");
        }
        state.save();
        state
    }

    // ----- read helpers -----

    /// Get the project title from the engine.
    pub fn project_name(&self) -> String {
        self.engine.get_document().metadata.title.clone()
    }

    /// Get the word count for the currently selected chapter.
    pub fn chapter_word_count(&self) -> usize {
        let doc = self.engine.get_document();
        if let Some(ch) = doc.chapters.get(self.selected_chapter_idx) {
            ch.word_count()
        } else {
            0
        }
    }

    /// Get the total word count across all chapters.
    pub fn total_word_count(&self) -> usize {
        self.engine.get_document().total_word_count()
    }

    /// Get the plain text of the currently selected chapter.
    pub fn chapter_text(&self) -> String {
        let doc = self.engine.get_document();
        if let Some(ch) = doc.chapters.get(self.selected_chapter_idx) {
            ch.content.to_plain_text()
        } else {
            String::new()
        }
    }

    /// Get the title of the currently selected chapter.
    pub fn chapter_title(&self) -> String {
        let doc = self.engine.get_document();
        if let Some(ch) = doc.chapters.get(self.selected_chapter_idx) {
            ch.title.clone()
        } else {
            "No Chapter".to_string()
        }
    }

    /// Get the number of chapters.
    pub fn chapter_count(&self) -> usize {
        self.engine.get_document().chapters.len()
    }

    /// Whether the document has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.engine.is_dirty()
    }

    // ----- UI actions -----

    pub fn select_chapter(&mut self, index: usize) {
        let count = self.engine.get_document().chapters.len();
        if index < count {
            self.selected_chapter_idx = index;
            self.input_focus = StoryInputFocus::Manuscript;
        }
    }

    pub fn select_tab(&mut self, tab: StoryTab) {
        self.active_tab = tab;
    }

    pub fn open_project(&mut self) {
        self.engine.mark_saved();
        self.saved_at = "opened".to_string();
    }

    pub fn open_export(&mut self) {
        self.show_export = true;
        self.show_command_palette = false;
        self.show_search = false;
    }

    pub fn toggle_focus_mode(&mut self) {
        self.focus_mode = !self.focus_mode;
    }

    pub fn toggle_command_palette(&mut self) {
        self.show_command_palette = !self.show_command_palette;
        if self.show_command_palette {
            self.show_export = false;
            self.show_search = false;
        }
    }

    pub fn toggle_search(&mut self) {
        self.show_search = !self.show_search;
        self.input_focus = if self.show_search {
            StoryInputFocus::Search
        } else {
            StoryInputFocus::Manuscript
        };
    }

    pub fn close_overlays(&mut self) {
        self.show_export = false;
        self.show_command_palette = false;
        self.show_search = false;
        self.input_focus = StoryInputFocus::Manuscript;
    }

    pub fn save(&mut self) {
        self.engine.mark_saved();
        self.saved_at = "now".to_string();
    }

    // ----- Chapter editing via engine -----

    /// Append text to the current chapter (simplified for prototype).
    pub fn append_text(&mut self, text: &str) {
        let doc = self.engine.get_document();
        if let Some(ch) = doc.chapters.get(self.selected_chapter_idx) {
            let id = ch.id.clone();
            let mut content = ch.content.clone();
            let new_text = format!("{}{}", content.to_plain_text(), text);
            content = tench_document_core::TenchDocument::plain_text(&new_text);
            let _ = doc;
            self.engine.set_chapter_content(&id, content);
        }
    }

    pub fn newline(&mut self) {
        self.append_text("\n");
    }

    pub fn backspace(&mut self) {
        let doc = self.engine.get_document();
        if let Some(ch) = doc.chapters.get(self.selected_chapter_idx) {
            let id = ch.id.clone();
            let mut text = ch.content.to_plain_text();
            text.pop();
            let content = tench_document_core::TenchDocument::plain_text(&text);
            let _ = doc;
            self.engine.set_chapter_content(&id, content);
        }
    }

    pub fn append_search_text(&mut self, text: &str) {
        self.search_query.push_str(text);
    }

    pub fn backspace_search(&mut self) {
        self.search_query.pop();
    }

    pub fn toggle_search_case_sensitive(&mut self) {
        self.search_case_sensitive = !self.search_case_sensitive;
    }

    /// Add a new chapter via the engine.
    pub fn add_chapter(&mut self, title: &str) {
        self.engine.add_chapter(title);
        self.selected_chapter_idx = self.engine.get_document().chapters.len() - 1;
    }

    /// Delete the current chapter.
    pub fn delete_current_chapter(&mut self) {
        let doc = self.engine.get_document();
        if let Some(ch) = doc.chapters.get(self.selected_chapter_idx) {
            let id = ch.id.clone();
            let _ = doc;
            self.engine.delete_chapter(&id);
            if self.selected_chapter_idx >= self.engine.get_document().chapters.len() {
                self.selected_chapter_idx =
                    self.engine.get_document().chapters.len().saturating_sub(1);
            }
        }
    }

    /// Rename the current chapter.
    pub fn rename_current_chapter(&mut self, new_title: &str) {
        let doc = self.engine.get_document();
        if let Some(ch) = doc.chapters.get(self.selected_chapter_idx) {
            let id = ch.id.clone();
            let _ = doc;
            self.engine.rename_chapter(&id, new_title);
        }
    }

    /// Move the current chapter to a new position.
    pub fn move_current_chapter(&mut self, new_position: usize) {
        let doc = self.engine.get_document();
        if let Some(ch) = doc.chapters.get(self.selected_chapter_idx) {
            let id = ch.id.clone();
            let _ = doc;
            self.engine.move_chapter(&id, new_position);
            self.selected_chapter_idx = new_position;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_has_one_chapter() {
        let state = StoryState::default();
        assert_eq!(state.chapter_count(), 1);
        assert_eq!(state.chapter_title(), "Chapter 1");
    }

    #[test]
    fn add_and_select_chapter() {
        let mut state = StoryState::default();
        state.add_chapter("Chapter 2");
        assert_eq!(state.chapter_count(), 2);
        assert_eq!(state.selected_chapter_idx, 1);
        assert_eq!(state.chapter_title(), "Chapter 2");
    }

    #[test]
    fn append_text_marks_dirty() {
        let mut state = StoryState::default();
        state.append_text("Hello world");
        assert!(state.is_dirty());
        assert!(state.chapter_text().contains("Hello world"));
    }

    #[test]
    fn save_clears_dirty_state() {
        let mut state = StoryState::default();
        state.append_text("saved");
        state.save();
        assert!(!state.is_dirty());
        assert_eq!(state.saved_at, "now");
    }

    #[test]
    fn overlays_are_mutually_scoped_and_can_be_closed() {
        let mut state = StoryState::default();
        state.open_export();
        state.toggle_command_palette();
        state.close_overlays();
        assert!(!state.show_export);
        assert!(!state.show_command_palette);
    }

    #[test]
    fn delete_current_chapter_adjusts_index() {
        let mut state = StoryState::default();
        state.add_chapter("Chapter 2");
        state.add_chapter("Chapter 3");
        assert_eq!(state.chapter_count(), 3);
        state.selected_chapter_idx = 2;
        state.delete_current_chapter();
        assert_eq!(state.chapter_count(), 2);
        assert!(state.selected_chapter_idx < 2);
    }
}
