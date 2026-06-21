use tench_ui_automation_core::{
    UiAutomationAction, UiAutomationCapture, UiAutomationKey, UiAutomationModifiers,
    UiAutomationSelector,
};
use tench_ui_test::{
    assert_capture_changed, harness::HarnessConfig, CaptureAssertions, TestHarness,
};
use tench_universe_lib::ui::UniverseApp;

fn harness() -> TestHarness {
    TestHarness::with_config(
        UniverseApp::new(),
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

#[test]
fn universe_plan_primary_controls_modals_and_automatic_nodes_use_real_events_ui_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);
    initial.assert_png_size(1280, 720);
    initial.assert_nonblank();

    for debug_id in [
        "universe.mode.chat",
        "universe.mode.novel",
        "universe.mode.interactive",
        "universe.mode.script",
        "universe.header.settings",
        "universe.character.search",
        "universe.character.new",
        "universe.template_picker",
        "universe.character.0",
        "universe.character.selected",
        "universe.history",
        "universe.left.settings",
        "universe.composer.input",
        "universe.composer.send",
        "universe.composer.placeholder",
        "universe.persona.edit",
        "universe.memory.0",
        "universe.active_content",
        "universe.right.persona_detail",
        "universe.local_runtime_status",
    ] {
        initial.assert_selector_present(&selector(debug_id));
    }

    let novel = click(&mut harness, "universe.mode.novel");
    novel.assert_selector_present(&selector("universe.toast"));
    assert_capture_changed(&initial, &novel);

    let selected = click(&mut harness, "universe.character.1");
    selected.assert_selector_present(&selector("universe.character.selected"));
    selected.assert_selector_present(&selector("universe.toast"));
    assert_capture_changed(&novel, &selected);

    let settings = click(&mut harness, "universe.header.settings");
    for debug_id in [
        "universe.modal.backdrop",
        "universe.modal.surface",
        "universe.modal.close",
        "universe.modal.done",
    ] {
        settings.assert_selector_present(&selector(debug_id));
    }
    assert_capture_changed(&selected, &settings);

    let closed = click(&mut harness, "universe.modal.done");
    closed.assert_selector_absent(&selector("universe.modal.surface"));
    assert_capture_changed(&settings, &closed);

    let new_character = click(&mut harness, "universe.character.new");
    new_character.assert_selector_present(&selector("universe.modal.surface"));
    let closed_new = click(&mut harness, "universe.modal.close");
    closed_new.assert_selector_absent(&selector("universe.modal.surface"));

    let template = click(&mut harness, "universe.template_picker");
    template.assert_selector_present(&selector("universe.modal.surface"));
    let closed_template = click(&mut harness, "universe.modal.backdrop");
    closed_template.assert_selector_absent(&selector("universe.modal.surface"));

    let history = click(&mut harness, "universe.history");
    history.assert_selector_present(&selector("universe.modal.surface"));
    click(&mut harness, "universe.modal.done")
        .assert_selector_absent(&selector("universe.modal.surface"));

    let persona = click(&mut harness, "universe.persona.edit");
    persona.assert_selector_present(&selector("universe.modal.surface"));

    // Plan 01: Left settings button opens settings modal
    let left_closed = click(&mut harness, "universe.modal.close");
    left_closed.assert_selector_absent(&selector("universe.modal.surface"));
    let left_settings = click(&mut harness, "universe.left.settings");
    left_settings.assert_selector_present(&selector("universe.modal.surface"));
    left_settings.assert_selector_present(&selector("universe.modal.close"));

    // Plan 02: Memory click verification
    click(&mut harness, "universe.modal.done");
    let mem = click(&mut harness, "universe.memory.0");
    mem.assert_selector_present(&selector("universe.toast"));
}

#[test]
fn universe_plan_search_composer_send_and_shortcuts_use_real_events_ui_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);

    let searched = type_text(&mut harness, "universe.character.search", "ari");
    searched.assert_selector_present(&selector("universe.character.search"));
    assert_capture_changed(&initial, &searched);

    let composer = type_text(&mut harness, "universe.composer.input", "hello ari");
    composer.assert_selector_present(&selector("universe.composer.input"));
    assert_capture_changed(&searched, &composer);

    let backed = key(
        &mut harness,
        UiAutomationKey::Backspace,
        UiAutomationModifiers::default(),
    );
    assert_capture_changed(&composer, &backed);

    let resent = type_text(&mut harness, "universe.composer.input", "!");
    let sent = click(&mut harness, "universe.composer.send");
    sent.assert_selector_present(&selector("universe.chat.message.0"));
    sent.assert_selector_present(&selector("universe.chat.message.1"));
    assert_capture_changed(&resent, &sent);

    let enter_ready = type_text(&mut harness, "universe.composer.input", "second");
    let enter_sent = key(
        &mut harness,
        UiAutomationKey::Enter,
        UiAutomationModifiers::default(),
    );
    enter_sent.assert_selector_present(&selector("universe.chat.message.2"));
    assert_capture_changed(&enter_ready, &enter_sent);
}

#[test]
fn universe_plan_interactive_choices_render_selection_and_toast_ui_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);
    let interactive = click(&mut harness, "universe.mode.interactive");
    assert_capture_changed(&initial, &interactive);

    for debug_id in [
        "universe.choice.open_gate",
        "universe.choice.call_ari_over",
        "universe.choice.walk_away",
        "universe.choice.touch_seal",
    ] {
        interactive.assert_selector_present(&selector(debug_id));
    }

    let selected = click(&mut harness, "universe.choice.open_gate");
    selected.assert_selector_present(&selector("universe.choice.selected"));
    selected.assert_selector_present(&selector("universe.toast"));
    assert_capture_changed(&interactive, &selected);

    let script = click(&mut harness, "universe.mode.script");
    script.assert_selector_present(&selector("universe.active_content"));
    assert_capture_changed(&selected, &script);
}
