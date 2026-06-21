use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;
use tench_ui::vello::Scene;

use super::editor;
use super::state::{
    UniverseHit, UniverseMode, UniverseState, ACCENT_UNIVERSE, NEUTRAL_100, NEUTRAL_300,
    NEUTRAL_400, NEUTRAL_50, NEUTRAL_500, NEUTRAL_600, NEUTRAL_700, NEUTRAL_800, NEUTRAL_900,
};

const HEADER_H: f64 = 40.0;
const LEFT_W: f64 = 180.0;
const RIGHT_W: f64 = 260.0;
const RIGHT_W_PREVIEW: f64 = 400.0;
const COMPOSER_H: f64 = 56.0;

pub struct UniverseRegions {
    pub header: Rect,
    pub left: Rect,
    pub center: Rect,
    pub right: Rect,
    pub composer: Rect,
}

pub fn regions(size: Size, preview_open: bool) -> UniverseRegions {
    let right_w = if preview_open {
        RIGHT_W_PREVIEW
    } else {
        RIGHT_W
    };
    UniverseRegions {
        header: Rect::new(0.0, 0.0, size.width, HEADER_H),
        left: Rect::new(0.0, HEADER_H, LEFT_W, size.height),
        center: Rect::new(LEFT_W, HEADER_H, size.width - right_w, size.height),
        right: Rect::new(size.width - right_w, HEADER_H, size.width, size.height),
        composer: Rect::new(
            LEFT_W,
            size.height - COMPOSER_H,
            size.width - right_w,
            size.height,
        ),
    }
}

pub fn paint_universe(state: &UniverseState, size: Size, scene: &mut Scene) {
    let mut p = Painter::new(scene);
    let regions = regions(size, state.prompt_preview_open);

    p.fill_background(size, NEUTRAL_900);
    paint_header(&mut p, state, &regions);
    paint_left_panel(&mut p, state, &regions);
    paint_center_panel(&mut p, state, &regions);
    paint_right_panel(&mut p, state, &regions);
    editor::paint_modals(&mut p, state, size);
    paint_toast(&mut p, state, size);
}

pub fn hit_test(
    regions: &UniverseRegions,
    state: &UniverseState,
    pos: Point,
) -> Option<UniverseHit> {
    if state.show_character_editor
        || state.show_persona_editor
        || state.show_sessions
        || state.show_template_picker
        || state.show_settings
    {
        let size = size_from_regions(regions);
        if editor::modal_close_rect(size).contains(pos)
            || editor::modal_done_rect(size).contains(pos)
            || !editor::modal_rect(size).contains(pos)
        {
            return Some(UniverseHit::CloseModal);
        }
        return None;
    }

    for (index, mode) in UniverseMode::ALL.into_iter().enumerate() {
        if mode_rect(index).contains(pos) {
            return Some(UniverseHit::Mode(mode));
        }
    }
    if settings_rect(regions.header).contains(pos) {
        return Some(UniverseHit::Settings);
    }
    if search_rect(regions.left).contains(pos) {
        return Some(UniverseHit::Search);
    }
    if new_character_rect(regions.left).contains(pos) {
        return Some(UniverseHit::NewCharacter);
    }
    if template_rect(regions.left).contains(pos) {
        return Some(UniverseHit::TemplatePicker);
    }
    if history_rect(regions.left).contains(pos) {
        return Some(UniverseHit::Sessions);
    }
    if left_settings_rect(regions.left).contains(pos) {
        return Some(UniverseHit::Settings);
    }
    for index in 0..state.characters.len() {
        if character_rect(index, regions.left).contains(pos) {
            return Some(UniverseHit::Character(index));
        }
    }
    if composer_input_rect(regions.composer).contains(pos) {
        return Some(UniverseHit::ComposerInput);
    }
    if send_rect(regions.composer).contains(pos) {
        return Some(UniverseHit::Send);
    }
    if edit_persona_rect(regions.right).contains(pos) {
        return Some(UniverseHit::EditPersona);
    }
    for index in 0..3 {
        if memory_rect(index, regions.right).contains(pos) {
            return Some(UniverseHit::Memory(index));
        }
    }
    if state.mode == UniverseMode::Interactive {
        for index in 0..4 {
            if choice_rect(index, regions.center).contains(pos) {
                return Some(UniverseHit::Choice(index));
            }
        }
    }
    None
}

