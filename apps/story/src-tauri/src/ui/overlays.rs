use super::state::StoryState;
use super::{commands, export};
use tench_ui::parley;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

pub(crate) fn paint_export_modal(
    p: &mut Painter<'_>,
    size: Size,
    state: &StoryState,
    theme: &Theme,
) {
    let modal = Rect::new(
        size.width / 2.0 - 170.0,
        size.height / 2.0 - 140.0,
        size.width / 2.0 + 170.0,
        size.height / 2.0 + 140.0,
    );
    p.fill_rounded_rect(modal, theme.surface, theme.border_radius);
    p.stroke_rounded_rect(modal, theme.border, 1.0, theme.border_radius);
    p.draw_text(
        "Export Story",
        modal.x0 + 16.0,
        modal.y0 + 28.0,
        theme.on_surface,
        16.0,
        parley::FontWeight::BOLD,
        false,
    );
    p.draw_text(
        &format!(
            "{} | {}",
            state.project_name(),
            export::word_count_label(state.total_word_count())
        ),
        modal.x0 + 16.0,
        modal.y0 + 54.0,
        theme.secondary,
        12.0,
        parley::FontWeight::NORMAL,
        false,
    );
    let mut y = modal.y0 + 86.0;
    for format in commands::export_formats() {
        let row = Rect::new(modal.x0 + 16.0, y - 14.0, modal.x1 - 16.0, y + 16.0);
        p.fill_rounded_rect(row, theme.background, 6.0);
        p.draw_text(
            format,
            row.x0 + 12.0,
            y + 4.0,
            theme.on_surface,
            12.0,
            parley::FontWeight::BOLD,
            false,
        );
        y += 36.0;
    }
}

pub(crate) fn paint_command_palette(p: &mut Painter<'_>, size: Size, theme: &Theme) {
    let palette = Rect::new(
        size.width / 2.0 - 210.0,
        76.0,
        size.width / 2.0 + 210.0,
        440.0,
    );
    p.fill_rounded_rect(palette, theme.surface, theme.border_radius);
    p.stroke_rounded_rect(palette, theme.border, 1.0, theme.border_radius);
    p.draw_text(
        "Command Palette",
        palette.x0 + 16.0,
        palette.y0 + 28.0,
        theme.on_surface,
        16.0,
        parley::FontWeight::BOLD,
        false,
    );
    let mut y = palette.y0 + 60.0;
    for command in commands::command_labels() {
        p.draw_text(
            command,
            palette.x0 + 18.0,
            y,
            theme.on_surface,
            12.0,
            parley::FontWeight::NORMAL,
            false,
        );
        y += 22.0;
    }
}
