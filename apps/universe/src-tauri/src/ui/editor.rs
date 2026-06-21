use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use super::state::{
    UniverseState, ACCENT_UNIVERSE, NEUTRAL_100, NEUTRAL_300, NEUTRAL_500, NEUTRAL_600,
    NEUTRAL_700, NEUTRAL_900,
};

pub fn paint_modals(p: &mut Painter<'_>, state: &UniverseState, size: Size) {
    let title = if state.show_character_editor {
        Some("Character Editor")
    } else if state.show_persona_editor {
        Some("Persona Editor")
    } else if state.show_sessions {
        Some("Conversation Sessions")
    } else if state.show_template_picker {
        Some("Template Picker")
    } else if state.show_settings {
        Some("Settings")
    } else {
        None
    };
    let Some(title) = title else {
        return;
    };

    let backdrop = Rect::new(0.0, 0.0, size.width, size.height);
    p.fill_rect(backdrop, Color::rgba8(0, 0, 0, 140));

    let modal = modal_rect(size);
    p.fill_rounded_rect(modal, NEUTRAL_700, 8.0);
    p.stroke_rounded_rect(modal, NEUTRAL_500, 1.0, 8.0);
    p.draw_text(
        title,
        modal.x0 + 18.0,
        modal.y0 + 34.0,
        NEUTRAL_100,
        18.0,
        FontWeight::BOLD,
        false,
    );

    let close = modal_close_rect(size);
    p.stroke_rounded_rect(close, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        "x",
        close.x0 + 10.0,
        close.y0 + 21.0,
        NEUTRAL_300,
        14.0,
        FontWeight::BOLD,
        false,
    );

    p.draw_text(
        "Native dialog surface is wired to the same UI state as the React modal.",
        modal.x0 + 18.0,
        modal.y0 + 74.0,
        NEUTRAL_300,
        13.0,
        FontWeight::NORMAL,
        false,
    );

    let action = Rect::new(
        modal.x1 - 120.0,
        modal.y1 - 54.0,
        modal.x1 - 18.0,
        modal.y1 - 18.0,
    );
    p.fill_rounded_rect(action, ACCENT_UNIVERSE, 6.0);
    p.draw_text(
        "Done",
        action.x0 + 34.0,
        action.y0 + 24.0,
        NEUTRAL_900,
        13.0,
        FontWeight::BOLD,
        false,
    );

    let panel = Rect::new(
        modal.x0 + 18.0,
        modal.y0 + 100.0,
        modal.x1 - 18.0,
        modal.y1 - 70.0,
    );
    p.fill_rounded_rect(panel, NEUTRAL_600, 6.0);
}

pub fn modal_close_rect(size: Size) -> Rect {
    let modal = modal_rect(size);
    Rect::new(
        modal.x1 - 44.0,
        modal.y0 + 14.0,
        modal.x1 - 16.0,
        modal.y0 + 42.0,
    )
}

pub fn modal_done_rect(size: Size) -> Rect {
    let modal = modal_rect(size);
    Rect::new(
        modal.x1 - 120.0,
        modal.y1 - 54.0,
        modal.x1 - 18.0,
        modal.y1 - 18.0,
    )
}

pub fn modal_rect(size: Size) -> Rect {
    let w = 480.0_f64.min(size.width - 48.0).max(280.0);
    let h = 300.0_f64.min(size.height - 48.0).max(220.0);
    let x = (size.width - w) / 2.0;
    let y = (size.height - h) / 2.0;
    Rect::new(x, y, x + w, y + h)
}