fn paint_header(p: &mut Painter<'_>, state: &UniverseState, regions: &UniverseRegions) {
    p.fill_rect(regions.header, NEUTRAL_900);
    p.draw_line(
        Point::new(regions.header.x0, regions.header.y1 - 1.0),
        Point::new(regions.header.x1, regions.header.y1 - 1.0),
        NEUTRAL_600,
        1.0,
    );
    p.draw_text(
        "Tench Universe",
        16.0,
        27.0,
        NEUTRAL_100,
        16.0,
        FontWeight::BOLD,
        false,
    );
    for (index, mode) in UniverseMode::ALL.into_iter().enumerate() {
        let rect = mode_rect(index);
        if index == 0 {
            p.stroke_rounded_rect(
                Rect::new(rect.x0, rect.y0, rect.x0 + rect.width() * 4.0, rect.y1),
                NEUTRAL_500,
                1.0,
                6.0,
            );
        }
        if state.mode == mode {
            p.fill_rect(rect, NEUTRAL_600);
        }
        p.draw_text(
            mode.label(),
            rect.x0 + 12.0,
            rect.y0 + 18.0,
            if state.mode == mode {
                mode.accent()
            } else {
                NEUTRAL_300
            },
            12.0,
            FontWeight::MEDIUM,
            false,
        );
    }
    p.draw_text(
        "online: local runtime",
        regions.header.x1 - 214.0,
        25.0,
        NEUTRAL_400,
        11.0,
        FontWeight::NORMAL,
        false,
    );
    let settings = settings_rect(regions.header);
    p.stroke_rounded_rect(settings, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        "⚙",
        settings.x0 + 8.0,
        settings.y0 + 20.0,
        NEUTRAL_300,
        14.0,
        FontWeight::NORMAL,
        false,
    );
}

fn paint_left_panel(p: &mut Painter<'_>, state: &UniverseState, regions: &UniverseRegions) {
    p.fill_rect(regions.left, NEUTRAL_700);
    p.draw_line(
        Point::new(regions.left.x1 - 1.0, regions.left.y0),
        Point::new(regions.left.x1 - 1.0, regions.left.y1),
        NEUTRAL_600,
        1.0,
    );
    p.fill_rounded_rect(search_rect(regions.left), NEUTRAL_600, 6.0);
    p.stroke_rounded_rect(search_rect(regions.left), NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        if state.search_query.is_empty() {
            "Search characters..."
        } else {
            &state.search_query
        },
        regions.left.x0 + 20.0,
        regions.left.y0 + 31.0,
        NEUTRAL_400,
        12.0,
        FontWeight::NORMAL,
        false,
    );
    paint_button(p, new_character_rect(regions.left), "+ New", true);
    paint_button(p, template_rect(regions.left), "Template", false);

    for (index, character) in state.characters.iter().enumerate() {
        let rect = character_rect(index, regions.left);
        if index == state.active_character_idx {
            p.fill_rect(rect, NEUTRAL_600);
            p.fill_rect(
                Rect::new(rect.x0, rect.y0, rect.x0 + 2.0, rect.y1),
                ACCENT_UNIVERSE,
            );
        }
        let avatar = Rect::new(
            rect.x0 + 12.0,
            rect.y0 + 10.0,
            rect.x0 + 40.0,
            rect.y0 + 38.0,
        );
        p.fill_rounded_rect(avatar, NEUTRAL_500, 999.0);
        p.draw_text(
            &character.name[0..1],
            avatar.x0 + 10.0,
            avatar.y0 + 19.0,
            NEUTRAL_100,
            11.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            character.name,
            rect.x0 + 50.0,
            rect.y0 + 22.0,
            NEUTRAL_100,
            13.0,
            FontWeight::MEDIUM,
            false,
        );
        p.fill_circle(
            Point::new(rect.x0 + 54.0, rect.y0 + 33.0),
            4.0,
            character.mood.color(),
        );
        p.draw_text(
            &format!("{} memories", character.memory_count),
            rect.x0 + 64.0,
            rect.y0 + 37.0,
            NEUTRAL_400,
            11.0,
            FontWeight::NORMAL,
            false,
        );
    }

    paint_button(p, history_rect(regions.left), "History", false);
    paint_button(p, left_settings_rect(regions.left), "Settings", false);
}

