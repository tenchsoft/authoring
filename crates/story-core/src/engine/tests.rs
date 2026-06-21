use super::*;
use tench_document_core::TenchDocument;

#[test]
fn chapter_crud() {
    let mut engine = StoryEngine::new_empty("Test");
    let ch1 = engine.add_chapter("Chapter 1");
    let ch2 = engine.add_chapter("Chapter 2");
    assert_eq!(engine.get_document().chapters.len(), 2);
    assert_eq!(engine.get_chapter(&ch1).unwrap().title, "Chapter 1");

    engine.rename_chapter(&ch1, "The Beginning");
    assert_eq!(engine.get_chapter(&ch1).unwrap().title, "The Beginning");

    engine.delete_chapter(&ch2);
    assert_eq!(engine.get_document().chapters.len(), 1);
}

#[test]
fn chapter_reorder() {
    let mut engine = StoryEngine::new_empty("Test");
    let ch1 = engine.add_chapter("Ch1");
    let ch2 = engine.add_chapter("Ch2");
    let ch3 = engine.add_chapter("Ch3");

    engine.move_chapter(&ch3, 0);
    let ids: Vec<&str> = engine
        .get_document()
        .chapters
        .iter()
        .map(|c| c.id.as_str())
        .collect();
    assert_eq!(ids, vec![ch3.as_str(), ch1.as_str(), ch2.as_str()]);
}

#[test]
fn character_crud() {
    let mut engine = StoryEngine::new_empty("Test");
    let c1 = engine.add_character("Alice");
    let c2 = engine.add_character("Bob");

    engine.update_character(
        &c1,
        CharacterUpdate {
            role: Some("Protagonist".to_string()),
            ..CharacterUpdate::new()
        },
    );
    assert_eq!(engine.get_character(&c1).unwrap().role, "Protagonist");

    let rel = engine.add_relationship(&c1, &c2, "friends");
    assert_eq!(engine.get_document().relationships.len(), 1);

    engine.delete_character(&c2);
    assert_eq!(engine.get_document().characters.len(), 1);
    // Relationship should be removed too.
    assert!(engine
        .get_document()
        .relationships
        .iter()
        .all(|r| r.id != rel));
}

#[test]
fn world_entry_crud() {
    let mut engine = StoryEngine::new_empty("Test");
    let e1 = engine.add_world_entry("Capital", WorldCategory::Place);
    engine.update_world_entry(
        &e1,
        WorldEntryUpdate {
            description: Some("A layered city.".to_string()),
            ..WorldEntryUpdate::new()
        },
    );
    assert_eq!(
        engine.get_document().world_entries[0].description,
        "A layered city."
    );
    engine.delete_world_entry(&e1);
    assert!(engine.get_document().world_entries.is_empty());
}

#[test]
fn timeline_event_crud() {
    let mut engine = StoryEngine::new_empty("Test");
    let ev = engine.add_timeline_event("Year 0", "The signal arrives");
    engine.update_timeline_event(
        &ev,
        TimelineEventUpdate {
            description: Some("A mysterious signal from deep space.".to_string()),
            ..TimelineEventUpdate::new()
        },
    );
    assert_eq!(
        engine.get_document().timeline_events[0].description,
        "A mysterious signal from deep space."
    );
}

#[test]
fn glossary_crud() {
    let mut engine = StoryEngine::new_empty("Test");
    let g = engine.add_glossary_entry("Aster Gate", "A transit point between districts.");
    assert_eq!(engine.get_document().glossary[0].term, "Aster Gate");
    engine.delete_glossary_entry(&g);
    assert!(engine.get_document().glossary.is_empty());
}

#[test]
fn search_across_chapters() {
    let mut engine = StoryEngine::new_empty("Test");
    let ch1 = engine.add_chapter("Ch1");
    engine.set_chapter_content(
        &ch1,
        TenchDocument::plain_text("The dragon flew over the mountains."),
    );
    let ch2 = engine.add_chapter("Ch2");
    engine.set_chapter_content(
        &ch2,
        TenchDocument::plain_text("A dragon appeared in the valley."),
    );

    let matches = engine.search("dragon", true);
    assert_eq!(matches.len(), 2);
}

#[test]
fn replace_all() {
    let mut engine = StoryEngine::new_empty("Test");
    let ch1 = engine.add_chapter("Ch1");
    engine.set_chapter_content(&ch1, TenchDocument::plain_text("foo bar foo baz foo"));

    let count = engine.replace_all("foo", "qux", true);
    assert_eq!(count, 3);
    assert_eq!(
        engine.get_chapter(&ch1).unwrap().content.to_plain_text(),
        "qux bar qux baz qux"
    );
}

#[test]
fn undo_redo() {
    let mut engine = StoryEngine::new_empty("Test");
    engine.add_chapter("Ch1");
    assert!(engine.can_undo());
    assert!(!engine.can_redo());

    engine.undo();
    assert!(engine.get_document().chapters.is_empty());
    assert!(engine.can_redo());

    engine.redo();
    assert_eq!(engine.get_document().chapters.len(), 1);
}

#[test]
fn statistics() {
    let mut engine = StoryEngine::new_empty("Test");
    engine.add_chapter("Ch1");
    let ch_id = engine.get_document().chapters[0].id.clone();
    engine.set_chapter_content(&ch_id, TenchDocument::plain_text("Hello world test"));
    engine.add_character("Alice");

    let stats = engine.statistics();
    assert_eq!(stats.total_words, 3);
    assert_eq!(stats.chapter_count, 1);
    assert_eq!(stats.character_count, 1);
}

#[test]
fn version_history() {
    let mut engine = StoryEngine::new_empty("Test");
    engine.add_chapter("Ch1");
    let v1 = engine.save_version("Initial");

    engine.add_chapter("Ch2");
    assert_eq!(engine.get_document().chapters.len(), 2);

    engine.restore_version(&v1);
    assert_eq!(engine.get_document().chapters.len(), 1);
}

#[test]
fn bookmarks_and_comments() {
    let mut engine = StoryEngine::new_empty("Test");
    let ch1 = engine.add_chapter("Ch1");

    let bm = engine.add_bookmark(&ch1, 0, 5, "Important");
    assert_eq!(engine.get_document().bookmarks.len(), 1);

    let cmt = engine.add_comment(&ch1, "Fix this paragraph");
    assert_eq!(engine.get_document().comments.len(), 1);

    engine.resolve_comment(&cmt);
    assert!(engine.get_document().comments[0].resolved);

    engine.delete_bookmark(&bm);
    assert!(engine.get_document().bookmarks.is_empty());
}
