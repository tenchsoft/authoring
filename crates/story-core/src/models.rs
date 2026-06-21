//! Story document models — the core data types for the Story authoring tool.
//!
//! Reuses `TenchDocument` from `tench-document-core` as the rich-text content
//! model for each chapter.

use serde::{Deserialize, Serialize};
use tench_document_core::TenchDocument;

// ---------------------------------------------------------------------------
// Story metadata
// ---------------------------------------------------------------------------

/// Top-level metadata for a story project.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StoryMetadata {
    pub title: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub genre: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Chapter status
// ---------------------------------------------------------------------------

/// The editorial status of a chapter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChapterStatus {
    #[default]
    Draft,
    Revised,
    Complete,
}

// ---------------------------------------------------------------------------
// Chapter
// ---------------------------------------------------------------------------

/// A single chapter within a story.
///
/// Each chapter owns a `TenchDocument` that holds its rich-text content.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chapter {
    /// Unique identifier.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Ordering index (0-based).
    pub order: usize,
    /// Rich-text content stored as a TDM document.
    pub content: TenchDocument,
    /// Authoring notes / annotations for this chapter.
    #[serde(default)]
    pub notes: String,
    /// Editorial status.
    #[serde(default)]
    pub status: ChapterStatus,
    /// Optional section / part grouping key.
    #[serde(default)]
    pub section: Option<String>,
}

impl Chapter {
    /// Create a new empty chapter with the given title.
    pub fn new(id: &str, title: &str, order: usize) -> Self {
        Chapter {
            id: id.to_string(),
            title: title.to_string(),
            order,
            content: TenchDocument::new(title),
            notes: String::new(),
            status: ChapterStatus::Draft,
            section: None,
        }
    }

    /// Word count for this chapter.
    pub fn word_count(&self) -> usize {
        self.content.word_count()
    }

    /// Character count for this chapter.
    pub fn character_count(&self) -> usize {
        self.content.character_count()
    }
}

// ---------------------------------------------------------------------------
// Character
// ---------------------------------------------------------------------------