fn paint_center_panel(p: &mut Painter<'_>, state: &UniverseState, regions: &UniverseRegions) {
    p.fill_rect(regions.center, NEUTRAL_800);
    let stream = Rect::new(
        regions.center.x0,
        regions.center.y0,
        regions.center.x1,
        regions.composer.y0,
    );
    match state.mode {
        UniverseMode::Chat => paint_chat_messages(p, state, stream),
        UniverseMode::Novel => paint_novel(p, state, stream),
        UniverseMode::Interactive => paint_interactive(p, state, stream),
        UniverseMode::Script => paint_script(p, state, stream),
    }
    paint_composer(p, state, regions.composer);
}

fn paint_chat_messages(p: &mut Painter<'_>, state: &UniverseState, stream: Rect) {
    if state.messages.is_empty() {
        p.draw_text(
            "Select a character and type a message.",
            stream.x0 + 24.0,
            stream.y0 + 40.0,
            NEUTRAL_400,
            13.0,
            FontWeight::NORMAL,
            false,
        );
        return;
    }
    for (index, message) in state.messages.iter().enumerate() {
        let user = message.sender == "user";
        let y = stream.y0 + 16.0 + index as f64 * 78.0;
        let x = if user {
            stream.x1 - 384.0
        } else {
            stream.x0 + 24.0
        };
        let bubble = Rect::new(x, y, x + 360.0, y + 58.0);
        p.fill_rounded_rect(bubble, if user { NEUTRAL_700 } else { NEUTRAL_600 }, 12.0);
        if user {
            p.stroke_rounded_rect(bubble, ACCENT_UNIVERSE, 1.0, 12.0);
        }
        p.draw_text(
            if user { "You" } else { &message.sender },
            bubble.x0 + 12.0,
            bubble.y0 + 20.0,
            NEUTRAL_300,
            12.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            &message.text,
            bubble.x0 + 12.0,
            bubble.y0 + 42.0,
            NEUTRAL_50,
            13.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &message.time,
            bubble.x1 - 40.0,
            bubble.y1 - 8.0,
            NEUTRAL_400,
            10.0,
            FontWeight::NORMAL,
            false,
        );
    }
}

fn paint_novel(p: &mut Painter<'_>, state: &UniverseState, stream: Rect) {
    p.draw_text(
        "Chapter 2 - The Sealed Gate",
        stream.x0 + 88.0,
        stream.y0 + 44.0,
        NEUTRAL_300,
        14.0,
        FontWeight::BOLD,
        false,
    );
    for (index, message) in state.messages.iter().enumerate() {
        p.draw_text(
            &message.text,
            stream.x0 + 88.0,
            stream.y0 + 88.0 + index as f64 * 42.0,
            if message.sender == "user" {
                ACCENT_UNIVERSE
            } else {
                NEUTRAL_100
            },
            15.0,
            FontWeight::NORMAL,
            false,
        );
    }
}

fn paint_interactive(p: &mut Painter<'_>, state: &UniverseState, stream: Rect) {
    if let Some(block) = state.interactive_blocks.first() {
        p.draw_text(
            block.text,
            stream.x0 + 64.0,
            stream.y0 + 52.0,
            NEUTRAL_100,
            15.0,
            FontWeight::NORMAL,
            false,
        );
        for (index, choice) in block.choices.iter().enumerate() {
            let rect = choice_rect(index, stream);
            let selected = block.selected == Some(index);
            p.fill_rounded_rect(rect, if selected { NEUTRAL_600 } else { NEUTRAL_800 }, 6.0);
            p.stroke_rounded_rect(
                rect,
                if selected {
                    ACCENT_UNIVERSE
                } else {
                    NEUTRAL_500
                },
                1.0,
                6.0,
            );
            p.draw_text(
                choice,
                rect.x0 + 16.0,
                rect.y0 + 25.0,
                if selected {
                    ACCENT_UNIVERSE
                } else {
                    NEUTRAL_100
                },
                13.0,
                FontWeight::NORMAL,
                false,
            );
        }
    }
}

fn paint_script(p: &mut Painter<'_>, state: &UniverseState, stream: Rect) {
    p.draw_text(
        "INT. ARCHIVE GATE - NIGHT",
        stream.x0 + 48.0,
        stream.y0 + 42.0,
        NEUTRAL_300,
        12.0,
        FontWeight::BOLD,
        false,
    );
    for (index, message) in state.messages.iter().enumerate() {
        let y = stream.y0 + 80.0 + index as f64 * 46.0;
        p.draw_text(
            if message.sender == "user" {
                "USER"
            } else {
                "CHAR"
            },
            stream.x0 + 48.0,
            y,
            ACCENT_UNIVERSE,
            12.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            &message.text,
            stream.x0 + 120.0,
            y,
            NEUTRAL_100,
            13.0,
            FontWeight::NORMAL,
            false,
        );
    }
}

