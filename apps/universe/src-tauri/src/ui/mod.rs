//! Tench Universe native UI.
//!
//! Recreates the React `UniverseExperience` structure with a header mode
//! control, character/search panel, mode-specific center renderer, composer,
//! right state panel, editor/settings modals, and toast state.

pub mod chat;
pub mod editor;
pub mod state;

use tench_ui::prelude::*;
use tench_ui::{UiAutomationNode, UiAutomationRect};

use state::{UniverseHit, UniverseState};

/// Universe app — character chat, novel, interactive story, script modes.
pub struct UniverseApp {
    state: UniverseState,
    size: Size,
}

impl UniverseApp {
    pub fn new() -> Self {
        Self {
            state: UniverseState::default(),
            size: Size::ZERO,
        }
    }
}

impl Default for UniverseApp {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for UniverseApp {
    fn measure(&mut self, _ctx: &mut MeasureCtx<'_>, _axis: Axis, available: f64) -> f64 {
        available
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx<'_>, size: Size) {
        self.size = size;
    }

    fn paint(&mut self, ctx: &mut PaintCtx<'_>, scene: &mut Scene) {
        chat::paint_universe(&self.state, ctx.size(), scene);
    }

    fn on_pointer_event(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let PointerEvent::Down(event) = event else {
            return;
        };

        let regions = chat::regions(self.size, self.state.prompt_preview_open);
        match chat::hit_test(&regions, &self.state, event.pos) {
            Some(UniverseHit::Mode(mode)) => self.state.set_mode(mode),
            Some(UniverseHit::Search) => self.state.focus_search(),
            Some(UniverseHit::Character(index)) => self.state.select_character(index),
            Some(UniverseHit::Choice(index)) => self.state.choose_interactive(index),
            Some(UniverseHit::ComposerInput) => self.state.focus_composer(),
            Some(UniverseHit::Send) => self.state.send_input(),
            Some(UniverseHit::NewCharacter) => self.state.open_character_editor(),
            Some(UniverseHit::TemplatePicker) => self.state.open_template_picker(),
            Some(UniverseHit::Sessions) => self.state.open_sessions(),
            Some(UniverseHit::Settings) => self.state.open_settings(),
            Some(UniverseHit::EditPersona) => self.state.open_persona_editor(),
            Some(UniverseHit::Memory(index)) => self.state.select_memory(index),
            Some(UniverseHit::CloseModal) => self.state.close_modals(),
            None => {}
        }
        ctx.request_paint();
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx<'_>, event: &TextEvent) {
        let TextEvent::Keyboard(event) = event else {
            return;
        };
        if !event.is_pressed {
            return;
        }

        match &event.logical_key {
            LogicalKey::Named(NamedKey::Escape) => self.state.close_modals(),
            LogicalKey::Named(NamedKey::Enter) => self.state.send_input(),
            LogicalKey::Named(NamedKey::Backspace) => self.state.backspace_input(),
            LogicalKey::Character(ch) => self.state.push_input_text(ch),
            _ => return,
        }
        ctx.request_paint();
    }

    fn debug_id(&self) -> Option<&str> {
        Some("universe.root")
    }

    fn automation_children(&self, state: &WidgetState) -> Vec<UiAutomationNode> {
        universe_automation_nodes(&self.state, state.size, state.id.to_raw())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn universe_automation_nodes(
    universe: &UniverseState,
    size: Size,
    base_id: u64,
) -> Vec<UiAutomationNode> {
    let regions = chat::regions(size, universe.prompt_preview_open);
    let mut nodes = Vec::new();
    let mut next_id = base_id.saturating_mul(1000).saturating_add(1);

    for (idx, mode) in state::UniverseMode::ALL.into_iter().enumerate() {
        push_node(
            &mut nodes,
            &mut next_id,
            "tab",
            mode.label(),
            format!("universe.mode.{}", mode.label().to_lowercase()),
            chat::mode_rect(idx),
        );
    }
    push_node(
        &mut nodes,
        &mut next_id,
        "status",
        "local runtime",
        "universe.local_runtime_status",
        Rect::new(size.width - 220.0, 8.0, size.width - 52.0, 32.0),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "Settings",
        "universe.header.settings",
        chat::settings_rect(regions.header),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "textbox",
        "Search characters",
        "universe.character.search",
        chat::search_rect(regions.left),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "New character",
        "universe.character.new",
        chat::new_character_rect(regions.left),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "Template picker",
        "universe.template_picker",
        chat::template_rect(regions.left),
    );
    for (idx, character) in universe.characters.iter().enumerate() {
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            character.name,
            format!("universe.character.{idx}"),
            chat::character_rect(idx, regions.left),
        );
        if idx == universe.active_character_idx {
            push_node(
                &mut nodes,
                &mut next_id,
                "status",
                character.name,
                "universe.character.selected",
                chat::character_rect(idx, regions.left),
            );
        }
    }
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "History",
        "universe.history",
        chat::history_rect(regions.left),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "Left settings",
        "universe.left.settings",
        chat::left_settings_rect(regions.left),
    );

