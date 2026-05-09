//! Minimal oxide-gui hello-world on Linux.
//! Run with:  cargo run --example hello_window -p oxide-gui-linux

use oxide_gui_linux::MinifbBackend;
use oxide_gui_core::{Backend, Canvas, Event, Key};
use oxide_gui_core::color::palette;

fn main() {
    let mut backend = MinifbBackend::new("oxide-gui — Hello Window", 640, 400)
        .expect("failed to open window");

    let mut counter: u32 = 0;

    loop {
        if !backend.is_open() { break; }

        // ── Draw ──────────────────────────────────────────────────────────
        {
            let mut canvas = Canvas::new(&mut backend);

            canvas.clear(palette::DARK_BG);

            // Toolbar
            canvas.fill_rect(0, 0, 640, 32, palette::TOOLBAR_BG);
            canvas.divider_h(0, 31, 640);
            canvas.draw_text(12, 8, "oxide-gui", palette::CYAN);

            let msg = "Hello from oxide-gui-core + oxide-gui-linux!";
            canvas.centered_text(0, 80, 640, msg, palette::TEXT);

            // Counter box
            canvas.panel(220, 130, 200, 50, palette::PANEL_BG, palette::DIVIDER);
            let mut buf = [0u8; 32];
            let s = fmt_u32(&mut buf, counter);
            canvas.centered_text(220, 148, 200, s, palette::CYAN);

            // Buttons
            canvas.button(180, 210, 100, 30, "- decr",
                          palette::TOOLBAR_BG, palette::DIVIDER, palette::TEXT);
            canvas.button(360, 210, 100, 30, "+ incr",
                          palette::TOOLBAR_BG, palette::DIVIDER, palette::TEXT);

            canvas.draw_text(12, 370,
                "Press Q to quit  |  arrow keys or buttons to change counter",
                palette::TEXT_DIM);

            canvas.present();
        } // canvas dropped here — borrow released

        // ── Events ────────────────────────────────────────────────────────
        while let Some(ev) = backend.poll_event() {
            match ev {
                Event::Close => return,
                Event::KeyDown(Key::Char('q')) | Event::KeyDown(Key::Escape) => return,
                Event::KeyDown(Key::Up)   | Event::KeyDown(Key::Right) => counter += 1,
                Event::KeyDown(Key::Down) | Event::KeyDown(Key::Left)  => {
                    counter = counter.saturating_sub(1);
                }
                Event::MouseButton { x, y, pressed: true, .. } => {
                    if y >= 210 && y < 240 {
                        if x >= 180 && x < 280 { counter = counter.saturating_sub(1); }
                        if x >= 360 && x < 460 { counter += 1; }
                    }
                }
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn fmt_u32<'a>(buf: &'a mut [u8; 32], mut v: u32) -> &'a str {
    let mut i = buf.len();
    if v == 0 { i -= 1; buf[i] = b'0'; }
    while v > 0 { i -= 1; buf[i] = b'0' + (v % 10) as u8; v /= 10; }
    std::str::from_utf8(&buf[i..]).unwrap_or("?")
}
