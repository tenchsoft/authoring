use crate::models::WorldCategory;

// ---------------------------------------------------------------------------
// Update structs (partial updates)
// ---------------------------------------------------------------------------

/// Partial update for a character.
pub struct CharacterUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub role: Option<String>,
    pub personality: Option<String>,
    pub appearance: Option<String>,
    pub background: Option<String>,
    pub custom_attributes: Option<Vec<(String, String)>>,
    pub tags: Option<Vec<String>>,
}

impl Default for CharacterUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl CharacterUpdate {
    pub fn new() -> Self {
        CharacterUpdate {
            name: None,
            description: None,
            role: None,
            personality: None,
            appearance: None,
            background: None,
            custom_attributes: None,
            tags: None,
        }
    }
}

/// Partial update for a world entry.
pub struct WorldEntryUpdate {
    pub name: Option<String>,
    pub category: Option<WorldCategory>,
    pub description: Option<String>,
    pub related_entries: Option<Vec<String>>,
    pub related_characters: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

impl Default for WorldEntryUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldEntryUpdate {
    pub fn new() -> Self {
        WorldEntryUpdate {
            name: None,
            category: None,
            description: None,
            related_entries: None,
            related_characters: None,
            tags: None,
        }
    }
}

/// Partial update for a timeline event.
pub struct TimelineEventUpdate {
    pub date: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub related_characters: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

impl Default for TimelineEventUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl TimelineEventUpdate {
    pub fn new() -> Self {
        TimelineEventUpdate {
            date: None,
            title: None,
            description: None,
            related_characters: None,
            tags: None,
        }
    }
}

/// Partial update for a glossary entry.
pub struct GlossaryEntryUpdate {
    pub term: Option<String>,
    pub definition: Option<String>,
    pub synonyms: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

impl Default for GlossaryEntryUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl GlossaryEntryUpdate {
    pub fn new() -> Self {
        GlossaryEntryUpdate {
            term: None,
            definition: None,
            synonyms: None,
            tags: None,
        }
    }
}
