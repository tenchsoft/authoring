//! Editor rendering — cursor, text display, and editing helpers.

use tench_ui::prelude::{Color, Painter, Rect, Theme};

/// Paint a blinking cursor at the given position.
pub fn paint_cursor(p: &mut Painter<'_>, x: f64, y: f64, color: Color) {
    p.fill_rect(Rect::new(x, y - 12.0, x + 2.0, y + 4.0), color);
}

/// Paint the chapter content area with text lines.
pub fn paint_chapter_content(
    p: &mut Painter<'_>,
    text: &str,
    x: f64,
    y_start: f64,
    max_y: f64,
    theme: &Theme,
) -> f64 {
    let mut cursor_y = y_start;
    for line in text.split('\n') {
        if cursor_y > max_y {
            break;
        }
        p.draw_text(
            line,
            x,
            cursor_y,
            theme.on_background,
            14.0,
            tench_ui::parley::FontWeight::NORMAL,
            false,
        );
        cursor_y += 22.0;
    }
    cursor_y
}

/// Paint the search bar overlay.
pub fn paint_search_bar(
    p: &mut Painter<'_>,
    size: tench_ui::prelude::Size,
    query: &str,
    case_sensitive: bool,
    theme: &Theme,
) {
    let bar = Rect::new(
        size.width / 2.0 - 210.0,
        52.0,
        size.width / 2.0 + 210.0,
        90.0,
    );
    p.fill_rounded_rect(bar, theme.surface, theme.border_radius);
    p.stroke_rounded_rect(bar, theme.border, 1.0, theme.border_radius);

    let label = if case_sensitive {
        "Search (case-sensitive)"
    } else {
        "Search"
    };
    p.draw_text(
        label,
        bar.x0 + 12.0,
        bar.y0 + 18.0,
        theme.on_surface,
        11.0,
        tench_ui::parley::FontWeight::BOLD,
        false,
    );

    let display = if query.is_empty() {
        "Type to search..."
    } else {
        query
    };
    p.draw_text(
        display,
        bar.x0 + 12.0,
        bar.y0 + 34.0,
        if query.is_empty() {
            theme.secondary
        } else {
            theme.on_background
        },
        12.0,
        tench_ui::parley::FontWeight::NORMAL,
        false,
    );
}
