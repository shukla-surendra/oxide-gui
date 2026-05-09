//! This module is NOT compiled as part of oxide-gui-core itself.
//! It is a reference showing how the OxideOS kernel implements the
//! Backend trait against its framebuffer.
//!
//! Copy this into your OxideOS kernel and add oxide-gui-core as a
//! dependency:
//!
//! ```toml
//! # kernel/Cargo.toml
//! [dependencies]
//! oxide-gui-core = { git = "https://github.com/your-user/oxide-gui" }
//! ```
//!
//! Then write a thin wrapper:
//!
//! ```rust,ignore
//! use oxide_gui_core::{Backend, Color, Event};
//!
//! pub struct OxideBackend<'a> {
//!     graphics: &'a crate::gui::graphics::Graphics,
//!     font: &'a crate::gui::fonts::BitmapFont,
//! }
//!
//! impl<'a> Backend for OxideBackend<'a> {
//!     fn width(&self)  -> u32 { self.graphics.width()  }
//!     fn height(&self) -> u32 { self.graphics.height() }
//!
//!     fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
//!         self.graphics.fill_rect(x, y, w, h, color);
//!     }
//!
//!     fn present(&mut self) {
//!         self.graphics.present();
//!     }
//!
//!     fn poll_event(&mut self) -> Option<Event> {
//!         // translate OxideOS keyboard/mouse events into oxide_gui_core::Event
//!         use crate::gui::mouse;
//!         use crate::kernel::keyboard;
//!         if let Some(ch) = keyboard::dequeue_char() {
//!             return Some(Event::KeyDown(oxide_gui_core::Key::Char(ch as char)));
//!         }
//!         if let Some((x, y)) = mouse::get_position() {
//!             return Some(Event::MouseMove { x: x as i32, y: y as i32 });
//!         }
//!         None
//!     }
//! }
//! ```
