//! Input event types.
//!
//! Both the Linux and OxideOS backends translate their native input into these
//! common types so application code never sees platform-specific key codes.

/// A keyboard key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    // Printable ASCII
    Char(char),

    // Control keys
    Enter,
    Backspace,
    Delete,
    Escape,
    Tab,
    Space,

    // Navigation
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,

    // Modifiers (sent as independent events)
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,

    // Function keys
    F(u8),

    /// A key the backend doesn't map to any of the above variants.
    Unknown,
}

/// A mouse button identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// A GUI input event.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// A key was pressed.
    KeyDown(Key),
    /// A key was released.
    KeyUp(Key),

    /// The pointer moved to `(x, y)` in window-content coordinates.
    MouseMove { x: i32, y: i32 },

    /// A mouse button was pressed or released.
    MouseButton {
        x:       i32,
        y:       i32,
        button:  MouseButton,
        pressed: bool,
    },

    /// The mouse wheel scrolled.  `delta` is positive = up, negative = down.
    Scroll { delta: i32 },

    /// The window was asked to close (e.g., the user clicked ✕).
    Close,

    /// The window was resized.
    Resize { width: u32, height: u32 },
}

impl Event {
    pub fn is_close(&self) -> bool { matches!(self, Event::Close) }

    /// Returns the character if this is `KeyDown(Char(_))`.
    pub fn as_char(&self) -> Option<char> {
        if let Event::KeyDown(Key::Char(c)) = self { Some(*c) } else { None }
    }

    /// Returns the key if this is a `KeyDown` event.
    pub fn as_key_down(&self) -> Option<Key> {
        if let Event::KeyDown(k) = self { Some(*k) } else { None }
    }

    /// Returns `(x, y, button, pressed)` if this is a `MouseButton` event.
    pub fn as_mouse_btn(&self) -> Option<(i32, i32, MouseButton, bool)> {
        if let Event::MouseButton { x, y, button, pressed } = self {
            Some((*x, *y, *button, *pressed))
        } else { None }
    }

    /// Returns `(x, y)` if this is a `MouseMove` event.
    pub fn as_mouse_move(&self) -> Option<(i32, i32)> {
        if let Event::MouseMove { x, y } = self { Some((*x, *y)) } else { None }
    }
}
