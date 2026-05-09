//! oxide-gui-core — no_std portable GUI core.
//!
//! # Architecture
//!
//! ```text
//!  Your app
//!    │
//!    ▼
//!  Canvas<B>          ← drawing helper; owns a &mut B
//!    │  uses
//!    ▼
//!  Backend (trait)    ← fill_rect / present / poll_event
//!    │  implemented by
//!    ├─ oxide-gui-linux :: MinifbBackend   (std, X11/Wayland via minifb)
//!    └─ oxide-gui-oxideos :: KernelBackend (no_std, OxideOS framebuffer)
//! ```
//!
//! `oxide-gui-core` itself has **zero dependencies** and is `no_std` compatible.
//! Enable the `alloc` feature if your environment has a heap.

#![no_std]

pub mod color;
pub mod event;
pub mod backend;
pub mod font;
pub mod canvas;

pub use color::Color;
pub use event::{Event, Key, MouseButton};
pub use backend::Backend;
pub use canvas::Canvas;
