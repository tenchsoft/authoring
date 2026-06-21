use tench_story_lib::ui::{
    state::{StoryInputFocus, StoryState, StoryTab},
    StoryApp,
};
use tench_ui_automation_core::{
    UiAutomationAction, UiAutomationCapture, UiAutomationKey, UiAutomationModifiers,
    UiAutomationSelector,
};
use tench_ui_test::{
    assert_capture_changed, harness::HarnessConfig, CaptureAssertions, TestHarness,
};

fn harness() -> TestHarness {
    TestHarness::with_config(
        StoryApp::with_state(StoryState::example()),
        HarnessConfig::with_viewport(1280.0, 720.0),
    )
}

fn selector(debug_id: &str) -> UiAutomationSelector {
    UiAutomationSelector::debug_id(debug_id)
}

fn capture(harness: &mut TestHarness) -> UiAutomationCapture {
    harness.automation_capture(Default::default())
}

fn click(harness: &mut TestHarness, debug_id: &str) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::Click {
            selector: selector(debug_id),
            modifiers: Default::default(),
        })
        .unwrap_or_else(|error| panic!("click {debug_id}: {error:?}"))
}

fn type_text(harness: &mut TestHarness, debug_id: &str, text: &str) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::TypeText {
            selector: selector(debug_id),
            text: text.to_string(),
        })
        .unwrap_or_else(|error| panic!("type text into {debug_id}: {error:?}"))
}

fn key(
    harness: &mut TestHarness,
    key: UiAutomationKey,
    modifiers: UiAutomationModifiers,
) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::KeyPress { key, modifiers })
        .expect("key press")
}

fn app_state(harness: &mut TestHarness) -> &mut StoryState {
    harness
        .root_mut()
        .widget
        .downcast_mut::<StoryApp>()
        .expect("root is StoryApp")
        .state_mut()
}

