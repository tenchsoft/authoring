use super::state::{StoryState, StoryTab};
use super::{commands, geometry};
use tench_ui::prelude::*;
use tench_ui::{UiAutomationNode, UiAutomationRect};

pub(crate) fn story_automation_nodes(
    state: &StoryState,
    size: Size,
    base_id: u64,
) -> Vec<UiAutomationNode> {
    let mut nodes = Vec::new();
    let mut next_id = base_id.saturating_mul(1000).saturating_add(1);

    for (idx, (label, debug_id)) in [
        ("New", "story.header.new"),
        ("Open", "story.header.open"),
        ("Save", "story.header.save"),
        ("Export", "story.header.export"),
        ("Focus", "story.header.focus"),
        ("Command", "story.header.command"),
    ]
    .into_iter()
    .enumerate()
    {
        push_story_node(
            &mut nodes,
            &mut next_id,
            "button",
            label,
            debug_id,
            Rect::new(
                230.0 + idx as f64 * 64.0,
                10.0,
                288.0 + idx as f64 * 64.0,
                38.0,
            ),
        );
    }

    push_story_node(
        &mut nodes,
        &mut next_id,
        "status",
        "Word count",
        "story.word_count",
        Rect::new(size.width - 180.0, 10.0, size.width - 16.0, 38.0),
    );
    if state.is_dirty() {
        push_story_node(
            &mut nodes,
            &mut next_id,
            "status",
            "Dirty title",
            "story.dirty_title",
            Rect::new(12.0, 8.0, 210.0, 40.0),
        );
    }

    let panel_y = 48.0;
    let status_h = 24.0;
    let panel_h = size.height - panel_y - status_h;
    let left_w = if state.focus_mode { 0.0 } else { 220.0 };
    let right_w = if state.focus_mode { 0.0 } else { 300.0 };
    let center_w = size.width - left_w - right_w;

    if state.focus_mode {
        push_story_node(
            &mut nodes,
            &mut next_id,
            "status",
            "Focus layout",
            "story.focus_layout",
            Rect::new(0.0, panel_y, size.width, size.height - status_h),
        );
    }

    if !state.focus_mode {
        for idx in 0..state.chapter_count() {
            push_story_node(
                &mut nodes,
                &mut next_id,
                "button",
                format!("Chapter {idx}"),
                format!("story.chapter.{idx}"),
                Rect::new(
                    8.0,
                    panel_y + 44.0 + idx as f64 * 36.0,
                    left_w - 8.0,
                    panel_y + 74.0 + idx as f64 * 36.0,
                ),
            );
        }
        push_story_node(
            &mut nodes,
            &mut next_id,
            "status",
            "Selected chapter",
            "story.chapter.selected",
            Rect::new(
                8.0,
                panel_y + 44.0 + state.selected_chapter_idx as f64 * 36.0,
                left_w - 8.0,
                panel_y + 74.0 + state.selected_chapter_idx as f64 * 36.0,
            ),
        );
    }

    push_story_node(
        &mut nodes,
        &mut next_id,
        "textbox",
        "Manuscript",
        "story.manuscript.editor",
        Rect::new(
            left_w + 16.0,
            panel_y + 16.0,
            left_w + center_w - 16.0,
            panel_y + panel_h - 16.0,
        ),
    );
    push_story_node(
        &mut nodes,
        &mut next_id,
        "status",
        "Cursor",
        "story.cursor",
        Rect::new(left_w + 28.0, panel_y + 56.0, left_w + 32.0, panel_y + 74.0),
    );
    push_story_node(
        &mut nodes,
        &mut next_id,
        "status",
        "Status bar",
        "story.status_bar",
        Rect::new(0.0, size.height - status_h, size.width, size.height),
    );

    if !state.focus_mode {
        for (idx, (label, tab)) in commands::RIGHT_PANEL_TABS.iter().enumerate() {
            push_story_node(
                &mut nodes,
                &mut next_id,
                "tab",
                *label,
                tab_debug_id(*tab),
                Rect::new(
                    left_w + center_w + 8.0 + idx as f64 * 32.0,
                    panel_y,
                    left_w + center_w + 38.0 + idx as f64 * 32.0,
                    panel_y + 20.0,
                ),
            );
        }
        push_story_node(
            &mut nodes,
            &mut next_id,
            "region",
            "Right panel content",
            "story.right_panel.content",
            Rect::new(left_w + center_w, panel_y, size.width, panel_y + panel_h),
        );
        push_panel_row_nodes(
            &mut nodes,
            &mut next_id,
            state,
            left_w + center_w + 16.0,
            panel_y + 84.0,
            size.width,
        );
    }

    if state.show_export {
        push_story_node(
            &mut nodes,
            &mut next_id,
            "button",
            "Export backdrop",
            "story.export.backdrop",
            Rect::new(0.0, 0.0, size.width, size.height),
        );
        push_story_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "Export modal",
            "story.export.modal",
            geometry::export_modal_rect(size),
        );
        for (idx, debug_id) in [
            "story.export.docx",
            "story.export.pdf",
            "story.export.epub",
            "story.export.markdown",
            "story.export.html",
            "story.export.plain_text",
            "story.export.bundle",
        ]
        .into_iter()
        .enumerate()
        {
            push_story_node(
                &mut nodes,
                &mut next_id,
                "button",
                commands::export_formats()[idx],
                debug_id,
                geometry::export_format_rect(size, idx),
            );
        }
    }

    if state.show_command_palette {
        push_story_node(
            &mut nodes,
            &mut next_id,
            "button",
            "Command backdrop",
            "story.command.backdrop",
            Rect::new(0.0, 0.0, size.width, size.height),
        );
        push_story_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "Command palette",
            "story.command.palette",
            geometry::command_palette_rect(size),
        );
        for (idx, debug_id) in command_debug_ids().into_iter().enumerate() {
            push_story_node(
                &mut nodes,
                &mut next_id,
                "button",
                commands::command_labels()[idx],
                debug_id,
                geometry::command_row_rect(size, idx),
            );
        }
    }

    if state.show_search {
        push_story_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "Search",
            "story.search.bar",
            geometry::search_bar_rect(size),
        );
        push_story_node(
            &mut nodes,
            &mut next_id,
            "textbox",
            "Search query",
            "story.search.query",
            geometry::search_input_rect(size),
        );
        push_story_node(
            &mut nodes,
            &mut next_id,
            "checkbox",
            "Case sensitive",
            "story.search.case_sensitive",
            geometry::search_case_rect(size),
        );
    }

    if state.show_export || state.show_command_palette || state.show_search {
        push_story_node(
            &mut nodes,
            &mut next_id,
            "status",
            "Overlay exclusive",
            "story.overlay.exclusive",
            Rect::new(0.0, 0.0, size.width, size.height),
        );
    }

    nodes
}

