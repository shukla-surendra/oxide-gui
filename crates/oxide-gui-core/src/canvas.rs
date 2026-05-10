//! `Canvas<B>` — high-level drawing helper built on top of a `Backend`.
//!
//! `Canvas` owns a mutable reference to a `Backend` and adds convenience
//! methods (rounded rects, progress bars, shadow boxes) that are composed
//! from the primitives the backend already provides.  Nothing here touches
//! the framebuffer directly — every call goes through `Backend::fill_rect`.

use crate::backend::Backend;
use crate::color::{Color, lerp_color, palette, rgb};
use crate::event::Event;
use crate::font;

pub struct Canvas<'b, B: Backend> {
    backend: &'b mut B,
}

impl<'b, B: Backend> Canvas<'b, B> {
    pub fn new(backend: &'b mut B) -> Self { Self { backend } }

    // ── Delegation ────────────────────────────────────────────────────────

    pub fn width(&self)  -> u32 { self.backend.width() }
    pub fn height(&self) -> u32 { self.backend.height() }
    pub fn size(&self)   -> (u32, u32) { self.backend.size() }

    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        self.backend.fill_rect(x, y, w, h, color);
    }

    pub fn draw_text(&mut self, x: u32, y: u32, text: &str, color: Color) {
        self.backend.draw_text(x, y, text, color);
    }

    pub fn hline(&mut self, x: u32, y: u32, w: u32, color: Color) {
        self.backend.hline(x, y, w, color);
    }

    pub fn vline(&mut self, x: u32, y: u32, h: u32, color: Color) {
        self.backend.vline(x, y, h, color);
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        self.backend.draw_rect(x, y, w, h, color);
    }

    pub fn clear(&mut self, color: Color) {
        self.backend.clear(color);
    }

    pub fn present(&mut self) {
        self.backend.present();
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.backend.poll_event()
    }

    // ── Composition helpers ───────────────────────────────────────────────

    /// Draw a filled rectangle with a 1-pixel border.
    pub fn panel(&mut self, x: u32, y: u32, w: u32, h: u32, fill: Color, border: Color) {
        self.fill_rect(x, y, w, h, fill);
        self.draw_rect(x, y, w, h, border);
    }

    /// Draw a title bar: filled strip + white text.
    pub fn title_bar(&mut self, x: u32, y: u32, w: u32, h: u32, title: &str, bg: Color) {
        self.fill_rect(x, y, w, h, bg);
        let pad = 8u32;
        self.draw_text(x + pad, y + (h - font::CHAR_H) / 2, title, palette::WHITE);
    }

    /// Draw a simple button with border, fill, and centred label.
    pub fn button(
        &mut self,
        x: u32, y: u32, w: u32, h: u32,
        label: &str,
        fill: Color, border: Color, text_color: Color,
    ) {
        self.panel(x, y, w, h, fill, border);
        let lw  = font::text_width(label);
        let tx  = x + w.saturating_sub(lw) / 2;
        let ty  = y + h.saturating_sub(font::CHAR_H) / 2;
        self.draw_text(tx, ty, label, text_color);
    }

    /// Draw a horizontal progress bar.  `percent` is clamped to 0–100.
    pub fn progress_bar(
        &mut self,
        x: u32, y: u32, w: u32, h: u32,
        percent: u32,
        track: Color, fill: Color, border: Color,
    ) {
        let pct  = percent.min(100);
        let fill_w = w * pct / 100;
        self.fill_rect(x, y, w, h, track);
        if fill_w > 0 { self.fill_rect(x, y, fill_w, h, fill); }
        self.draw_rect(x, y, w, h, border);
    }

    /// Draw a vertical divider line.
    pub fn divider_v(&mut self, x: u32, y: u32, h: u32) {
        self.vline(x, y, h, palette::DIVIDER);
    }

    /// Draw a horizontal divider line.
    pub fn divider_h(&mut self, x: u32, y: u32, w: u32) {
        self.hline(x, y, w, palette::DIVIDER);
    }

    /// Draw text centered horizontally in a `w`-wide strip starting at `x`.
    pub fn centered_text(&mut self, x: u32, y: u32, w: u32, text: &str, color: Color) {
        let tw = font::text_width(text);
        let tx = x + w.saturating_sub(tw) / 2;
        self.draw_text(tx, y, text, color);
    }

