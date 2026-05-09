//! `Canvas<B>` — high-level drawing helper built on top of a `Backend`.
//!
//! `Canvas` owns a mutable reference to a `Backend` and adds convenience
//! methods (rounded rects, progress bars, shadow boxes) that are composed
//! from the primitives the backend already provides.  Nothing here touches
//! the framebuffer directly — every call goes through `Backend::fill_rect`.

use crate::backend::Backend;
use crate::color::{Color, palette};
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

    /// Access the underlying backend directly for operations not in Canvas.
    pub fn backend_mut(&mut self) -> &mut B { self.backend }
}
