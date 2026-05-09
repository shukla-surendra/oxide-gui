//! `Canvas<B>` — high-level drawing helper built on top of a `Backend`.
//!
//! `Canvas` owns a mutable reference to a `Backend` and adds convenience
//! methods (rounded rects, progress bars, shadow boxes) that are composed
//! from the primitives the backend already provides.  Nothing here touches
//! the framebuffer directly — every call goes through `Backend::fill_rect`.

use crate::backend::Backend;
use crate::color::{Color, lerp_color, palette};
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

    /// Access the underlying backend directly for operations not in Canvas.
    pub fn backend_mut(&mut self) -> &mut B { self.backend }
}