#[test]
fn story_plan_header_chapters_panels_and_automatic_nodes_use_real_events_ui_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);
    initial.assert_png_size(1280, 720);
    initial.assert_nonblank();

    for debug_id in [
        "story.header.new",
        "story.header.open",
        "story.header.save",
        "story.header.export",
        "story.header.focus",
        "story.header.command",
        "story.chapter.0",
        "story.chapter.selected",
        "story.manuscript.editor",
        "story.cursor",
        "story.word_count",
        "story.status_bar",
        "story.tab.characters",
        "story.right_panel.content",
        "story.character.0",
    ] {
        initial.assert_selector_present(&selector(debug_id));
    }

    // Plan 03: Save state verification
    click(&mut harness, "story.header.save");
    assert_eq!(
        app_state(&mut harness).saved_at,
        "now",
        "save should update saved_at"
    );

    // Plan 02: Open project state verification
    click(&mut harness, "story.header.open");
    assert_eq!(
        app_state(&mut harness).saved_at,
        "opened",
        "open should update saved_at"
    );

    // Plan 06/18: Chapter tree row click — verify selected_chapter_idx and editor content
    let before_chapter_text = app_state(&mut harness).chapter_text();
    let chapter = click(&mut harness, "story.chapter.1");
    chapter.assert_selector_present(&selector("story.chapter.selected"));
    assert_capture_changed(&initial, &chapter);
    assert_eq!(
        app_state(&mut harness).selected_chapter_idx,
        1,
        "clicking chapter.1 should set selected_chapter_idx to 1"
    );
    assert_ne!(
        app_state(&mut harness).chapter_text(),
        before_chapter_text,
        "chapter text should change after switching chapters"
    );
    assert_eq!(
        app_state(&mut harness).chapter_title(),
        "Chapter 2",
        "editor title should reflect selected chapter"
    );

    // Plan 13: Right panel tab switching — verify active_tab state for each tab
    let expected_tabs = [
        ("story.tab.world", StoryTab::World, "story.world.0"),
        ("story.tab.timeline", StoryTab::Timeline, "story.timeline.0"),
        ("story.tab.comments", StoryTab::Comments, "story.comment.0"),
        ("story.tab.stats", StoryTab::Stats, "story.statistics.0"),
        ("story.tab.glossary", StoryTab::Glossary, "story.glossary.0"),
        (
            "story.tab.relationships",
            StoryTab::Relationships,
            "story.relationship.0",
        ),
        (
            "story.tab.mind_map",
            StoryTab::MindMap,
            "story.mind_map.premise",
        ),
        (
            "story.tab.ai_assist",
            StoryTab::AiAssist,
            "story.ai.placeholder",
        ),
    ];
    for (tab, expected_tab, row) in &expected_tabs {
        let tab_capture = click(&mut harness, tab);
        tab_capture.assert_selector_present(&selector(row));
        tab_capture.assert_selector_present(&selector("story.right_panel.content"));
        // Plan 13: Verify active_tab matches the clicked tab
        assert_eq!(
            app_state(&mut harness).active_tab,
            *expected_tab,
            "clicking {tab} should set active_tab to {expected_tab:?}"
        );
    }

    // Plan 05: AI Assist tab — click again (already active) and verify state is unchanged
    let pre_ai_state = (
        app_state(&mut harness).selected_chapter_idx,
        app_state(&mut harness).chapter_text(),
        app_state(&mut harness).focus_mode,
        app_state(&mut harness).show_export,
        app_state(&mut harness).show_command_palette,
        app_state(&mut harness).show_search,
    );
    click(&mut harness, "story.tab.ai_assist");
    assert_eq!(
        app_state(&mut harness).active_tab,
        StoryTab::AiAssist,
        "re-clicking AI tab should keep active_tab as AiAssist"
    );
    assert_eq!(
        app_state(&mut harness).selected_chapter_idx,
        pre_ai_state.0,
        "re-clicking AI tab should not change selected_chapter_idx"
    );
    assert_eq!(
        app_state(&mut harness).chapter_text(),
        pre_ai_state.1,
        "re-clicking AI tab should not change chapter text"
    );
    assert_eq!(
        app_state(&mut harness).focus_mode,
        pre_ai_state.2,
        "re-clicking AI tab should not change focus_mode"
    );

    // Plan 04: AI Assist placeholder click — verify state update
    click(&mut harness, "story.ai.placeholder");
    // Placeholder click sets saved_at = "selected ai" in current implementation
    assert_eq!(
        app_state(&mut harness).saved_at,
        "selected ai",
        "clicking AI placeholder should update saved_at"
    );

    // Plan 21: Characters tab — switch back from AI Assist and verify state
    click(&mut harness, "story.tab.characters");
    assert_eq!(
        app_state(&mut harness).active_tab,
        StoryTab::Characters,
        "clicking Characters tab should set active_tab to Characters"
    );

    // Plan 20: Character row click — verify state
    click(&mut harness, "story.character.0");
    assert_eq!(
        app_state(&mut harness).saved_at,
        "selected character.0",
        "clicking character.0 should update saved_at"
    );

    // Plan 01: New project state verification (must be last - resets state)
    click(&mut harness, "story.header.new");
    assert_eq!(
        app_state(&mut harness).chapter_count(),
        1,
        "new project should have 1 chapter"
    );
    assert!(
        app_state(&mut harness).chapter_text().is_empty(),
        "new project manuscript should be empty"
    );
    assert_eq!(
        app_state(&mut harness).selected_chapter_idx,
        0,
        "new project should select chapter 0"
    );
    assert_eq!(
        app_state(&mut harness).active_tab,
        StoryTab::Characters,
        "new project should default to Characters tab"
    );
    assert!(
        !app_state(&mut harness).focus_mode,
        "new project should not be in focus mode"
    );
    assert!(
        !app_state(&mut harness).show_export,
        "new project should not show export"
    );
    assert!(
        !app_state(&mut harness).show_command_palette,
        "new project should not show command palette"
    );
    assert!(
        !app_state(&mut harness).show_search,
        "new project should not show search"
    );

    // Plan 11: Focus mode — verify focus_mode state toggle
    let focused = click(&mut harness, "story.header.focus");
    focused.assert_selector_present(&selector("story.focus_layout"));
    focused.assert_selector_absent(&selector("story.tab.characters"));
    assert_capture_changed(&initial, &focused);
    assert!(
        app_state(&mut harness).focus_mode,
        "focus mode should be enabled after clicking focus"
    );
    // Plan 11: Verify focus mode preserves document state
    assert_eq!(
        app_state(&mut harness).chapter_count(),
        1,
        "focus mode should not change chapter count"
    );
    assert!(
        app_state(&mut harness).chapter_text().is_empty(),
        "focus mode should not change chapter text"
    );

    // Plan 11: Toggle focus mode off and verify restoration
    let unfocused = click(&mut harness, "story.header.focus");
    unfocused.assert_selector_present(&selector("story.tab.characters"));
    assert!(
        !app_state(&mut harness).focus_mode,
        "focus mode should be disabled after second click"
    );
}