fn paint_composer(p: &mut Painter<'_>, state: &UniverseState, composer: Rect) {
    p.fill_rect(composer, NEUTRAL_700);
    p.draw_line(
        Point::new(composer.x0, composer.y0),
        Point::new(composer.x1, composer.y0),
        NEUTRAL_600,
        1.0,
    );
    let input = Rect::new(
        composer.x0 + 16.0,
        composer.y0 + 8.0,
        composer.x1 - 60.0,
        composer.y1 - 8.0,
    );
    p.fill_rounded_rect(input, NEUTRAL_600, 8.0);
    p.stroke_rounded_rect(input, ACCENT_UNIVERSE, 1.0, 8.0);
    p.draw_text(
        if state.input_text.is_empty() {
            "Type a message..."
        } else {
            &state.input_text
        },
        input.x0 + 12.0,
        input.y0 + 26.0,
        if state.input_text.is_empty() {
            NEUTRAL_400
        } else {
            NEUTRAL_100
        },
        13.0,
        FontWeight::NORMAL,
        false,
    );
    let send = send_rect(composer);
    p.fill_rounded_rect(send, ACCENT_UNIVERSE, 6.0);
    p.draw_text(
        "➤",
        send.x0 + 11.0,
        send.y0 + 24.0,
        NEUTRAL_900,
        16.0,
        FontWeight::BOLD,
        false,
    );
}

