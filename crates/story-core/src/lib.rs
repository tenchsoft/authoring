//! `tench-story-core` — shared core logic for the Tench Story authoring tool.
//!
//! This crate provides:
//! - **Models** (`models`): `StoryDocument`, `Chapter`, `Character`, `WorldEntry`,
//!   `TimelineEvent`, `GlossaryEntry`, and supporting types.
//! - **Engine** (`engine`): `StoryEngine` with full CRUD, search, undo/redo, and
//!   statistics for every model.
//! - **Project I/O** (`project_io`): Save/load story projects as ZIP bundles,
//!   plus import/export in DOCX, PDF, EPUB, Markdown, HTML, and TXT formats.

pub mod engine;
pub mod models;
pub mod project_io;

pub use engine::{StoryEngine, StorySearchMatch, StoryStatistics};
pub use models::*;