#[test]
fn story_plan_export_search_command_palette_and_shortcuts_use_real_events_ui_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);

    // Plan 10: Export modal — verify show_export state
    let export = click(&mut harness, "story.header.export");
    for debug_id in [
        "story.export.modal",
        "story.export.docx",
        "story.export.pdf",
        "story.export.epub",
        "story.export.markdown",
        "story.export.html",
        "story.export.plain_text",
        "story.export.bundle",
        "story.export.backdrop",
        "story.overlay.exclusive",
    ] {
        export.assert_selector_present(&selector(debug_id));
    }
    assert_capture_changed(&initial, &export);
    assert!(
        app_state(&mut harness).show_export,
        "export modal should set show_export to true"
    );
    assert!(
        !app_state(&mut harness).show_command_palette,
        "export modal should not show command palette"
    );
    assert!(
        !app_state(&mut harness).show_search,
        "export modal should not show search"
    );

    // Plan 10: Export format click — verify modal closes
    let exported = click(&mut harness, "story.export.pdf");
    exported.assert_selector_absent(&selector("story.export.modal"));
    assert_capture_changed(&export, &exported);
    assert!(
        !app_state(&mut harness).show_export,
        "clicking export format should close export modal"
    );

    // Plan 07: Command palette — verify show_command_palette state
    let command = click(&mut harness, "story.header.command");
    for debug_id in [
        "story.command.palette",
        "story.command.new_project",
        "story.command.open_project",
        "story.command.save_project",
        "story.command.export",
        "story.command.focus_mode",
        "story.command.add_chapter",
        "story.command.delete_chapter",
        "story.command.undo",
        "story.command.redo",
        "story.command.characters_panel",
        "story.command.world_panel",
        "story.command.timeline_panel",
        "story.command.glossary_panel",
        "story.command.statistics_panel",
        "story.command.search",
        "story.command.ai_assist_panel",
        "story.command.backdrop",
    ] {
        command.assert_selector_present(&selector(debug_id));
    }
    assert!(
        app_state(&mut harness).show_command_palette,
        "command palette should set show_command_palette to true"
    );
    assert!(
        !app_state(&mut harness).show_export,
        "command palette should not show export"
    );
    assert!(
        !app_state(&mut harness).show_search,
        "command palette should not show search"
    );

    // Plan 22: Add Chapter — verify chapter count, selection, palette closure
    let chapter_count_before = app_state(&mut harness).chapter_count();
    let added = click(&mut harness, "story.command.add_chapter");
    added.assert_selector_present(&selector("story.chapter.2"));
    assert_capture_changed(&command, &added);
    assert_eq!(
        app_state(&mut harness).chapter_count(),
        chapter_count_before + 1,
        "add chapter should increment chapter count by 1"
    );
    assert_eq!(
        app_state(&mut harness).selected_chapter_idx,
        2,
        "add chapter should select the newly added chapter"
    );
    assert!(
        !app_state(&mut harness).show_command_palette,
        "add chapter should close command palette"
    );

    // Plan 24: Command palette Escape — verify palette closes without side effects
    let shortcut_command = key(
        &mut harness,
        UiAutomationKey::Character("p".to_string()),
        UiAutomationModifiers {
            control: true,
            ..UiAutomationModifiers::default()
        },
    );
    shortcut_command.assert_selector_present(&selector("story.command.palette"));
    assert!(
        app_state(&mut harness).show_command_palette,
        "Ctrl+P should open command palette"
    );

    // Plan 24: Escape closes command palette
    let escape_cmd = key(
        &mut harness,
        UiAutomationKey::Escape,
        UiAutomationModifiers::default(),
    );
    escape_cmd.assert_selector_absent(&selector("story.command.palette"));
    assert!(
        !app_state(&mut harness).show_command_palette,
        "Escape should close command palette"
    );
    assert!(
        !app_state(&mut harness).show_export,
        "Escape should not open export"
    );
    assert!(
        !app_state(&mut harness).show_search,
        "Escape should not open search"
    );

    // Plan 14: Re-open command palette to access search command
    let reopened = click(&mut harness, "story.header.command");
    reopened.assert_selector_present(&selector("story.command.palette"));
    assert!(
        app_state(&mut harness).show_command_palette,
        "command palette should be open again"
    );

    // Plan 14: Search — verify show_search state
    let search = click(&mut harness, "story.command.search");
    search.assert_selector_present(&selector("story.search.bar"));
    search.assert_selector_present(&selector("story.search.query"));
    search.assert_selector_present(&selector("story.search.case_sensitive"));
    assert!(
        app_state(&mut harness).show_search,
        "search command should set show_search to true"
    );
    assert_eq!(
        app_state(&mut harness).input_focus,
        StoryInputFocus::Search,
        "search should set input focus to Search"
    );

    // Plan 14: Search query typing — verify search_query state
    let typed = type_text(&mut harness, "story.search.query", "gate");
    assert_capture_changed(&search, &typed);
    assert_eq!(
        app_state(&mut harness).search_query,
        "gate",
        "typing should update search_query"
    );

    // Plan 14: Case sensitive toggle — verify search_case_sensitive state
    let case = click(&mut harness, "story.search.case_sensitive");
    assert_capture_changed(&typed, &case);
    assert!(
        app_state(&mut harness).search_case_sensitive,
        "clicking case sensitive should toggle to true"
    );

    // Plan 14: Escape closes search — verify input_focus returns to Manuscript
    let escaped = key(
        &mut harness,
        UiAutomationKey::Escape,
        UiAutomationModifiers::default(),
    );
    escaped.assert_selector_absent(&selector("story.search.bar"));
    assert!(
        !app_state(&mut harness).show_search,
        "Escape should close search"
    );
    assert_eq!(
        app_state(&mut harness).input_focus,
        StoryInputFocus::Manuscript,
        "Escape should restore manuscript focus"
    );
    assert_eq!(
        app_state(&mut harness).search_query,
        "gate",
        "Escape should preserve search query"
    );
}