fn paint_right_panel(p: &mut Painter<'_>, state: &UniverseState, regions: &UniverseRegions) {
    let right = regions.right;
    p.fill_rect(right, NEUTRAL_700);
    p.draw_line(
        Point::new(right.x0, right.y0),
        Point::new(right.x0, right.y1),
        NEUTRAL_600,
        1.0,
    );
    p.draw_text(
        "Persona",
        right.x0 + 12.0,
        right.y0 + 28.0,
        NEUTRAL_400,
        11.0,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        &state.persona.name,
        right.x0 + 12.0,
        right.y0 + 58.0,
        NEUTRAL_100,
        14.0,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        &state.persona.tone,
        right.x0 + 12.0,
        right.y0 + 78.0,
        NEUTRAL_300,
        12.0,
        FontWeight::NORMAL,
        false,
    );
    paint_button(p, edit_persona_rect(right), "Edit", false);

    p.draw_text(
        "Pinned Memory",
        right.x0 + 12.0,
        right.y0 + 132.0,
        NEUTRAL_400,
        11.0,
        FontWeight::BOLD,
        false,
    );
    for (index, item) in ["gate opened", "trust: 42", "chapter: 2"]
        .iter()
        .enumerate()
    {
        let rect = Rect::new(
            right.x0 + 12.0,
            right.y0 + 148.0 + index as f64 * 34.0,
            right.x1 - 12.0,
            right.y0 + 176.0 + index as f64 * 34.0,
        );
        p.fill_rounded_rect(rect, NEUTRAL_600, 4.0);
        p.fill_rect(
            Rect::new(rect.x0, rect.y0, rect.x0 + 2.0, rect.y1),
            ACCENT_UNIVERSE,
        );
        p.draw_text(
            item,
            rect.x0 + 8.0,
            rect.y0 + 19.0,
            NEUTRAL_100,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }

    p.draw_text(
        "Persona Bio",
        right.x0 + 12.0,
        right.y0 + 270.0,
        NEUTRAL_400,
        11.0,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        &state.persona.bio,
        right.x0 + 12.0,
        right.y0 + 294.0,
        NEUTRAL_300,
        12.0,
        FontWeight::NORMAL,
        false,
    );
    if let Some(character) = state.active_character() {
        p.draw_text(
            "Active Character",
            right.x0 + 12.0,
            right.y0 + 350.0,
            NEUTRAL_400,
            11.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            character.description,
            right.x0 + 12.0,
            right.y0 + 374.0,
            NEUTRAL_100,
            12.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &format!(
                "score {} / tags {}",
                character.score,
                character.tags.join(", ")
            ),
            right.x0 + 12.0,
            right.y0 + 400.0,
            NEUTRAL_300,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }
}

fn paint_toast(p: &mut Painter<'_>, state: &UniverseState, size: Size) {
    if state.toast.is_empty() {
        return;
    }
    let rect = Rect::new(
        size.width - 280.0,
        size.height - 54.0,
        size.width - 16.0,
        size.height - 16.0,
    );
    p.fill_rounded_rect(rect, NEUTRAL_800, 6.0);
    p.stroke_rounded_rect(rect, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        &state.toast,
        rect.x0 + 12.0,
        rect.y0 + 25.0,
        NEUTRAL_100,
        12.0,
        FontWeight::NORMAL,
        false,
    );
}

fn paint_button(p: &mut Painter<'_>, rect: Rect, label: &str, primary: bool) {
    p.fill_rounded_rect(
        rect,
        if primary {
            ACCENT_UNIVERSE
        } else {
            NEUTRAL_600
        },
        6.0,
    );
    p.stroke_rounded_rect(
        rect,
        if primary {
            ACCENT_UNIVERSE
        } else {
            NEUTRAL_500
        },
        1.0,
        6.0,
    );
    p.draw_text(
        label,
        rect.x0 + 10.0,
        rect.y0 + 22.0,
        if primary { NEUTRAL_900 } else { NEUTRAL_100 },
        12.0,
        FontWeight::BOLD,
        false,
    );
}

pub(crate) fn mode_rect(index: usize) -> Rect {
    Rect::new(
        188.0 + index as f64 * 82.0,
        8.0,
        270.0 + index as f64 * 82.0,
        32.0,
    )
}

pub(crate) fn settings_rect(header: Rect) -> Rect {
    Rect::new(
        header.x1 - 42.0,
        header.y0 + 6.0,
        header.x1 - 14.0,
        header.y0 + 34.0,
    )
}

pub(crate) fn search_rect(left: Rect) -> Rect {
    Rect::new(left.x0 + 8.0, left.y0 + 10.0, left.x1 - 8.0, left.y0 + 38.0)
}

pub(crate) fn new_character_rect(left: Rect) -> Rect {
    Rect::new(
        left.x0 + 8.0,
        left.y0 + 46.0,
        left.x0 + 86.0,
        left.y0 + 76.0,
    )
}

pub(crate) fn template_rect(left: Rect) -> Rect {
    Rect::new(
        left.x0 + 94.0,
        left.y0 + 46.0,
        left.x1 - 8.0,
        left.y0 + 76.0,
    )
}

pub(crate) fn character_rect(index: usize, left: Rect) -> Rect {
    Rect::new(
        left.x0,
        left.y0 + 88.0 + index as f64 * 48.0,
        left.x1,
        left.y0 + 136.0 + index as f64 * 48.0,
    )
}

pub(crate) fn history_rect(left: Rect) -> Rect {
    Rect::new(
        left.x0 + 8.0,
        left.y1 - 40.0,
        left.x0 + 86.0,
        left.y1 - 10.0,
    )
}

pub(crate) fn left_settings_rect(left: Rect) -> Rect {
    Rect::new(
        left.x0 + 94.0,
        left.y1 - 40.0,
        left.x1 - 8.0,
        left.y1 - 10.0,
    )
}

pub(crate) fn composer_input_rect(composer: Rect) -> Rect {
    Rect::new(
        composer.x0 + 16.0,
        composer.y0 + 8.0,
        composer.x1 - 60.0,
        composer.y1 - 8.0,
    )
}

pub(crate) fn send_rect(composer: Rect) -> Rect {
    Rect::new(
        composer.x1 - 48.0,
        composer.y0 + 10.0,
        composer.x1 - 12.0,
        composer.y1 - 10.0,
    )
}

pub(crate) fn choice_rect(index: usize, center: Rect) -> Rect {
    Rect::new(
        center.x0 + 64.0,
        center.y0 + 140.0 + index as f64 * 48.0,
        center.x0 + 464.0,
        center.y0 + 180.0 + index as f64 * 48.0,
    )
}

pub(crate) fn edit_persona_rect(right: Rect) -> Rect {
    Rect::new(
        right.x1 - 62.0,
        right.y0 + 36.0,
        right.x1 - 12.0,
        right.y0 + 64.0,
    )
}

pub(crate) fn memory_rect(index: usize, right: Rect) -> Rect {
    Rect::new(
        right.x0 + 12.0,
        right.y0 + 148.0 + index as f64 * 34.0,
        right.x1 - 12.0,
        right.y0 + 176.0 + index as f64 * 34.0,
    )
}

fn size_from_regions(regions: &UniverseRegions) -> Size {
    Size::new(regions.header.x1, regions.left.y1)
}
