//! Reference notes for the OxideOS kernel integration.
//!
//! The real backend implementation lives in the OxideOS repository at:
//!   `OxideOS/kernel/src/gui/oxide_backend.rs`
//!
//! That file compiles against the OxideOS kernel's own types (Graphics,
//! mouse, keyboard) and is NOT compiled into oxide-gui-core itself.
//!
//! # Quick-start
//!
//! 1. Add to `OxideOS/kernel/Cargo.toml`:
//! ```toml
//! [dependencies]
//! oxide-gui-core = { path = "../../oxide-gui/crates/oxide-gui-core" }
//! ```
//!
//! 2. `OxideOS/kernel/src/gui/mod.rs` already declares:
//! ```
//! pub mod oxide_backend;
//! ```
//!
//! 3. Wire up in your GUI loop after `Graphics` is initialised:
//! ```rust,ignore
//! use crate::gui::oxide_backend::OxideBackend;
//! use oxide_gui_core::{Canvas, color::palette};
//!
//! let mut backend = OxideBackend::new(&graphics);
//! backend.register_callbacks();   // wires keyboard + mouse → event queue
//!
//! loop {
//!     let mut c = Canvas::new(&mut backend);
//!
//!     // Full GNOME-style widget vocabulary available:
//!     c.gnome_headerbar(0, 0, width, "OxideOS", false, palette::SURFACE2);
//!     c.action_row_toggle(0, 48, 480,
//!         "Night Mode", "Dark colour scheme",
//!         true, palette::GNOME_BLUE, false, true);
//!     c.spinner(640, 400, frame, palette::GNOME_BLUE);
//!     c.present();
//!
//!     while let Some(ev) = backend.poll_event() {
//!         match ev {
//!             Event::KeyDown(Key::Escape) => break,
//!             Event::MouseButton { x, y, pressed: true, .. } => { /* hit test */ }
//!             _ => {}
//!         }
//!     }
//! }
//! ```
//!
//! # What the bridge provides
//!
//! | OxideOS primitive | oxide-gui surface |
//! |---|---|
//! | `Graphics::fill_rect(u64,u64,u64,u64,u32)` | `Backend::fill_rect(u32,u32,u32,u32,Color)` |
//! | `Graphics::present()` | `Backend::present()` |
//! | `Graphics::get_dimensions()` | `Backend::width()` / `height()` |
//! | `keyboard::register_gui_key_callback` | key byte → `Event::KeyDown(Key::Char)` etc. |
//! | `keyboard::register_arrow_key_callback` | arrow enum → `Event::KeyDown(Key::Up)` etc. |
//! | `mouse::get_mouse_position()` | delta → `Event::MouseMove` |
//! | `mouse::is_mouse_button_pressed()` | change → `Event::MouseButton` |
//!
//! # What stays in OxideOS
//!
//! oxide-gui-core is a *widget layer*, not a full replacement for the kernel
//! GUI stack.  These OxideOS systems are unchanged:
//!
//! - `gui/graphics.rs` — framebuffer, back-buffer, background wallpapers, alpha blending
//! - `gui/window_manager.rs` — Z-order, drag, resize, minimize, maximize
//! - `gui/mouse.rs` — PS/2 cursor tracking and rendering
//! - `gui/fonts.rs` — 8×8 kernel font (oxide-gui-core brings its own 8×16 font)
//! - All kernel apps (terminal, notepad, launcher, start_menu, …)
//! - `kernel/gui/compositor.rs` — IPC-based draw command channel