    /// Draw right-aligned text ending at `x + w`.
    pub fn right_text(&mut self, x: u32, y: u32, w: u32, text: &str, color: Color) {
        let tw = font::text_width(text);
        let tx = x + w.saturating_sub(tw);
        self.draw_text(tx, y, text, color);
    }

    /// Fill a rounded-corner rectangle (corners are clipped by 1 pixel).
    pub fn fill_rounded(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        if w < 2 || h < 2 { self.fill_rect(x, y, w, h, color); return; }
        self.fill_rect(x + 1, y,         w - 2, h,     color); // main body
        self.fill_rect(x,     y + 1,     1,     h - 2, color); // left strip
        self.fill_rect(x + w - 1, y + 1, 1,     h - 2, color); // right strip
    }

    /// Fill a horizontal gradient from `left` to `right` color across `w` pixels.
    pub fn gradient_h(&mut self, x: u32, y: u32, w: u32, h: u32, left: Color, right: Color) {
        if w == 0 { return; }
        let steps = (w - 1).max(1);
        for i in 0..w {
            let c = lerp_color(left, right, i, steps);
            self.fill_rect(x + i, y, 1, h, c);
        }
    }

    /// Fill a vertical gradient from `top` to `bottom` color across `h` pixels.
    pub fn gradient_v(&mut self, x: u32, y: u32, w: u32, h: u32, top: Color, bottom: Color) {
        if h == 0 { return; }
        let steps = (h - 1).max(1);
        for i in 0..h {
            let c = lerp_color(top, bottom, i, steps);
            self.fill_rect(x, y + i, w, 1, c);
        }
    }

    /// Panel with a 4-pixel dark drop shadow behind it.
    pub fn shadow_panel(&mut self, x: u32, y: u32, w: u32, h: u32, fill: Color, border: Color) {
        self.fill_rect(x + 4, y + 4, w, h, palette::BLACK);
        self.panel(x, y, w, h, fill, border);
    }

    /// 3-pixel vertical accent bar — used for selection indicators.
    pub fn accent_bar(&mut self, x: u32, y: u32, h: u32, color: Color) {
        self.fill_rect(x, y, 3, h, color);
    }

    /// Tiny 4×4 filled status dot.
    pub fn dot(&mut self, x: u32, y: u32, color: Color) {
        self.fill_rect(x, y, 4, 4, color);
    }

    /// Colored app icon tile: rounded background + centered label.
    pub fn icon_tile(&mut self, x: u32, y: u32, size: u32, bg: Color, label: &str) {
        self.fill_rounded(x, y, size, size, bg);
        self.centered_text(x, y + size.saturating_sub(font::CHAR_H) / 2, size, label, palette::WHITE);
    }

    /// Progress bar with a two-stop horizontal gradient fill.
    pub fn gradient_progress(
        &mut self,
        x: u32, y: u32, w: u32, h: u32,
        percent: u32,
        track: Color, fill_l: Color, fill_r: Color, border: Color,
    ) {
        let pct    = percent.min(100);
        let fill_w = w * pct / 100;
        self.fill_rect(x, y, w, h, track);
        if fill_w > 0 {
            let steps = fill_w.saturating_sub(1).max(1);
            for i in 0..fill_w {
                let c = lerp_color(fill_l, fill_r, i, steps);
                self.fill_rect(x + i, y, 1, h, c);
            }
        }
        self.draw_rect(x, y, w, h, border);
    }

    // ── GNOME / libadwaita-style widgets ─────────────────────────────────

    /// Filled rectangle with 4-pixel rounded corners (AdwPreferencesGroup / card style).
    pub fn fill_round4(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        if w < 8 || h < 8 { self.fill_rect(x, y, w, h, color); return; }
        // Body between rounded rows
        self.fill_rect(x, y + 4, w, h.saturating_sub(8), color);
        // Top 4 rows (staircase approximating 4 px radius)
        self.fill_rect(x + 4, y,     w.saturating_sub(8), 1, color);
        self.fill_rect(x + 2, y + 1, w.saturating_sub(4), 1, color);
        self.fill_rect(x + 1, y + 2, w.saturating_sub(2), 2, color);
        // Bottom 4 rows
        self.fill_rect(x + 1, y + h - 4, w.saturating_sub(2), 2, color);
        self.fill_rect(x + 2, y + h - 2, w.saturating_sub(4), 1, color);
        self.fill_rect(x + 4, y + h - 1, w.saturating_sub(8), 1, color);
    }

