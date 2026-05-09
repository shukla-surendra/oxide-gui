//! `Backend` — the single trait that every platform must implement.
//!
//! A backend owns the framebuffer (or window) and provides three core
//! operations: draw into the buffer, flush the buffer to the screen, and
//! return the next pending input event.
//!
//! Default implementations of `draw_char` and `draw_text` are provided using
//! the embedded bitmap font in `font.rs`.  A backend can override these for
//! hardware-accelerated text.
//!
//! ## Implementing a new backend
//!
//! ```rust,ignore
//! struct MyBackend { /* framebuffer pointer, width, height */ }
//!
//! impl Backend for MyBackend {
//!     fn width(&self)  -> u32 { … }
//!     fn height(&self) -> u32 { … }
//!
//!     fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
//!         // write pixels into your framebuffer
//!     }
//!
//!     fn present(&mut self) {
//!         // flush back-buffer to screen (or no-op for direct FB)
//!     }
//!
//!     fn poll_event(&mut self) -> Option<Event> {
//!         // translate native input into oxide-gui Event
//!         None
//!     }
//! }
//! ```

use crate::color::Color;
use crate::event::Event;
use crate::font;

pub trait Backend {
    // ── Required ───────────────────────────────────────────────────────────

    /// Framebuffer width in pixels.
    fn width(&self) -> u32;

    /// Framebuffer height in pixels.
    fn height(&self) -> u32;

    /// Fill a rectangle with a solid color.
    /// Out-of-bounds coordinates should be clipped silently.
    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color);

    /// Flush the back-buffer to the physical display.
    fn present(&mut self);

    /// Return the next pending input event, or `None` if the queue is empty.
    fn poll_event(&mut self) -> Option<Event>;

    // ── Provided (override for hardware acceleration) ──────────────────────

    /// Draw a single character at `(x, y)` using the embedded bitmap font.
    fn draw_char(&mut self, x: u32, y: u32, ch: char, color: Color) {
        font::render_char(self, x, y, ch, color);
    }

    /// Draw a string at `(x, y)` advancing by `font::CHAR_W` per glyph.
    fn draw_text(&mut self, x: u32, y: u32, text: &str, color: Color) {
        let mut cx = x;
        for ch in text.chars() {
            if cx + font::CHAR_W > self.width() { break; }
            self.draw_char(cx, y, ch, color);
            cx += font::CHAR_W;
        }
    }

    /// Draw a 1-pixel horizontal rule.
    fn hline(&mut self, x: u32, y: u32, w: u32, color: Color) {
        self.fill_rect(x, y, w, 1, color);
    }

    /// Draw a 1-pixel vertical rule.
    fn vline(&mut self, x: u32, y: u32, h: u32, color: Color) {
        self.fill_rect(x, y, 1, h, color);
    }

    /// Draw a hollow rectangle (border only, 1-pixel thick).
    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        self.hline(x, y,           w, color);
        self.hline(x, y + h - 1,   w, color);
        self.vline(x,           y, h, color);
        self.vline(x + w - 1,   y, h, color);
    }

    /// Clear the entire framebuffer to `color`.
    fn clear(&mut self, color: Color) {
        let (w, h) = (self.width(), self.height());
        self.fill_rect(0, 0, w, h, color);
    }

    /// Return `(width, height)` as a tuple.
    fn size(&self) -> (u32, u32) { (self.width(), self.height()) }
}
