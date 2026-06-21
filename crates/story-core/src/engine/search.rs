use super::*;
use tench_document_core::TenchDocument;

// ---------------------------------------------------------------------------
// Search result
// ---------------------------------------------------------------------------

/// A search match within the story.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorySearchMatch {
    pub chapter_id: String,
    pub chapter_title: String,
    pub block_idx: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

// ---------------------------------------------------------------------------
// Story statistics
// ---------------------------------------------------------------------------

/// Aggregated statistics for the story.
#[derive(Clone, Debug, Default)]
pub struct StoryStatistics {
    pub total_words: usize,
    pub total_characters: usize,
    pub total_sentences: usize,
    pub avg_sentence_length: f64,
    pub reading_time_minutes: usize,
    pub chapter_count: usize,
    pub character_count: usize,
    pub world_entry_count: usize,
    pub timeline_event_count: usize,
    pub glossary_entry_count: usize,
}

impl StoryEngine {
    // ----- search -----

    /// Full-text search across all chapters.
    pub fn search(&self, query: &str, case_sensitive: bool) -> Vec<StorySearchMatch> {
        let mut matches = Vec::new();
        if query.is_empty() {
            return matches;
        }

        for chapter in &self.document.chapters {
            let text = chapter.content.to_plain_text();
            let haystack = if case_sensitive {
                text.clone()
            } else {
                text.to_lowercase()
            };
            let needle = if case_sensitive {
                query.to_string()
            } else {
                query.to_lowercase()
            };

            let mut start = 0;
            while let Some(pos) = haystack[start..].find(&needle) {
                let abs = start + pos;
                matches.push(StorySearchMatch {
                    chapter_id: chapter.id.clone(),
                    chapter_title: chapter.title.clone(),
                    block_idx: 0, // simplified; real impl would track block
                    start_offset: abs,
                    end_offset: abs + needle.len(),
                });
                start = abs + 1;
                if start >= haystack.len() {
                    break;
                }
            }
        }
        matches
    }

    /// Regex search across all chapters.
    pub fn search_regex(&self, query: &str) -> Result<Vec<StorySearchMatch>, String> {
        let re = regex::RegexBuilder::new(query)
            .case_insensitive(true)
            .build()
            .map_err(|e| format!("Invalid regex: {e}"))?;

        let mut matches = Vec::new();
        for chapter in &self.document.chapters {
            let text = chapter.content.to_plain_text();
            for m in re.find_iter(&text) {
                matches.push(StorySearchMatch {
                    chapter_id: chapter.id.clone(),
                    chapter_title: chapter.title.clone(),
                    block_idx: 0,
                    start_offset: m.start(),
                    end_offset: m.end(),
                });
            }
        }
        Ok(matches)
    }

    /// Replace text in a specific chapter.
    pub fn replace_in_chapter(
        &mut self,
        chapter_id: &str,
        query: &str,
        replacement: &str,
        case_sensitive: bool,
    ) -> usize {
        self.push_undo();
        let Some(chapter) = self
            .document
            .chapters
            .iter_mut()
            .find(|c| c.id == chapter_id)
        else {
            return 0;
        };

        let text = chapter.content.to_plain_text();
        let (haystack, needle) = if case_sensitive {
            (text.clone(), query.to_string())
        } else {
            (text.to_lowercase(), query.to_lowercase())
        };

        let mut count = 0;
        let mut new_text = String::new();
        let mut last_end = 0;
        let mut start = 0;

        while let Some(pos) = haystack[start..].find(&needle) {
            let abs = start + pos;
            new_text.push_str(&text[last_end..abs]);
            new_text.push_str(replacement);
            last_end = abs + needle.len();
            start = last_end;
            count += 1;
        }
        new_text.push_str(&text[last_end..]);

        if count > 0 {
            chapter.content = TenchDocument::plain_text(&new_text);
            self.dirty = true;
        }
        count
    }

    /// Replace across all chapters.
    pub fn replace_all(&mut self, query: &str, replacement: &str, case_sensitive: bool) -> usize {
        self.push_undo();
        let mut total = 0;
        for chapter in &mut self.document.chapters {
            let text = chapter.content.to_plain_text();
            let (haystack, needle) = if case_sensitive {
                (text.clone(), query.to_string())
            } else {
                (text.to_lowercase(), query.to_lowercase())
            };

            let mut count = 0;
            let mut new_text = String::new();
            let mut last_end = 0;
            let mut start = 0;

            while let Some(pos) = haystack[start..].find(&needle) {
                let abs = start + pos;
                new_text.push_str(&text[last_end..abs]);
                new_text.push_str(replacement);
                last_end = abs + needle.len();
                start = last_end;
                count += 1;
            }
            new_text.push_str(&text[last_end..]);

            if count > 0 {
                chapter.content = TenchDocument::plain_text(&new_text);
                total += count;
            }
        }
        if total > 0 {
            self.dirty = true;
        }
        total
    }

    // ----- statistics -----

    pub fn statistics(&self) -> StoryStatistics {
        StoryStatistics {
            total_words: self.document.total_word_count(),
            total_characters: self.document.total_character_count(),
            total_sentences: self.document.total_sentence_count(),
            avg_sentence_length: self.document.avg_sentence_length(),
            reading_time_minutes: self.document.reading_time_minutes(),
            chapter_count: self.document.chapters.len(),
            character_count: self.document.characters.len(),
            world_entry_count: self.document.world_entries.len(),
            timeline_event_count: self.document.timeline_events.len(),
            glossary_entry_count: self.document.glossary.len(),
        }
    }
}
