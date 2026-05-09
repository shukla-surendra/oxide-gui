# oxide-gui

A portable, `no_std` GUI framework for systems software — designed to power desktop environments like **OxideOS**, with a Linux backend for development and testing.

Inspired by the architecture of [GNOME](https://www.gnome.org/) and modern Wayland compositors, oxide-gui provides the primitive drawing layer that a higher-level shell or compositor would sit on top of. It is intentionally low-level: immediate-mode rendering, a single `Backend` trait, and zero heap allocation in the core.

---

## Vision

OxideOS is a Rust-first operating system. oxide-gui is its graphical foundation:

```
OxideOS Kernel
    └── oxide-gui-core          (no_std, runs in kernel/userspace)
            ├── Canvas          (widget composition layer)
            ├── Backend trait   (framebuffer + input contract)
            └── Event system    (keyboard, mouse, window events)
                    │
                    ├── MinifbBackend   (Linux/X11/Wayland, for dev)
                    └── OxideBackend    (kernel framebuffer, production)
```

Like GNOME's Mutter compositor owns the framebuffer and delegates rendering to GTK apps via Wayland, OxideOS will expose a `Backend` implementation that maps directly to kernel graphics and HID drivers. Application code is unchanged between environments.

---

## Workspace Layout

```
oxide-gui/
├── crates/
│   ├── oxide-gui-core/         # no_std portable core — zero dependencies
│   └── oxide-gui-linux/        # Linux backend (minifb, X11/Wayland)
│       └── examples/
│           ├── hello_window.rs # minimal counter app
│           └── widgets_demo.rs # full desktop UI showcase
```

---

## Quick Start

Add the Linux backend to your `Cargo.toml`:

```toml
[dependencies]
oxide-gui-linux = { path = "crates/oxide-gui-linux" }
```

Minimal window loop:

```rust
use oxide_gui_linux::MinifbBackend;
use oxide_gui_core::{Canvas, Event, Key, palette};

fn main() {
    let mut backend = MinifbBackend::new("My App", 800, 600).unwrap();
    let mut canvas = Canvas::new(&mut backend);

    while backend.is_open() {
        canvas.clear(palette::DARK_BG);
        canvas.draw_text(32, 32, "Hello, OxideOS!", palette::TEXT);
        canvas.present();

        while let Some(event) = canvas.poll_event() {
            if event.is_close() { return; }
        }
    }
}
```

Run the bundled examples:

```bash
# Gradient counter demo (640×400) — keyboard + mouse input
cargo run --example hello_window -p oxide-gui-linux

# Full GNOME-style desktop (1280×800) — animated, tabbed, interactive
cargo run --example widgets_demo -p oxide-gui-linux
```

---

## Running & Testing the Examples

Both examples require a running X11 or Wayland display (any Linux desktop session). If you are on SSH without X forwarding, prepend `DISPLAY=:0`:

```bash
DISPLAY=:0 cargo run --example widgets_demo -p oxide-gui-linux
```

### `widgets_demo` controls (1280×800)

| Input | Action |
|---|---|
| `Tab` | Cycle tabs: Overview → Terminal → Apps → Settings |
| Mouse click on tab bar | Jump directly to that tab |
| Mouse hover | Highlights dock icons, app tiles, and buttons |
| `Q` or `Escape` | Quit |
| Window close | Quit |

**What to watch for:**
- **Overview tab** — CPU / RAM / GPU / Net bars animate in real time; mini CPU history chart updates live; clock in the top bar shows real system time
- **Terminal tab** — blinking cursor at the prompt; syntax-colored `ls` and `cat` output
- **Apps tab** — hover over any app tile to see shadow-lift effect
- **Settings tab** — toggle switches shown in teal (on) / dark (off); Apply button brightens on hover

### `hello_window` controls (640×400)

| Input | Action |
|---|---|
| `↑` or `→` | Increment counter |
| `↓` or `←` | Decrement counter |
| `R` | Reset counter to zero |
| Click `+ Increment` | Increment |
| Click `- Decrement` | Decrement |
| Click `Reset` | Reset |
| `Q` or `Escape` | Quit |

Counter box gradient and label change colour as the value grows: dark → teal → blue → amber → rose.

---

## Core API

### `Backend` trait — `oxide-gui-core`

The single abstraction over any display surface. Implement this to target a new platform.

| Method | Description |
|---|---|
| `width() -> u32` | Framebuffer width in pixels |
| `height() -> u32` | Framebuffer height in pixels |
| `fill_rect(x, y, w, h, color)` | Draw a solid rectangle |
| `present()` | Flush back-buffer to screen |
| `poll_event() -> Option<Event>` | Dequeue one input event |

Provided methods with default implementations (override for hardware acceleration):

| Method | Description |
|---|---|
| `draw_char(x, y, ch, color)` | Render a single glyph (8×16 bitmap) |
| `draw_text(x, y, text, color)` | Render a string |
| `hline(x, y, w, color)` | Horizontal 1px line |
| `vline(x, y, h, color)` | Vertical 1px line |
| `draw_rect(x, y, w, h, color)` | Hollow rectangle border |
| `clear(color)` | Fill entire framebuffer |
| `size() -> (u32, u32)` | Return (width, height) |

### `Canvas<B: Backend>` — high-level drawing

Wraps a `Backend` reference and adds composed widget helpers:

| Method | Description |
|---|---|
| `panel(x, y, w, h, fill, border)` | Filled rectangle with border |
| `title_bar(x, y, w, h, title, bg)` | Header bar with white text |
| `button(x, y, w, h, label, fill, border, text_color)` | Clickable button with centered label |
| `progress_bar(x, y, w, h, percent, track, fill, border)` | Horizontal progress indicator |
| `gradient_progress(x, y, w, h, percent, track, fill_l, fill_r, border)` | Progress bar with two-stop gradient fill |
| `gradient_h(x, y, w, h, left, right)` | Horizontal gradient fill left→right |
| `gradient_v(x, y, w, h, top, bottom)` | Vertical gradient fill top→bottom |
| `shadow_panel(x, y, w, h, fill, border)` | Panel with 4px drop shadow |
| `fill_rounded(x, y, w, h, color)` | Rectangle with 1px corner clip |
| `icon_tile(x, y, size, bg, label)` | Square app icon tile with colored background and centered label |
| `accent_bar(x, y, h, color)` | 3px vertical selection indicator bar |
| `dot(x, y, color)` | 4×4 filled status dot |
| `divider_h(x, y, w)` | Horizontal rule using `palette::DIVIDER` |
| `divider_v(x, y, h)` | Vertical rule using `palette::DIVIDER` |
| `centered_text(x, y, w, text, color)` | Horizontally centered string |
| `right_text(x, y, w, text, color)` | Right-aligned string |
| `backend_mut() -> &mut B` | Access the underlying backend |

### Events

```rust
match event {
    Event::KeyDown(Key::Char('q'))       => quit(),
    Event::KeyDown(Key::Enter)           => confirm(),
    Event::MouseButton { x, y, button: MouseButton::Left, pressed: true } => click(x, y),
    Event::MouseMove { x, y }            => hover(x, y),
    Event::Scroll { delta }              => scroll(delta),
    Event::Resize { width, height }      => resize(width, height),
    Event::Close                         => quit(),
    _ => {}
}
```

`Key` covers: `Char(char)`, `Enter`, `Backspace`, `Delete`, `Escape`, `Tab`, `Space`, arrow keys, Home/End/PageUp/PageDown, `LeftShift`/`RightShift`/`LeftCtrl`/`RightCtrl`/`LeftAlt`/`RightAlt`, and `F(u8)` for F1–F12.

### Color

```rust
use oxide_gui_core::color::{self, palette};

let red   = color::rgb(255, 0, 0);
let faded = color::argb(128, 255, 0, 0);

// Extract components
let r = color::red(c);
let g = color::green(c);
let b = color::blue(c);
let a = color::alpha(c);
```

**Built-in palette** (`palette::*`):

| Constant | Purpose |
|---|---|
| `BLACK`, `WHITE` | Pure extremes |
| `DARK_GRAY`, `GRAY`, `LIGHT_GRAY` | UI neutrals |
| `RED`, `GREEN`, `BLUE`, `YELLOW`, `CYAN` | Standard accents |
| `PURPLE`, `DEEP_PURPLE` | Violet / indigo accents |
| `PINK`, `ROSE` | Hot pink / rose red |
| `ORANGE`, `AMBER` | Warm highlight tones |
| `TEAL`, `NEON_CYAN` | Cool green-cyan accents |
| `INDIGO`, `ELECTRIC_BLUE` | Deep blue accents |
| `SURFACE`, `SURFACE2` | Deep-space dark backgrounds |
| `CARD_BG`, `CARD_BORDER` | Card fill and border |
| `ACCENT` | Primary brand color |
| `DARK_BG`, `PANEL_BG`, `TOOLBAR_BG` | Legacy theme backgrounds |
| `TEXT` | Primary text |
| `TEXT_DIM` | Secondary/muted text |
| `DIVIDER` | Separator lines |

`lerp_color(a, b, t, steps)` interpolates between any two colors using integer arithmetic — safe in `no_std`:

```rust
use oxide_gui_core::color::lerp_color;

let mid = lerp_color(palette::PURPLE, palette::ELECTRIC_BLUE, 1, 2); // midpoint
```

### Font

The embedded 8×16 bitmap font covers ASCII 0x20–0x7E (printable range). No external font files are needed — glyph data is baked into the binary as a const array.

```rust
use oxide_gui_core::font;

let px_wide = font::text_width("Settings");   // returns u32
// CHAR_W = 8, CHAR_H = 16
```

---

## Implementing a Custom Backend

Port to a new platform by implementing `Backend`:

```rust
use oxide_gui_core::{Backend, Color, Event, color};

pub struct KernelFramebuffer {
    fb: *mut u32,
    width: u32,
    height: u32,
}

impl Backend for KernelFramebuffer {
    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }

    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, c: Color) {
        let packed = color::to_rgb24(c);
        for row in y..y + h {
            for col in x..x + w {
                unsafe {
                    *self.fb.add((row * self.width + col) as usize) = packed;
                }
            }
        }
    }

    fn present(&mut self) {
        // kernel flush / vsync
    }

    fn poll_event(&mut self) -> Option<Event> {
        // read from kernel HID ring buffer
        None
    }
}
```

For the full OxideOS reference template see `crates/oxide-gui-core/src/oxide_os_integration.rs`.

---

## OxideOS Integration Roadmap

oxide-gui is designed to grow into a full desktop environment stack:

| Layer | Status | Description |
|---|---|---|
| `oxide-gui-core` | Done | `no_std` primitives, Backend trait, Canvas widgets |
| `oxide-gui-linux` | Done | minifb backend for Linux development |
| `oxide-gui-oxide` | Planned | Kernel framebuffer backend for OxideOS |
| Window manager | Planned | Tiling/stacking compositor (analogous to Mutter) |
| Application shell | Planned | App launcher, top bar, notification tray (analogous to GNOME Shell) |
| Widget toolkit | Planned | Retained-mode widget tree above Canvas (analogous to GTK4) |
| Wayland protocol | Future | Wayland compositor for running third-party apps |

The immediate-mode `Canvas` layer maps to what GTK does internally: primitive drawing commands against a surface. The retained widget tree, layout engine, and accessibility layer are the next step up.

---

## Design Principles

- **`no_std` core** — compiles in a kernel or on bare metal with no allocator
- **Single trait boundary** — swap the entire platform by changing one type
- **Immediate-mode rendering** — no hidden state; the app owns the render loop
- **Zero external deps in core** — bitmap font, color math, and event types are self-contained
- **GNOME-aligned vocabulary** — panel, title bar, status bar, sidebar, tab bar match GNOME Shell's component model, easing the path to a familiar desktop UX

---

## License

MIT OR Apache-2.0