/// A character in the story.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub personality: String,
    #[serde(default)]
    pub appearance: String,
    #[serde(default)]
    pub background: String,
    /// Custom key-value attributes (age, occupation, nickname, etc.).
    #[serde(default)]
    pub custom_attributes: Vec<(String, String)>,
    /// IDs of chapters this character appears in.
    #[serde(default)]
    pub appearance_chapters: Vec<String>,
    /// Tags for filtering / searching.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Character {
    pub fn new(id: &str, name: &str) -> Self {
        Character {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            role: String::new(),
            personality: String::new(),
            appearance: String::new(),
            background: String::new(),
            custom_attributes: Vec::new(),
            appearance_chapters: Vec::new(),
            tags: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Character relationship
// ---------------------------------------------------------------------------

/// A relationship between two characters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterRelationship {
    pub id: String,
    pub character_a_id: String,
    pub character_b_id: String,
    /// Description of the relationship (family, friend, enemy, colleague, etc.).
    pub kind: String,
    #[serde(default)]
    pub description: String,
}

impl CharacterRelationship {
    pub fn new(id: &str, a_id: &str, b_id: &str, kind: &str) -> Self {
        CharacterRelationship {
            id: id.to_string(),
            character_a_id: a_id.to_string(),
            character_b_id: b_id.to_string(),
            kind: kind.to_string(),
            description: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// World entry
// ---------------------------------------------------------------------------

/// Category classification for world-building entries.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorldCategory {
    #[default]
    Place,
    Concept,
    Rule,
    History,
    Culture,
    Item,
    Custom,
}

/// An entry in the world-building database.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEntry {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub category: WorldCategory,
    #[serde(default)]
    pub description: String,
    /// IDs of related world entries.
    #[serde(default)]
    pub related_entries: Vec<String>,
    /// IDs of related characters.
    #[serde(default)]
    pub related_characters: Vec<String>,
    /// Tags for filtering / searching.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl WorldEntry {
    pub fn new(id: &str, name: &str) -> Self {
        WorldEntry {
            id: id.to_string(),
            name: name.to_string(),
            category: WorldCategory::Place,
            description: String::new(),
            related_entries: Vec::new(),
            related_characters: Vec::new(),
            tags: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Timeline event
// ---------------------------------------------------------------------------

/// An event on the story timeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: String,
    /// Date / time label (free-form string for custom calendar support).
    pub date: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    /// IDs of characters involved.
    #[serde(default)]
    pub related_characters: Vec<String>,
    /// Tags for filtering.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl TimelineEvent {
    pub fn new(id: &str, date: &str, title: &str) -> Self {
        TimelineEvent {
            id: id.to_string(),
            date: date.to_string(),
            title: title.to_string(),
            description: String::new(),
            related_characters: Vec::new(),
            tags: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Glossary entry
// ---------------------------------------------------------------------------

/// A glossary term with definition and synonyms.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlossaryEntry {
    pub id: String,
    pub term: String,
    #[serde(default)]
    pub definition: String,
    #[serde(default)]
    pub synonyms: Vec<String>,
    /// Tags for filtering.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl GlossaryEntry {
    pub fn new(id: &str, term: &str) -> Self {
        GlossaryEntry {
            id: id.to_string(),
            term: term.to_string(),
            definition: String::new(),
            synonyms: Vec::new(),
            tags: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Bookmark
// ---------------------------------------------------------------------------

/// A bookmark pointing to a specific position in a chapter.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub chapter_id: String,
    /// Block index within the chapter document.
    pub block_idx: usize,
    /// Character offset within the block.
    pub offset: usize,
    #[serde(default)]
    pub label: String,
}

impl Bookmark {
    pub fn new(id: &str, chapter_id: &str, block_idx: usize, offset: usize) -> Self {
        Bookmark {
            id: id.to_string(),
            chapter_id: chapter_id.to_string(),
            block_idx,
            offset,
            label: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Comment
// ---------------------------------------------------------------------------

/// A comment attached to a range of text in a chapter.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryComment {
    pub id: String,
    pub chapter_id: String,
    pub block_idx: usize,
    pub start_offset: usize,
    pub end_offset: usize,
    pub text: String,
    #[serde(default)]
    pub resolved: bool,
    #[serde(default)]
    pub created_at: Option<String>,
}

impl StoryComment {
    pub fn new(id: &str, chapter_id: &str, text: &str) -> Self {
        StoryComment {
            id: id.to_string(),
            chapter_id: chapter_id.to_string(),
            block_idx: 0,
            start_offset: 0,
            end_offset: 0,
            text: text.to_string(),
            resolved: false,
            created_at: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Version snapshot
// ---------------------------------------------------------------------------

/// A snapshot of the entire story project for version history.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionSnapshot {
    pub id: String,
    #[serde(default)]
    pub label: String,
    /// Timestamp as Unix seconds.
    pub timestamp: u64,
    /// Serialized StoryDocument at this point in time.
    pub snapshot_json: String,
}

impl VersionSnapshot {
    pub fn new(id: &str, label: &str, timestamp: u64, snapshot_json: String) -> Self {
        VersionSnapshot {
            id: id.to_string(),
            label: label.to_string(),
            timestamp,
            snapshot_json,
        }
    }
}

// ---------------------------------------------------------------------------
// Writing goal
// ---------------------------------------------------------------------------

/// A daily or weekly writing goal.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WritingGoal {
    pub id: String,
    /// Target word count.
    pub target_words: usize,
    /// "daily" or "weekly".
    pub period: String,
    /// Words written so far in this period.
    pub progress: usize,
    /// Start timestamp.
    pub started_at: u64,
}

impl WritingGoal {
    pub fn new(id: &str, target_words: usize, period: &str) -> Self {
        WritingGoal {
            id: id.to_string(),
            target_words,
            period: period.to_string(),
            progress: 0,
            started_at: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// StoryDocument — the root
// ---------------------------------------------------------------------------

/// The root document model for a Story project.
///
/// Contains metadata, chapters, characters, world-building entries,
/// timeline events, glossary, and all supporting data.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StoryDocument {
    pub metadata: StoryMetadata,
    #[serde(default)]
    pub chapters: Vec<Chapter>,
    #[serde(default)]
    pub characters: Vec<Character>,
    #[serde(default)]
    pub relationships: Vec<CharacterRelationship>,
    #[serde(default)]
    pub world_entries: Vec<WorldEntry>,
    #[serde(default)]
    pub timeline_events: Vec<TimelineEvent>,
    #[serde(default)]
    pub glossary: Vec<GlossaryEntry>,
    #[serde(default)]
    pub bookmarks: Vec<Bookmark>,
    #[serde(default)]
    pub comments: Vec<StoryComment>,
    #[serde(default)]
    pub version_history: Vec<VersionSnapshot>,
    #[serde(default)]
    pub writing_goals: Vec<WritingGoal>,
}

impl StoryDocument {
    /// Create a new empty story with a title.
    pub fn new(title: &str) -> Self {
        StoryDocument {
            metadata: StoryMetadata {
                title: title.to_string(),
                ..StoryMetadata::default()
            },
            ..StoryDocument::default()
        }
    }

    /// Total word count across all chapters.
    pub fn total_word_count(&self) -> usize {
        self.chapters.iter().map(|c| c.word_count()).sum()
    }

    /// Total character count across all chapters.
    pub fn total_character_count(&self) -> usize {
        self.chapters.iter().map(|c| c.character_count()).sum()
    }

    /// Count the total number of sentences (naive: count '.', '!', '?' characters).
    pub fn total_sentence_count(&self) -> usize {
        let text: String = self
            .chapters
            .iter()
            .map(|c| c.content.to_plain_text())
            .collect::<Vec<_>>()
            .join(" ");
        text.chars()
            .filter(|c| *c == '.' || *c == '!' || *c == '?')
            .count()
            .max(1)
    }

    /// Average sentence length in words.
    pub fn avg_sentence_length(&self) -> f64 {
        let words = self.total_word_count() as f64;
        let sentences = self.total_sentence_count() as f64;
        if sentences == 0.0 {
            0.0
        } else {
            words / sentences
        }
    }

    /// Estimated reading time in minutes (assuming 250 words/min).
    pub fn reading_time_minutes(&self) -> usize {
        let words = self.total_word_count();
        (words as f64 / 250.0).ceil() as usize
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| format!("Serialization error: {e}"))
    }

    /// Deserialize from JSON string.
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("Deserialization error: {e}"))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_story_has_empty_metadata() {
        let story = StoryDocument::new("My Novel");
        assert_eq!(story.metadata.title, "My Novel");
        assert!(story.chapters.is_empty());
        assert!(story.characters.is_empty());
    }

    #[test]
    fn total_word_count_sums_chapters() {
        let mut story = StoryDocument::new("Test");
        story.chapters.push(Chapter::new("ch1", "Chapter 1", 0));
        story.chapters[0].content = TenchDocument::plain_text("Hello world foo bar");
        story.chapters.push(Chapter::new("ch2", "Chapter 2", 1));
        story.chapters[1].content = TenchDocument::plain_text("Baz qux");
        assert_eq!(story.total_word_count(), 6);
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let mut story = StoryDocument::new("Round Trip");
        story.metadata.author = Some("Author".to_string());
        story.chapters.push(Chapter::new("ch1", "Ch 1", 0));
        story.characters.push(Character::new("c1", "Alice"));

        let json = story.to_json().unwrap();
        let decoded = StoryDocument::from_json(&json).unwrap();
        assert_eq!(decoded.metadata.title, "Round Trip");
        assert_eq!(decoded.metadata.author, Some("Author".to_string()));
        assert_eq!(decoded.chapters.len(), 1);
        assert_eq!(decoded.characters.len(), 1);
    }

    #[test]
    fn chapter_word_count() {
        let ch = Chapter {
            content: TenchDocument::plain_text("one two three"),
            ..Chapter::new("ch1", "Test", 0)
        };
        assert_eq!(ch.word_count(), 3);
    }

    #[test]
    fn reading_time_calculation() {
        let mut story = StoryDocument::new("Test");
        story.chapters.push(Chapter::new("ch1", "Ch 1", 0));
        // 500 words => 2 minutes
        let text = (0..500)
            .map(|i| format!("word{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        story.chapters[0].content = TenchDocument::plain_text(&text);
        assert_eq!(story.reading_time_minutes(), 2);
    }
}
