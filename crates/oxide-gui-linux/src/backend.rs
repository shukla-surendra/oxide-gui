//! `MinifbBackend` — Linux backend using the `minifb` crate.
//!
//! minifb opens a native X11/Wayland/Win32 window and gives us a `Vec<u32>`
//! pixel buffer to draw into.  We translate minifb's key and mouse events into
//! `oxide_gui_core::Event` so application code stays platform-agnostic.

use minifb::{Window, WindowOptions, Key as MKey, MouseMode, MouseButton as MBtn};
use oxide_gui_core::{Backend, Color, Event, Key, MouseButton};
use oxide_gui_core::color::to_rgb24;

pub struct MinifbBackend {
    window: Window,
    buffer: Vec<u32>,
    width:  usize,
    height: usize,
}

impl MinifbBackend {
    /// Create a window of the given size.
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, minifb::Error> {
        let opts = WindowOptions {
            resize:    true,
            scale:     minifb::Scale::X1,
            ..Default::default()
        };
        let window = Window::new(title, width as usize, height as usize, opts)?;
        let buffer = vec![0u32; (width * height) as usize];
        Ok(Self { window, buffer, width: width as usize, height: height as usize })
    }

    /// Returns `true` while the window is open and the user hasn't pressed Escape.
    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(MKey::Escape)
    }

    /// Resize the internal buffer to match the window if it was resized.
    fn sync_size(&mut self) {
        let (w, h) = self.window.get_size();
        if w != self.width || h != self.height {
            self.width  = w;
            self.height = h;
            self.buffer.resize(w * h, 0);
        }
    }
}

impl Backend for MinifbBackend {
    fn width(&self)  -> u32 { self.width  as u32 }
    fn height(&self) -> u32 { self.height as u32 }

    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        // minifb buffer uses 0x00RRGGBB; we strip alpha from our ARGB color.
        let rgb = to_rgb24(color);
        let (fw, fh) = (self.width as u32, self.height as u32);
        let x2 = (x + w).min(fw);
        let y2 = (y + h).min(fh);
        if x >= fw || y >= fh || x2 == 0 || y2 == 0 { return; }
        for row in y..y2 {
            let base = row as usize * self.width + x as usize;
            let end  = base + (x2 - x) as usize;
            if end <= self.buffer.len() {
                self.buffer[base..end].fill(rgb);
            }
        }
    }

    fn present(&mut self) {
        self.sync_size();
        let _ = self.window.update_with_buffer(&self.buffer, self.width, self.height);
    }

    fn poll_event(&mut self) -> Option<Event> {
        // Window close
        if !self.window.is_open() { return Some(Event::Close); }

        // Resize
        let (w, h) = self.window.get_size();
        if w != self.width || h != self.height {
            return Some(Event::Resize { width: w as u32, height: h as u32 });
        }

        // Keys pressed this frame
        for k in self.window.get_keys_pressed(minifb::KeyRepeat::Yes) {
            if let Some(ev) = translate_key(k) {
                return Some(Event::KeyDown(ev));
            }
        }

        // Mouse position
        if let Some((mx, my)) = self.window.get_mouse_pos(MouseMode::Clamp) {
            // Mouse buttons
            for (mbtn, ogbtn) in [
                (MBtn::Left,   MouseButton::Left),
                (MBtn::Right,  MouseButton::Right),
                (MBtn::Middle, MouseButton::Middle),
            ] {
                if self.window.get_mouse_down(mbtn) {
                    return Some(Event::MouseButton {
                        x: mx as i32, y: my as i32,
                        button: ogbtn, pressed: true,
                    });
                }
            }
            // Mouse move (always emit so hover works)
            return Some(Event::MouseMove { x: mx as i32, y: my as i32 });
        }

        None
    }
}

// ── Key translation ───────────────────────────────────────────────────────────

fn translate_key(k: MKey) -> Option<Key> {
    Some(match k {
        MKey::A => Key::Char('a'), MKey::B => Key::Char('b'),
        MKey::C => Key::Char('c'), MKey::D => Key::Char('d'),
        MKey::E => Key::Char('e'), MKey::F => Key::Char('f'),
        MKey::G => Key::Char('g'), MKey::H => Key::Char('h'),
        MKey::I => Key::Char('i'), MKey::J => Key::Char('j'),
        MKey::K => Key::Char('k'), MKey::L => Key::Char('l'),
        MKey::M => Key::Char('m'), MKey::N => Key::Char('n'),
        MKey::O => Key::Char('o'), MKey::P => Key::Char('p'),
        MKey::Q => Key::Char('q'), MKey::R => Key::Char('r'),
        MKey::S => Key::Char('s'), MKey::T => Key::Char('t'),
        MKey::U => Key::Char('u'), MKey::V => Key::Char('v'),
        MKey::W => Key::Char('w'), MKey::X => Key::Char('x'),
        MKey::Y => Key::Char('y'), MKey::Z => Key::Char('z'),
        MKey::Key0 => Key::Char('0'), MKey::Key1 => Key::Char('1'),
        MKey::Key2 => Key::Char('2'), MKey::Key3 => Key::Char('3'),
        MKey::Key4 => Key::Char('4'), MKey::Key5 => Key::Char('5'),
        MKey::Key6 => Key::Char('6'), MKey::Key7 => Key::Char('7'),
        MKey::Key8 => Key::Char('8'), MKey::Key9 => Key::Char('9'),
        MKey::Space     => Key::Space,
        MKey::Enter     => Key::Enter,
        MKey::Backspace => Key::Backspace,
        MKey::Delete    => Key::Delete,
        MKey::Escape    => Key::Escape,
        MKey::Tab       => Key::Tab,
        MKey::Up        => Key::Up,
        MKey::Down      => Key::Down,
        MKey::Left      => Key::Left,
        MKey::Right     => Key::Right,
        MKey::Home      => Key::Home,
        MKey::End       => Key::End,
        MKey::PageUp    => Key::PageUp,
        MKey::PageDown  => Key::PageDown,
        MKey::F1  => Key::F(1),  MKey::F2  => Key::F(2),
        MKey::F3  => Key::F(3),  MKey::F4  => Key::F(4),
        MKey::F5  => Key::F(5),  MKey::F6  => Key::F(6),
        MKey::F7  => Key::F(7),  MKey::F8  => Key::F(8),
        MKey::F9  => Key::F(9),  MKey::F10 => Key::F(10),
        MKey::F11 => Key::F(11), MKey::F12 => Key::F(12),
        MKey::LeftShift  => Key::LeftShift,
        MKey::RightShift => Key::RightShift,
        MKey::LeftCtrl   => Key::LeftCtrl,
        MKey::RightCtrl  => Key::RightCtrl,
        MKey::LeftAlt    => Key::LeftAlt,
        _ => return None,
    })
}
