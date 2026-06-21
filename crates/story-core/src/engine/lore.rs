use super::*;

impl StoryEngine {
    // ----- character CRUD -----

    pub fn add_character(&mut self, name: &str) -> String {
        self.push_undo();
        let id = self.next_id();
        let character = Character::new(&id, name);
        self.document.characters.push(character);
        self.dirty = true;
        id
    }

    pub fn delete_character(&mut self, character_id: &str) -> bool {
        self.push_undo();
        let before = self.document.characters.len();
        self.document.characters.retain(|c| c.id != character_id);
        // Also remove relationships involving this character.
        self.document
            .relationships
            .retain(|r| r.character_a_id != character_id && r.character_b_id != character_id);
        self.dirty = true;
        self.document.characters.len() < before
    }

    pub fn update_character(&mut self, character_id: &str, update: CharacterUpdate) -> bool {
        self.push_undo();
        if let Some(ch) = self
            .document
            .characters
            .iter_mut()
            .find(|c| c.id == character_id)
        {
            if let Some(name) = update.name {
                ch.name = name;
            }
            if let Some(description) = update.description {
                ch.description = description;
            }
            if let Some(role) = update.role {
                ch.role = role;
            }
            if let Some(personality) = update.personality {
                ch.personality = personality;
            }
            if let Some(appearance) = update.appearance {
                ch.appearance = appearance;
            }
            if let Some(background) = update.background {
                ch.background = background;
            }
            if let Some(attrs) = update.custom_attributes {
                ch.custom_attributes = attrs;
            }
            if let Some(tags) = update.tags {
                ch.tags = tags;
            }
            self.dirty = true;
            true
        } else {
            false
        }
    }

    pub fn add_character_appearance(&mut self, character_id: &str, chapter_id: &str) -> bool {
        self.push_undo();
        if let Some(ch) = self
            .document
            .characters
            .iter_mut()
            .find(|c| c.id == character_id)
        {
            if !ch.appearance_chapters.contains(&chapter_id.to_string()) {
                ch.appearance_chapters.push(chapter_id.to_string());
            }
            self.dirty = true;
            true
        } else {
            false
        }
    }

    pub fn get_character(&self, character_id: &str) -> Option<&Character> {
        self.document
            .characters
            .iter()
            .find(|c| c.id == character_id)
    }

    // ----- relationship CRUD -----

    pub fn add_relationship(&mut self, a_id: &str, b_id: &str, kind: &str) -> String {
        self.push_undo();
        let id = self.next_id();
        let rel = CharacterRelationship::new(&id, a_id, b_id, kind);
        self.document.relationships.push(rel);
        self.dirty = true;
        id
    }

    pub fn delete_relationship(&mut self, relationship_id: &str) -> bool {
        self.push_undo();
        let before = self.document.relationships.len();
        self.document
            .relationships
            .retain(|r| r.id != relationship_id);
        self.dirty = true;
        self.document.relationships.len() < before
    }

    // ----- world entry CRUD -----

    pub fn add_world_entry(&mut self, name: &str, category: WorldCategory) -> String {
        self.push_undo();
        let id = self.next_id();
        let mut entry = WorldEntry::new(&id, name);
        entry.category = category;
        self.document.world_entries.push(entry);
        self.dirty = true;
        id
    }

    pub fn delete_world_entry(&mut self, entry_id: &str) -> bool {
        self.push_undo();
        let before = self.document.world_entries.len();
        self.document.world_entries.retain(|e| e.id != entry_id);
        self.dirty = true;
        self.document.world_entries.len() < before
    }

    pub fn update_world_entry(&mut self, entry_id: &str, update: WorldEntryUpdate) -> bool {
        self.push_undo();
        if let Some(entry) = self
            .document
            .world_entries
            .iter_mut()
            .find(|e| e.id == entry_id)
        {
            if let Some(name) = update.name {
                entry.name = name;
            }
            if let Some(category) = update.category {
                entry.category = category;
            }
            if let Some(description) = update.description {
                entry.description = description;
            }
            if let Some(related_entries) = update.related_entries {
                entry.related_entries = related_entries;
            }
            if let Some(related_characters) = update.related_characters {
                entry.related_characters = related_characters;
            }
            if let Some(tags) = update.tags {
                entry.tags = tags;
            }
            self.dirty = true;
            true
        } else {
            false
        }
    }

    // ----- timeline event CRUD -----

    pub fn add_timeline_event(&mut self, date: &str, title: &str) -> String {
        self.push_undo();
        let id = self.next_id();
        let event = TimelineEvent::new(&id, date, title);
        self.document.timeline_events.push(event);
        self.dirty = true;
        id
    }

    pub fn delete_timeline_event(&mut self, event_id: &str) -> bool {
        self.push_undo();
        let before = self.document.timeline_events.len();
        self.document.timeline_events.retain(|e| e.id != event_id);
        self.dirty = true;
        self.document.timeline_events.len() < before
    }

    pub fn update_timeline_event(&mut self, event_id: &str, update: TimelineEventUpdate) -> bool {
        self.push_undo();
        if let Some(event) = self
            .document
            .timeline_events
            .iter_mut()
            .find(|e| e.id == event_id)
        {
            if let Some(date) = update.date {
                event.date = date;
            }
            if let Some(title) = update.title {
                event.title = title;
            }
            if let Some(description) = update.description {
                event.description = description;
            }
            if let Some(related_characters) = update.related_characters {
                event.related_characters = related_characters;
            }
            if let Some(tags) = update.tags {
                event.tags = tags;
            }
            self.dirty = true;
            true
        } else {
            false
        }
    }

    // ----- glossary CRUD -----

    pub fn add_glossary_entry(&mut self, term: &str, definition: &str) -> String {
        self.push_undo();
        let id = self.next_id();
        let mut entry = GlossaryEntry::new(&id, term);
        entry.definition = definition.to_string();
        self.document.glossary.push(entry);
        self.dirty = true;
        id
    }

    pub fn delete_glossary_entry(&mut self, entry_id: &str) -> bool {
        self.push_undo();
        let before = self.document.glossary.len();
        self.document.glossary.retain(|e| e.id != entry_id);
        self.dirty = true;
        self.document.glossary.len() < before
    }

    pub fn update_glossary_entry(&mut self, entry_id: &str, update: GlossaryEntryUpdate) -> bool {
        self.push_undo();
        if let Some(entry) = self.document.glossary.iter_mut().find(|e| e.id == entry_id) {
            if let Some(term) = update.term {
                entry.term = term;
            }
            if let Some(definition) = update.definition {
                entry.definition = definition;
            }
            if let Some(synonyms) = update.synonyms {
                entry.synonyms = synonyms;
            }
            if let Some(tags) = update.tags {
                entry.tags = tags;
            }
            self.dirty = true;
            true
        } else {
            false
        }
    }
}
