//! Right panel rendering — characters, world, timeline, glossary, stats, etc.
//!
//! All data is sourced from the StoryEngine via StoryState.

use tench_ui::parley;
use tench_ui::prelude::{Painter, Rect, Theme};

use super::state::{StoryState, StoryTab};

/// Paint the auxiliary panel background.
pub fn paint_aux_panel_background(p: &mut Painter<'_>, theme: &Theme, rect: Rect) {
    p.fill_rect(rect, theme.surface);
}

/// Paint the full right-panel content for the active tab.
pub fn paint_tab_content(
    p: &mut Painter<'_>,
    state: &StoryState,
    panel_x: f64,
    content_y: f64,
    width: f64,
    theme: &Theme,
) {
    let mut row_y = content_y + 44.0;
    let doc = state.engine.get_document();

    match state.active_tab {
        StoryTab::Characters => {
            paint_panel_title(p, panel_x, content_y, "Characters", theme);
            for ch in &doc.characters {
                paint_panel_row(p, panel_x, row_y, width, &ch.name, &ch.role, theme);
                row_y += 40.0;
            }
        }
        StoryTab::World => {
            paint_panel_title(p, panel_x, content_y, "World Building", theme);
            for entry in &doc.world_entries {
                let cat = format!("{:?}", entry.category);
                paint_panel_row(p, panel_x, row_y, width, &entry.name, &cat, theme);
                row_y += 46.0;
            }
        }
        StoryTab::Timeline => {
            paint_panel_title(p, panel_x, content_y, "Timeline", theme);
            for event in &doc.timeline_events {
                paint_panel_row(p, panel_x, row_y, width, &event.date, &event.title, theme);
                row_y += 44.0;
            }
        }
        StoryTab::Comments => {
            paint_panel_title(p, panel_x, content_y, "Comments", theme);
            for comment in &doc.comments {
                let label = if comment.resolved { "Resolved" } else { "Open" };
                paint_panel_row(p, panel_x, row_y, width, label, &comment.text, theme);
                row_y += 44.0;
            }
        }
        StoryTab::Stats => {
            paint_panel_title(p, panel_x, content_y, "Statistics", theme);
            let stats = state.engine.statistics();
            let rows = [
                ("Total Words", stats.total_words.to_string()),
                ("Characters", stats.total_characters.to_string()),
                ("Sentences", stats.total_sentences.to_string()),
                (
                    "Avg Sentence",
                    format!("{:.1} words", stats.avg_sentence_length),
                ),
                (
                    "Reading Time",
                    format!("{} min", stats.reading_time_minutes),
                ),
                ("Chapters", stats.chapter_count.to_string()),
                ("Character Entries", stats.character_count.to_string()),
                ("World Entries", stats.world_entry_count.to_string()),
                ("Timeline Events", stats.timeline_event_count.to_string()),
                ("Glossary Terms", stats.glossary_entry_count.to_string()),
            ];
            for (label, value) in &rows {
                paint_panel_row(p, panel_x, row_y, width, label, value, theme);
                row_y += 38.0;
            }
        }
        StoryTab::Glossary => {
            paint_panel_title(p, panel_x, content_y, "Glossary", theme);
            for entry in &doc.glossary {
                paint_panel_row(
                    p,
                    panel_x,
                    row_y,
                    width,
                    &entry.term,
                    &entry.definition,
                    theme,
                );
                row_y += 44.0;
            }
        }
        StoryTab::Relationships => {
            paint_panel_title(p, panel_x, content_y, "Relationships", theme);
            for rel in &doc.relationships {
                // Look up character names.
                let a_name = doc
                    .characters
                    .iter()
                    .find(|c| c.id == rel.character_a_id)
                    .map(|c| c.name.as_str())
                    .unwrap_or("?");
                let b_name = doc
                    .characters
                    .iter()
                    .find(|c| c.id == rel.character_b_id)
                    .map(|c| c.name.as_str())
                    .unwrap_or("?");
                paint_panel_row(
                    p,
                    panel_x,
                    row_y,
                    width,
                    &format!("{a_name} -> {b_name}"),
                    &rel.kind,
                    theme,
                );
                row_y += 44.0;
            }
        }
        StoryTab::MindMap => {
            paint_panel_title(p, panel_x, content_y, "Mind Map", theme);
            for (idx, node) in ["Premise", "Conflict", "Setting", "Character arc"]
                .iter()
                .enumerate()
            {
                let x = panel_x + (idx % 2) as f64 * 120.0;
                let y = row_y + (idx / 2) as f64 * 54.0;
                p.fill_rounded_rect(Rect::new(x, y, x + 104.0, y + 38.0), theme.background, 6.0);
                p.draw_text(
                    node,
                    x + 52.0,
                    y + 23.0,
                    theme.on_surface,
                    11.0,
                    parley::FontWeight::BOLD,
                    true,
                );
            }
        }
        StoryTab::AiAssist => {
            paint_panel_title(p, panel_x, content_y, "AI Assist", theme);
            p.draw_text(
                "AI features will be added in a future phase.",
                panel_x + 12.0,
                row_y + 14.0,
                theme.secondary,
                12.0,
                parley::FontWeight::NORMAL,
                false,
            );
        }
    }
}

fn paint_panel_title(p: &mut Painter<'_>, x: f64, content_y: f64, title: &str, theme: &Theme) {
    p.draw_text(
        title,
        x,
        content_y + 20.0,
        theme.on_background,
        14.0,
        parley::FontWeight::BOLD,
        false,
    );
}

fn paint_panel_row(
    p: &mut Painter<'_>,
    x: f64,
    y: f64,
    width: f64,
    title: &str,
    detail: &str,
    theme: &Theme,
) {
    p.fill_rounded_rect(
        Rect::new(x, y, width - 16.0, y + 34.0),
        theme.background,
        6.0,
    );
    p.draw_text(
        title,
        x + 12.0,
        y + 14.0,
        theme.on_background,
        13.0,
        parley::FontWeight::NORMAL,
        false,
    );
    p.draw_text(
        detail,
        x + 12.0,
        y + 28.0,
        theme.on_surface,
        11.0,
        parley::FontWeight::NORMAL,
        false,
    );
}