#[test]
fn story_plan_manuscript_keyboard_dirty_wordcount_and_save_use_real_events_ui_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);

    // Plan 09/17: Typing marks dirty — verify is_dirty and word counts
    let typed = type_text(&mut harness, "story.manuscript.editor", " new words");
    typed.assert_selector_present(&selector("story.dirty_title"));
    typed.assert_selector_present(&selector("story.word_count"));
    assert_capture_changed(&initial, &typed);
    assert!(
        app_state(&mut harness).is_dirty(),
        "typing should mark document as dirty"
    );
    let word_count_after = app_state(&mut harness).chapter_word_count();
    assert!(
        word_count_after > 0,
        "word count should be positive after typing"
    );
    assert!(
        app_state(&mut harness).chapter_text().contains("new words"),
        "chapter text should contain typed text"
    );

    // Plan 08: Cursor after newline — verify newline dispatches
    let newline = key(
        &mut harness,
        UiAutomationKey::Enter,
        UiAutomationModifiers::default(),
    );
    newline.assert_selector_present(&selector("story.cursor"));
    // Note: TenchDocument::plain_text uses .lines() which strips trailing newlines,
    // so a trailing newline may not appear in chapter_text(). Verify the key was processed
    // by checking the cursor is still present and subsequent text appends correctly.

    let after_newline_text = type_text(&mut harness, "story.manuscript.editor", "z");
    assert_capture_changed(&typed, &after_newline_text);

    // Plan 08: Backspace — verify text removal
    let backspace = key(
        &mut harness,
        UiAutomationKey::Backspace,
        UiAutomationModifiers::default(),
    );
    assert_capture_changed(&after_newline_text, &backspace);
    assert!(
        !app_state(&mut harness).chapter_text().ends_with('z'),
        "backspace should remove last character"
    );

    // Plan 09: Ctrl+S clears dirty — verify is_dirty and saved_at
    let saved = key(
        &mut harness,
        UiAutomationKey::Character("s".to_string()),
        UiAutomationModifiers {
            control: true,
            ..UiAutomationModifiers::default()
        },
    );
    saved.assert_selector_absent(&selector("story.dirty_title"));
    saved.assert_selector_present(&selector("story.status_bar"));
    assert!(
        !app_state(&mut harness).is_dirty(),
        "Ctrl+S should clear dirty state"
    );
    assert_eq!(
        app_state(&mut harness).saved_at,
        "now",
        "Ctrl+S should update saved_at"
    );

    // Plan 12/27: Export shortcut — verify overlay exclusivity
    let export_shortcut = key(
        &mut harness,
        UiAutomationKey::Character("e".to_string()),
        UiAutomationModifiers {
            control: true,
            ..UiAutomationModifiers::default()
        },
    );
    export_shortcut.assert_selector_present(&selector("story.export.modal"));
    assert!(
        app_state(&mut harness).show_export,
        "Ctrl+E should open export modal"
    );
    assert!(
        !app_state(&mut harness).show_command_palette,
        "export should not show command palette"
    );
    assert!(
        !app_state(&mut harness).show_search,
        "export should not show search"
    );

    // Plan 12: Escape closes export — verify overlay cleanup
    let close_export = key(
        &mut harness,
        UiAutomationKey::Escape,
        UiAutomationModifiers::default(),
    );
    close_export.assert_selector_absent(&selector("story.export.modal"));
    assert!(
        !app_state(&mut harness).show_export,
        "Escape should close export modal"
    );
    assert!(
        !app_state(&mut harness).show_command_palette,
        "Escape should not open command palette"
    );
    assert!(
        !app_state(&mut harness).show_search,
        "Escape should not open search"
    );
    assert_eq!(
        app_state(&mut harness).input_focus,
        StoryInputFocus::Manuscript,
        "Escape should restore manuscript focus"
    );
}
