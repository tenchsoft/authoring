//! Command palette, tab definitions, and keyboard shortcut mappings.

use super::state::StoryTab;

pub const RIGHT_PANEL_TABS: [(&str, StoryTab); 9] = [
    ("Chars", StoryTab::Characters),
    ("World", StoryTab::World),
    ("Time", StoryTab::Timeline),
    ("Notes", StoryTab::Comments),
    ("Stats", StoryTab::Stats),
    ("Gloss", StoryTab::Glossary),
    ("Rel", StoryTab::Relationships),
    ("Mind", StoryTab::MindMap),
    ("AI", StoryTab::AiAssist),
];

/// Hit-test a point against the right-panel tab bar.
pub fn hit_test_tab(x: f64, y: f64, left_w: f64, center_w: f64) -> Option<StoryTab> {
    for (idx, (_, tab)) in RIGHT_PANEL_TABS.iter().enumerate() {
        let tx = left_w + center_w + 8.0 + idx as f64 * 32.0;
        if x >= tx && x <= tx + 30.0 && (48.0..=68.0).contains(&y) {
            return Some(*tab);
        }
    }
    None
}

/// Labels shown in the command palette.
pub fn command_labels() -> [&'static str; 16] {
    [
        "New project",
        "Open project",
        "Save project",
        "Export",
        "Focus mode",
        "Add chapter",
        "Delete chapter",
        "Undo",
        "Redo",
        "Characters panel",
        "World building panel",
        "Timeline panel",
        "Glossary panel",
        "Statistics panel",
        "Search (Ctrl+F)",
        "AI assist panel",
    ]
}

/// Export format labels shown in the export modal.
pub fn export_formats() -> [&'static str; 7] {
    [
        "DOCX (.docx)",
        "PDF (.pdf)",
        "EPUB (.epub)",
        "Markdown (.md)",
        "HTML (.html)",
        "Plain Text (.txt)",
        "Tench Story Bundle (.tench-story)",
    ]
}