    /// GNOME AdwToggleSwitch (52×26 px). Knob slides left=off, right=on.
    pub fn toggle_switch(&mut self, x: u32, y: u32, on: bool, accent: Color) {
        let (tw, th) = (52u32, 26u32);
        let bg = if on { accent } else { rgb(0x4A, 0x4A, 0x4A) };
        // Pill: left cap + body + right cap (each cap is th × th rounded)
        self.fill_rect(x + th / 2, y, tw.saturating_sub(th), th, bg);
        self.fill_round4(x, y, th, th, bg);
        self.fill_round4(x + tw.saturating_sub(th), y, th, th, bg);
        // White knob (22×22), 2 px inset
        let kx = if on { x + tw.saturating_sub(th) + 2 } else { x + 2 };
        self.fill_round4(kx, y + 2, th - 4, th - 4, palette::WHITE);
    }

    /// GNOME AdwActionRow — title + optional subtitle on left, text value on right.
    /// Set `last = true` on the final row of a group to skip the separator.
    /// Returns the row height (52).
    pub fn action_row(
        &mut self, x: u32, y: u32, w: u32,
        title: &str, subtitle: &str, value: &str,
        hovered: bool, last: bool,
    ) -> u32 {
        let h = 52u32;
        let bg = if hovered { rgb(0x24, 0x24, 0x3C) } else { palette::CARD_BG };
        self.fill_rect(x, y, w, h, bg);
        self.draw_text(x + 16, y + 10, title, palette::TEXT);
        if !subtitle.is_empty() {
            self.draw_text(x + 16, y + 30, subtitle, palette::TEXT_DIM);
        }
        if !value.is_empty() {
            self.right_text(x, y + 18, w.saturating_sub(16), value, palette::TEXT_DIM);
        }
        if !last {
            self.hline(x + 16, y + h - 1, w.saturating_sub(16), palette::CARD_BORDER);
        }
        h
    }

    /// GNOME AdwSwitchRow — action row with a toggle switch on the right.
    /// Returns the row height (52).
    pub fn action_row_toggle(
        &mut self, x: u32, y: u32, w: u32,
        title: &str, subtitle: &str,
        on: bool, accent: Color,
        hovered: bool, last: bool,
    ) -> u32 {
        let h = 52u32;
        let bg = if hovered { rgb(0x24, 0x24, 0x3C) } else { palette::CARD_BG };
        self.fill_rect(x, y, w, h, bg);
        self.draw_text(x + 16, y + 10, title, palette::TEXT);
        if !subtitle.is_empty() {
            self.draw_text(x + 16, y + 30, subtitle, palette::TEXT_DIM);
        }
        self.toggle_switch(x + w.saturating_sub(68), y + (h - 26) / 2, on, accent);
        if !last {
            self.hline(x + 16, y + h - 1, w.saturating_sub(16), palette::CARD_BORDER);
        }
        h
    }

    /// Navigation action row — chevron on right, no value text.
    /// Returns the row height (52).
    pub fn action_row_nav(
        &mut self, x: u32, y: u32, w: u32,
        title: &str, subtitle: &str,
        hovered: bool, last: bool,
    ) -> u32 {
        let h = 52u32;
        let bg = if hovered { rgb(0x24, 0x24, 0x3C) } else { palette::CARD_BG };
        self.fill_rect(x, y, w, h, bg);
        self.draw_text(x + 16, y + 10, title, palette::TEXT);
        if !subtitle.is_empty() {
            self.draw_text(x + 16, y + 30, subtitle, palette::TEXT_DIM);
        }
        self.draw_text(x + w.saturating_sub(24), y + 18, ">", palette::TEXT_DIM);
        if !last {
            self.hline(x + 16, y + h - 1, w.saturating_sub(16), palette::CARD_BORDER);
        }
        h
    }

    /// Pill-shaped search entry (height 36). Draws placeholder or query text.
    pub fn search_bar(
        &mut self, x: u32, y: u32, w: u32,
        text: &str, focused: bool, accent: Color,
    ) {
        let h = 36u32;
        self.fill_round4(x, y, w, h, rgb(0x1E, 0x1E, 0x30));
        let border = if focused { accent } else { palette::CARD_BORDER };
        self.draw_rect(x, y, w, h, border);
        if focused {
            self.draw_rect(x + 1, y + 1, w - 2, h - 2,
                           lerp_color(border, palette::CARD_BG, 1, 2));
        }
        // Magnifier dot (symbolic circle using the 'O' glyph)
        self.draw_text(x + 8, y + 10, "O", palette::TEXT_DIM);
        let qx = x + 28;
        if text.is_empty() {
            self.draw_text(qx, y + 10, "Search...", palette::TEXT_DIM);
        } else {
            self.draw_text(qx, y + 10, text, palette::TEXT);
        }
    }

