//! oxide-gui-linux — minifb backend for oxide-gui.
//!
//! Opens an X11/Wayland window on your running Ubuntu desktop and exposes it
//! as an `oxide_gui_core::Backend`.  Use this for development and testing.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use oxide_gui_linux::MinifbBackend;
//! use oxide_gui_core::{Canvas, color::palette};
//!
//! let mut backend = MinifbBackend::new("My Window", 800, 600).unwrap();
//! let mut canvas  = Canvas::new(&mut backend);
//!
//! while backend.is_open() {
//!     canvas.clear(palette::DARK_BG);
//!     canvas.draw_text(20, 20, "Hello, oxide-gui!", palette::CYAN);
//!     canvas.present();
//!
//!     if let Some(ev) = canvas.poll_event() {
//!         if ev.is_close() { break; }
//!     }
//! }
//! ```

pub mod backend;
pub use backend::MinifbBackend;
