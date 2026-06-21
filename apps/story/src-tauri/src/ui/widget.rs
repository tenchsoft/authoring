use super::state::StoryInputFocus;
use super::*;
use tench_ui::prelude::*;
use tench_ui::render::Painter;
use tench_ui::{parley, vello, UiAutomationNode};

impl Widget for StoryApp {
    fn measure(&mut self, _ctx: &mut MeasureCtx<'_>, _axis: Axis, _available: f64) -> f64 {
        f64::INFINITY
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx<'_>, size: Size) {
        self.width = size.width;
    }

    fn paint(&mut self, ctx: &mut PaintCtx<'_>, scene: &mut vello::Scene) {
        let size = ctx.size();
        let theme = ctx.theme().clone();
        let mut p = Painter::new(scene);

        p.fill_background(size, theme.background);

        // ── Header ──────────────────────────────────────────────────────
        let header_rect = Rect::new(0.0, 0.0, size.width, 48.0);
        p.fill_rect(header_rect, theme.surface);

        p.draw_text(
            &format!(
                "{}{}",
                self.state.project_name(),
                if self.state.is_dirty() { " *" } else { "" }
            ),
            16.0,
            32.0,
            theme.on_background,
            18.0,
            parley::FontWeight::EXTRA_BOLD,
            false,
        );

        let actions = ["New", "Open", "Save", "Export", "Focus", "Cmd"];
        let mut ax = 230.0;
        for action in actions {
            let rect = Rect::new(ax, 10.0, ax + 58.0, 38.0);
            let active = (action == "Focus" && self.state.focus_mode)
                || (action == "Cmd" && self.state.show_command_palette);
            p.fill_rounded_rect(
                rect,
                if active {
                    theme.primary
                } else {
                    theme.background
                },
                6.0,
            );
            p.stroke_rounded_rect(rect, theme.border, 1.0, 6.0);
            p.draw_text(
                action,
                ax + 29.0,
                28.0,
                if active {
                    theme.on_primary
                } else {
                    theme.on_surface
                },
                11.0,
                parley::FontWeight::BOLD,
                true,
            );
            ax += 64.0;
        }

        // Word count badge
        p.fill_rounded_rect(
            Rect::new(size.width - 180.0, 10.0, size.width - 16.0, 38.0),
            theme.border,
            8.0,
        );
        p.draw_text(
            &export::word_count_label(self.state.total_word_count()),
            size.width - 168.0,
            30.0,
            theme.on_surface,
            13.0,
            parley::FontWeight::NORMAL,
            false,
        );

        // ── Panel geometry ─────────────────────────────────────────────
        let panel_y = 48.0;
        let status_h = 24.0;
        let panel_h = size.height - panel_y - status_h;
        let left_w = if self.state.focus_mode { 0.0 } else { 220.0 };
        let right_w = if self.state.focus_mode { 0.0 } else { 300.0 };
        let center_w = size.width - left_w - right_w;

        // ── Left panel — chapter tree ──────────────────────────────────
        if !self.state.focus_mode {
            let left_rect = Rect::new(0.0, panel_y, left_w, panel_y + panel_h);
            p.fill_rect(left_rect, theme.surface);

            p.draw_text(
                "Chapters",
                16.0,
                panel_y + 24.0,
                theme.on_background,
                14.0,
                parley::FontWeight::BOLD,
                false,
            );

            let titles = chapter_tree::chapter_titles(&self.state);
            for (i, ch_title) in titles.iter().enumerate() {
                let y = panel_y + 44.0 + (i as f64) * 36.0;
                let is_selected = i == self.state.selected_chapter_idx;
                let bg = if is_selected {
                    theme.primary
                } else {
                    theme.surface
                };
                let fg = if is_selected {
                    theme.on_primary
                } else {
                    theme.on_background
                };
                p.fill_rounded_rect(Rect::new(8.0, y, left_w - 8.0, y + 30.0), bg, 6.0);
                p.draw_text(
                    ch_title,
                    16.0,
                    y + 20.0,
                    fg,
                    12.0,
                    parley::FontWeight::NORMAL,
                    false,
                );
            }
        }

        // ── Center — manuscript editor ─────────────────────────────────
        let center_rect = Rect::new(left_w, panel_y, left_w + center_w, panel_y + panel_h);
        p.fill_rect(center_rect, theme.background);

        // Editor card
        p.fill_rounded_rect(
            Rect::new(
                left_w + 16.0,
                panel_y + 16.0,
                left_w + center_w - 16.0,
                panel_y + panel_h - 16.0,
            ),
            theme.surface,
            8.0,
        );

        // Chapter title
        p.draw_text(
            &self.state.chapter_title(),
            left_w + 28.0,
            panel_y + 40.0,
            theme.on_background,
            16.0,
            parley::FontWeight::BOLD,
            false,
        );

        // Chapter content
        let text = self.state.chapter_text();
        let cursor_y = editor::paint_chapter_content(
            &mut p,
            &text,
            left_w + 28.0,
            panel_y + 68.0,
            panel_y + panel_h - 40.0,
            &theme,
        );

        // Cursor indicator
        if cursor_y < panel_y + panel_h - 40.0 {
            editor::paint_cursor(&mut p, left_w + 28.0, cursor_y, theme.primary);
        }

        // ── Right panel — auxiliary ────────────────────────────────────
        let right_rect = Rect::new(left_w + center_w, panel_y, size.width, panel_y + panel_h);
        if !self.state.focus_mode {
            panels::paint_aux_panel_background(&mut p, &theme, right_rect);

            // Tab bar
            for (i, (label, tab)) in commands::RIGHT_PANEL_TABS.iter().enumerate() {
                let x = left_w + center_w + 8.0 + (i as f64) * 32.0;
                let is_active = self.state.active_tab == *tab;
                let fg = if is_active {
                    theme.primary
                } else {
                    theme.on_surface
                };
                p.draw_text(
                    label,
                    x,
                    panel_y + 20.0,
                    fg,
                    9.0,
                    parley::FontWeight::NORMAL,
                    false,
                );
            }

            // Tab content
            let content_y = panel_y + 40.0;
            let panel_x = left_w + center_w + 16.0;
            panels::paint_tab_content(&mut p, &self.state, panel_x, content_y, size.width, &theme);
        }

        // ── Status bar ─────────────────────────────────────────────────
        let status_rect = Rect::new(0.0, size.height - status_h, size.width, size.height);
        p.fill_rect(status_rect, theme.surface);
        p.draw_text(
            &format!(
                "autosaved {}    {} words this chapter    Ch {:02}    {} total{}",
                self.state.saved_at,
                self.state.chapter_word_count(),
                self.state.selected_chapter_idx + 1,
                self.state.total_word_count(),
                if self.state.focus_mode {
                    "    focus mode"
                } else {
                    ""
                }
            ),
            16.0,
            size.height - 8.0,
            theme.secondary,
            11.0,
            parley::FontWeight::NORMAL,
            false,
        );

        // ── Overlays ───────────────────────────────────────────────────
        if self.state.show_export {
            overlays::paint_export_modal(&mut p, size, &self.state, &theme);
        }
        if self.state.show_command_palette {
            overlays::paint_command_palette(&mut p, size, &theme);
        }
        if self.state.show_search {
            editor::paint_search_bar(
                &mut p,
                size,
                &self.state.search_query,
                self.state.search_case_sensitive,
                &theme,
            );
        }
    }