fn push_panel_row_nodes(
    nodes: &mut Vec<UiAutomationNode>,
    next_id: &mut u64,
    state: &StoryState,
    panel_x: f64,
    first_row_y: f64,
    width: f64,
) {
    let doc = state.engine.get_document();
    match state.active_tab {
        StoryTab::Characters => {
            for (idx, ch) in doc.characters.iter().enumerate() {
                push_story_node(
                    nodes,
                    next_id,
                    "button",
                    ch.name.as_str(),
                    format!("story.character.{idx}"),
                    geometry::panel_row_rect(panel_x, first_row_y, width, idx, 40.0),
                );
            }
        }
        StoryTab::World => {
            for (idx, entry) in doc.world_entries.iter().enumerate() {
                push_story_node(
                    nodes,
                    next_id,
                    "button",
                    entry.name.as_str(),
                    format!("story.world.{idx}"),
                    geometry::panel_row_rect(panel_x, first_row_y, width, idx, 46.0),
                );
            }
        }
        StoryTab::Timeline => {
            for (idx, event) in doc.timeline_events.iter().enumerate() {
                push_story_node(
                    nodes,
                    next_id,
                    "button",
                    event.title.as_str(),
                    format!("story.timeline.{idx}"),
                    geometry::panel_row_rect(panel_x, first_row_y, width, idx, 44.0),
                );
            }
        }
        StoryTab::Comments => {
            for (idx, comment) in doc.comments.iter().enumerate() {
                push_story_node(
                    nodes,
                    next_id,
                    "button",
                    comment.text.as_str(),
                    format!("story.comment.{idx}"),
                    geometry::panel_row_rect(panel_x, first_row_y, width, idx, 44.0),
                );
            }
        }
        StoryTab::Stats => {
            push_story_node(
                nodes,
                next_id,
                "status",
                "Statistics refresh",
                "story.statistics.refresh",
                geometry::panel_row_rect(panel_x, first_row_y, width, 0, 38.0),
            );
            for idx in 0..10 {
                push_story_node(
                    nodes,
                    next_id,
                    "button",
                    "Statistic",
                    format!("story.statistics.{idx}"),
                    geometry::panel_row_rect(panel_x, first_row_y, width, idx, 38.0),
                );
            }
        }
        StoryTab::Glossary => {
            for (idx, entry) in doc.glossary.iter().enumerate() {
                push_story_node(
                    nodes,
                    next_id,
                    "button",
                    entry.term.as_str(),
                    format!("story.glossary.{idx}"),
                    geometry::panel_row_rect(panel_x, first_row_y, width, idx, 44.0),
                );
            }
        }
        StoryTab::Relationships => {
            for idx in 0..doc.relationships.len() {
                push_story_node(
                    nodes,
                    next_id,
                    "button",
                    "Relationship",
                    format!("story.relationship.{idx}"),
                    geometry::panel_row_rect(panel_x, first_row_y, width, idx, 44.0),
                );
            }
        }
        StoryTab::MindMap => {
            for (idx, debug_id) in [
                "story.mind_map.premise",
                "story.mind_map.conflict",
                "story.mind_map.setting",
                "story.mind_map.character_arc",
            ]
            .into_iter()
            .enumerate()
            {
                let x = panel_x + (idx % 2) as f64 * 120.0;
                let y = first_row_y + (idx / 2) as f64 * 54.0;
                push_story_node(
                    nodes,
                    next_id,
                    "button",
                    debug_id,
                    debug_id,
                    Rect::new(x, y, x + 104.0, y + 38.0),
                );
            }
        }
        StoryTab::AiAssist => {
            push_story_node(
                nodes,
                next_id,
                "button",
                "AI assist placeholder",
                "story.ai.placeholder",
                geometry::panel_row_rect(panel_x, first_row_y, width, 0, 44.0),
            );
        }
    }
}