    /// GNOME AdwHeaderBar (48 px tall) — centered title, back button, end menu button.
    pub fn gnome_headerbar(
        &mut self, x: u32, y: u32, w: u32,
        title: &str, has_back: bool, bg: Color,
    ) {
        let h = 48u32;
        self.fill_rect(x, y, w, h, bg);
        self.hline(x, y + h - 1, w, palette::CARD_BORDER);
        if has_back {
            self.fill_round4(x + 8, y + 7, 34, 34, rgb(0x30, 0x30, 0x4A));
            self.draw_text(x + 18, y + 16, "<", palette::TEXT);
        }
        self.centered_text(x, y + (h - 16) / 2, w, title, palette::TEXT);
        // Three-dot menu button at end
        let mx = x + w.saturating_sub(50);
        self.fill_round4(mx, y + 7, 34, 34, rgb(0x30, 0x30, 0x4A));
        self.fill_rect(mx +  7, y + 21, 4, 4, palette::TEXT_DIM);
        self.fill_rect(mx + 15, y + 21, 4, 4, palette::TEXT_DIM);
        self.fill_rect(mx + 23, y + 21, 4, 4, palette::TEXT_DIM);
    }

    /// 8-dot animated spinner. `cx`/`cy` is the center. Frame drives rotation.
    pub fn spinner(&mut self, cx: u32, cy: u32, frame: u32, color: Color) {
        const DOTS: &[(i32, i32)] = &[
            (0, -10), (7, -7), (10, 0), (7, 7),
            (0, 10),  (-7, 7), (-10, 0), (-7, -7),
        ];
        let head = (frame / 4) as usize % 8;
        for (i, &(dx, dy)) in DOTS.iter().enumerate() {
            let age = (i + 8 - head) % 8;
            let c   = lerp_color(color, palette::SURFACE, age as u32, 7);
            let px  = cx as i32 + dx - 1;
            let py  = cy as i32 + dy - 1;
            if px >= 0 && py >= 0 {
                self.fill_rect(px as u32, py as u32, 3, 3, c);
            }
        }
    }

    /// Pill-shaped in-app notification toast (44 px tall).
    pub fn toast(&mut self, x: u32, y: u32, w: u32, message: &str, accent: Color) {
        let h = 44u32;
        // Drop shadow
        self.fill_rect(x + 4, y + 4, w, h, rgb(0, 0, 0));
        // Background
        self.fill_round4(x, y, w, h, rgb(0x2E, 0x2E, 0x46));
        self.draw_rect(x, y, w, h, rgb(0x44, 0x44, 0x66));
        // Colored accent square
        self.fill_rect(x + 12, y + (h - 8) / 2, 8, 8, accent);
        // Message
        self.draw_text(x + 28, y + (h - 16) / 2, message, palette::TEXT);
        // Dismiss 'x'
        self.draw_text(x + w.saturating_sub(22), y + (h - 16) / 2, "x", palette::TEXT_DIM);
    }

    /// Circular avatar tile with initials (uses fill_round4 as circle approximation).
    pub fn avatar(&mut self, x: u32, y: u32, size: u32, initials: &str, color: Color) {
        self.fill_round4(x, y, size, size, color);
        let tw = font::text_width(initials);
        let tx = x + size.saturating_sub(tw) / 2;
        let ty = y + size.saturating_sub(font::CHAR_H) / 2;
        self.draw_text(tx, ty, initials, palette::WHITE);
    }

    /// Small pill-shaped chip / badge label.
    pub fn chip(&mut self, x: u32, y: u32, text: &str, bg: Color, fg: Color) {
        let tw = font::text_width(text);
        let w  = tw + 16;
        let h  = 22u32;
        self.fill_round4(x, y, w, h, bg);
        self.draw_text(x + 8, y + 3, text, fg);
    }

    /// Access the underlying backend directly for operations not in Canvas.
    pub fn backend_mut(&mut self) -> &mut B { self.backend }
}
