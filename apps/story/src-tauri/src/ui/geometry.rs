use super::commands;
use super::state::{StoryState, StoryTab};
use tench_ui::prelude::*;

pub(crate) fn hit_test_right_panel_row(
    pos: Point,
    state: &StoryState,
    width: f64,
) -> Option<String> {
    let left_w = if state.focus_mode { 0.0 } else { 220.0 };
    let right_w = if state.focus_mode { 0.0 } else { 300.0 };
    let center_w = width - left_w - right_w;
    let panel_x = left_w + center_w + 16.0;
    let first_row_y = 48.0 + 84.0;
    let doc = state.engine.get_document();
    let (count, step, prefix) = match state.active_tab {
        StoryTab::Characters => (doc.characters.len(), 40.0, "character"),
        StoryTab::World => (doc.world_entries.len(), 46.0, "world"),
        StoryTab::Timeline => (doc.timeline_events.len(), 44.0, "timeline"),
        StoryTab::Comments => (doc.comments.len(), 44.0, "comment"),
        StoryTab::Stats => (10, 38.0, "statistics"),
        StoryTab::Glossary => (doc.glossary.len(), 44.0, "glossary"),
        StoryTab::Relationships => (doc.relationships.len(), 44.0, "relationship"),
        StoryTab::MindMap => {
            for (idx, label) in ["premise", "conflict", "setting", "character_arc"]
                .into_iter()
                .enumerate()
            {
                let x = panel_x + (idx % 2) as f64 * 120.0;
                let y = first_row_y + (idx / 2) as f64 * 54.0;
                if Rect::new(x, y, x + 104.0, y + 38.0).contains(pos) {
                    return Some(label.to_string());
                }
            }
            return None;
        }
        StoryTab::AiAssist => {
            if panel_row_rect(panel_x, first_row_y, width, 0, 44.0).contains(pos) {
                return Some("ai".to_string());
            }
            return None;
        }
    };
    for idx in 0..count {
        if panel_row_rect(panel_x, first_row_y, width, idx, step).contains(pos) {
            return Some(format!("{prefix}.{idx}"));
        }
    }
    None
}

pub(crate) fn export_modal_rect(size: Size) -> Rect {
    Rect::new(
        size.width / 2.0 - 170.0,
        size.height / 2.0 - 140.0,
        size.width / 2.0 + 170.0,
        size.height / 2.0 + 140.0,
    )
}

pub(crate) fn export_format_rect(size: Size, index: usize) -> Rect {
    let modal = export_modal_rect(size);
    let y = modal.y0 + 86.0 + index as f64 * 36.0;
    Rect::new(modal.x0 + 16.0, y - 14.0, modal.x1 - 16.0, y + 16.0)
}

pub(crate) fn hit_test_export_format(pos: Point, size: Size) -> Option<usize> {
    (0..commands::export_formats().len()).find(|idx| export_format_rect(size, *idx).contains(pos))
}

pub(crate) fn command_palette_rect(size: Size) -> Rect {
    Rect::new(
        size.width / 2.0 - 210.0,
        76.0,
        size.width / 2.0 + 210.0,
        440.0,
    )
}

pub(crate) fn command_row_rect(size: Size, index: usize) -> Rect {
    let palette = command_palette_rect(size);
    let y = palette.y0 + 44.0 + index as f64 * 22.0;
    Rect::new(palette.x0 + 12.0, y, palette.x1 - 12.0, y + 22.0)
}

pub(crate) fn hit_test_command_row(pos: Point, size: Size) -> Option<usize> {
    (0..commands::command_labels().len()).find(|idx| command_row_rect(size, *idx).contains(pos))
}

pub(crate) fn search_bar_rect(size: Size) -> Rect {
    Rect::new(
        size.width / 2.0 - 210.0,
        52.0,
        size.width / 2.0 + 210.0,
        90.0,
    )
}

pub(crate) fn search_input_rect(size: Size) -> Rect {
    let bar = search_bar_rect(size);
    Rect::new(bar.x0 + 8.0, bar.y0 + 8.0, bar.x1 - 72.0, bar.y1 - 4.0)
}

pub(crate) fn search_case_rect(size: Size) -> Rect {
    let bar = search_bar_rect(size);
    Rect::new(bar.x1 - 68.0, bar.y0 + 8.0, bar.x1 - 8.0, bar.y1 - 4.0)
}

pub(crate) fn panel_row_rect(
    panel_x: f64,
    first_row_y: f64,
    width: f64,
    index: usize,
    step: f64,
) -> Rect {
    let y = first_row_y + index as f64 * step;
    Rect::new(panel_x, y, width - 16.0, y + 34.0)
}
