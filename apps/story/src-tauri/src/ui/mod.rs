//! Story app — main widget that composes the full UI.
//!
//! Layout: LeftPanel (chapter tree) | Center (manuscript editor) | Right (auxiliary panel)

mod automation;
pub mod chapter_tree;
pub mod commands;
pub mod editor;
pub mod export;
mod geometry;
mod overlays;
pub mod panels;
pub mod state;
mod widget;

use state::{StoryState, StoryTab};

/// Story app — manuscript editor with chapter tree, panels, and export.
pub struct StoryApp {
    state: StoryState,
    /// Last known layout width, computed during layout for use in event handlers.
    width: f64,
}

impl Default for StoryApp {
    fn default() -> Self {
        Self::new()
    }
}

impl StoryApp {
    pub fn new() -> Self {
        Self {
            state: StoryState::default(),
            width: 0.0,
        }
    }

    pub fn with_state(state: StoryState) -> Self {
        Self { state, width: 0.0 }
    }

    pub fn state_mut(&mut self) -> &mut StoryState {
        &mut self.state
    }
}

impl StoryApp {
    fn dispatch_command_palette(&mut self, index: usize) {
        match index {
            0 => self.state = StoryState::default(),
            1 => self.state.open_project(),
            2 => self.state.save(),
            3 => self.state.open_export(),
            4 => self.state.toggle_focus_mode(),
            5 => self.state.add_chapter("New Chapter"),
            6 => self.state.delete_current_chapter(),
            7 => {
                self.state.engine.undo();
            }
            8 => {
                self.state.engine.redo();
            }
            9 => self.state.select_tab(StoryTab::Characters),
            10 => self.state.select_tab(StoryTab::World),
            11 => self.state.select_tab(StoryTab::Timeline),
            12 => self.state.select_tab(StoryTab::Glossary),
            13 => self.state.select_tab(StoryTab::Stats),
            14 => self.state.toggle_search(),
            15 => self.state.select_tab(StoryTab::AiAssist),
            _ => {}
        }
        if index != 3 {
            self.state.show_command_palette = false;
        }
    }
}

// ── Overlay painting ──────────────────────────────────────────────────────