    fn on_pointer_event(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let PointerEvent::Down(e) = event {
            let x = e.pos.x;
            let y = e.pos.y;
            let size = ctx.state.size;

            if self.state.show_export {
                if let Some(index) = geometry::hit_test_export_format(e.pos, size) {
                    self.state.saved_at = format!("exported {}", commands::export_formats()[index]);
                    self.state.show_export = false;
                    ctx.request_paint();
                    return;
                }
                if !geometry::export_modal_rect(size).contains(e.pos) {
                    self.state.close_overlays();
                    ctx.request_paint();
                    return;
                }
            }

            if self.state.show_command_palette {
                if let Some(index) = geometry::hit_test_command_row(e.pos, size) {
                    self.dispatch_command_palette(index);
                    ctx.request_paint();
                    return;
                }
                if !geometry::command_palette_rect(size).contains(e.pos) {
                    self.state.close_overlays();
                    ctx.request_paint();
                    return;
                }
            }

            if self.state.show_search {
                if geometry::search_case_rect(size).contains(e.pos) {
                    self.state.toggle_search_case_sensitive();
                    ctx.request_paint();
                    return;
                }
                if geometry::search_bar_rect(size).contains(e.pos) {
                    self.state.input_focus = StoryInputFocus::Search;
                    ctx.request_paint();
                    return;
                }
            }

            // Header buttons
            if y < 48.0 {
                let mut ax = 230.0;
                for action in ["New", "Open", "Save", "Export", "Focus", "Cmd"] {
                    if Rect::new(ax, 10.0, ax + 58.0, 38.0).contains(e.pos) {
                        match action {
                            "New" => {
                                self.state = StoryState::default();
                            }
                            "Open" => {
                                self.state.open_project();
                            }
                            "Save" => self.state.save(),
                            "Export" => self.state.open_export(),
                            "Focus" => self.state.toggle_focus_mode(),
                            "Cmd" => self.state.toggle_command_palette(),
                            _ => {}
                        }
                        ctx.request_paint();
                        return;
                    }
                    ax += 64.0;
                }
            }

            // Chapter tree
            if !self.state.focus_mode {
                if let Some(idx) = chapter_tree::hit_test(e.pos, self.state.chapter_count()) {
                    self.state.select_chapter(idx);
                    ctx.request_paint();
                    return;
                }
            }

            // Tab selection
            let left_w = if self.state.focus_mode { 0.0 } else { 220.0 };
            let right_w = if self.state.focus_mode { 0.0 } else { 300.0 };
            let center_w = self.width - left_w - right_w;
            if !self.state.focus_mode {
                if let Some(tab) = commands::hit_test_tab(x, y, left_w, center_w) {
                    self.state.select_tab(tab);
                    ctx.request_paint();
                    return;
                }
                if let Some(label) =
                    geometry::hit_test_right_panel_row(e.pos, &self.state, self.width)
                {
                    self.state.saved_at = format!("selected {label}");
                    ctx.request_paint();
                }
            }
        }
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx<'_>, event: &TextEvent) {
        if let TextEvent::Keyboard(e) = event {
            if e.is_pressed && !e.is_repeat {
                match &e.logical_key {
                    LogicalKey::Named(NamedKey::Escape) => {
                        self.state.close_overlays();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(ch) if ch == "p" && e.modifiers.control => {
                        self.state.toggle_command_palette();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(ch) if ch == "s" && e.modifiers.control => {
                        self.state.save();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(ch) if ch == "e" && e.modifiers.control => {
                        self.state.open_export();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(ch) if ch == "f" && e.modifiers.control => {
                        self.state.toggle_search();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(ch) if ch == "z" && e.modifiers.control => {
                        if e.modifiers.shift {
                            self.state.engine.redo();
                        } else {
                            self.state.engine.undo();
                        }
                        ctx.request_paint();
                    }
                    LogicalKey::Character(ch) if ch == "y" && e.modifiers.control => {
                        self.state.engine.redo();
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::Enter) => {
                        if self.state.input_focus == StoryInputFocus::Search {
                            self.state.input_focus = StoryInputFocus::Manuscript;
                        } else {
                            self.state.newline();
                        }
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::Backspace) => {
                        if self.state.input_focus == StoryInputFocus::Search {
                            self.state.backspace_search();
                        } else {
                            self.state.backspace();
                        }
                        ctx.request_paint();
                    }
                    LogicalKey::Character(ch) => {
                        if self.state.input_focus == StoryInputFocus::Search {
                            self.state.append_search_text(ch);
                        } else {
                            self.state.append_text(ch);
                        }
                        ctx.request_paint();
                    }
                    _ => {}
                }
            }
        }
    }

    fn debug_id(&self) -> Option<&str> {
        Some("story.root")
    }

    fn automation_children(&self, state: &WidgetState) -> Vec<UiAutomationNode> {
        automation::story_automation_nodes(&self.state, state.size, state.id.to_raw())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