    push_node(
        &mut nodes,
        &mut next_id,
        "region",
        "Active mode content",
        "universe.active_content",
        Rect::new(
            regions.center.x0,
            regions.center.y0,
            regions.center.x1,
            regions.composer.y0,
        ),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "textbox",
        "Composer",
        "universe.composer.input",
        chat::composer_input_rect(regions.composer),
    );
    if universe.input_text.is_empty() {
        push_node(
            &mut nodes,
            &mut next_id,
            "text",
            "Type a message",
            "universe.composer.placeholder",
            chat::composer_input_rect(regions.composer),
        );
    }
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "Send",
        "universe.composer.send",
        chat::send_rect(regions.composer),
    );
    for (idx, _message) in universe.messages.iter().enumerate() {
        push_node(
            &mut nodes,
            &mut next_id,
            "text",
            "Message",
            format!("universe.chat.message.{idx}"),
            Rect::new(
                regions.center.x0 + 24.0,
                regions.center.y0 + 16.0 + idx as f64 * 78.0,
                regions.center.x0 + 384.0,
                regions.center.y0 + 74.0 + idx as f64 * 78.0,
            ),
        );
    }
    if universe.mode == state::UniverseMode::Interactive {
        for (idx, label) in ["open_gate", "call_ari_over", "walk_away", "touch_seal"]
            .into_iter()
            .enumerate()
        {
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                label,
                format!("universe.choice.{label}"),
                chat::choice_rect(idx, regions.center),
            );
        }
        if let Some(selected) = universe
            .interactive_blocks
            .first()
            .and_then(|block| block.selected)
        {
            push_node(
                &mut nodes,
                &mut next_id,
                "status",
                "Selected choice",
                "universe.choice.selected",
                chat::choice_rect(selected, regions.center),
            );
        }
    }

    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "Edit persona",
        "universe.persona.edit",
        chat::edit_persona_rect(regions.right),
    );
    for idx in 0..3 {
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "Pinned memory",
            format!("universe.memory.{idx}"),
            Rect::new(
                regions.right.x0 + 12.0,
                regions.right.y0 + 148.0 + idx as f64 * 34.0,
                regions.right.x1 - 12.0,
                regions.right.y0 + 176.0 + idx as f64 * 34.0,
            ),
        );
    }
    push_node(
        &mut nodes,
        &mut next_id,
        "region",
        "Right persona detail",
        "universe.right.persona_detail",
        Rect::new(
            regions.right.x0,
            regions.right.y0,
            regions.right.x1,
            regions.right.y1,
        ),
    );
    if universe.prompt_preview_open {
        push_node(
            &mut nodes,
            &mut next_id,
            "region",
            "Prompt preview",
            "universe.prompt_preview",
            regions.right,
        );
    }
    if !universe.toast.is_empty() {
        push_node(
            &mut nodes,
            &mut next_id,
            "status",
            "Toast",
            "universe.toast",
            Rect::new(
                size.width - 280.0,
                size.height - 54.0,
                size.width - 16.0,
                size.height - 16.0,
            ),
        );
    }

    if universe.show_character_editor
        || universe.show_persona_editor
        || universe.show_sessions
        || universe.show_template_picker
        || universe.show_settings
    {
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "Modal backdrop",
            "universe.modal.backdrop",
            Rect::new(0.0, 0.0, editor::modal_rect(size).x0, size.height),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "Modal",
            "universe.modal.surface",
            editor::modal_rect(size),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "Close",
            "universe.modal.close",
            editor::modal_close_rect(size),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "Done",
            "universe.modal.done",
            editor::modal_done_rect(size),
        );
    }

    nodes
}

fn push_node(
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