fn tab_debug_id(tab: StoryTab) -> &'static str {
    match tab {
        StoryTab::Characters => "story.tab.characters",
        StoryTab::World => "story.tab.world",
        StoryTab::Timeline => "story.tab.timeline",
        StoryTab::Comments => "story.tab.comments",
        StoryTab::Stats => "story.tab.stats",
        StoryTab::Glossary => "story.tab.glossary",
        StoryTab::Relationships => "story.tab.relationships",
        StoryTab::MindMap => "story.tab.mind_map",
        StoryTab::AiAssist => "story.tab.ai_assist",
    }
}

fn command_debug_ids() -> [&'static str; 16] {
    [
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
    ]
}

fn push_story_node(
    nodes: &mut Vec<UiAutomationNode>,
    next_id: &mut u64,
    role: &str,
    label: impl Into<String>,
    debug_id: impl Into<String>,
    rect: Rect,
) {
    nodes.push(UiAutomationNode {
        id: *next_id,
        debug_id: Some(debug_id.into()),
        role: role.to_string(),
        label: Some(label.into()),
        value: None,
        bounds: UiAutomationRect {
            x: rect.x0,
            y: rect.y0,
            width: rect.width(),
            height: rect.height(),
        },
        enabled: true,
        focused: false,
        hovered: false,
        children: Vec::new(),
    });
    *next_id = next_id.saturating_add(1);
}
