//! oxide-gui hello-world — vibrant gradient counter demo.
//! Run with:  cargo run --example hello_window -p oxide-gui-linux

use oxide_gui_linux::MinifbBackend;
use oxide_gui_core::{Backend, Canvas, Event, Key};
use oxide_gui_core::color::{palette, lerp_color, rgb};

const W: u32 = 640;
const H: u32 = 400;

// Button layout
const BTN_W: u32 = 120;
const BTN_H: u32 = 36;
const BTN_DEC_X: u32 = 160;
const BTN_INC_X: u32 = 360;
const BTN_Y: u32 = 216;

fn main() {
    let mut backend = MinifbBackend::new("oxide-gui — Counter", W, H)
        .expect("failed to open window");

    let mut counter: u32 = 0;
    let mut mouse_x = 0i32;
    let mut mouse_y = 0i32;
    let mut frame = 0u32;

    loop {
        if !backend.is_open() { break; }
        frame = frame.wrapping_add(1);

        {
            let mut c = Canvas::new(&mut backend);

            // ── Background: deep space gradient ──
            c.gradient_v(0, 0, W, H, rgb(0x0A, 0x0A, 0x18), rgb(0x0E, 0x06, 0x1A));

            // ── Header bar ──
            c.gradient_h(0, 0, W, 36, rgb(0x18, 0x08, 0x30), rgb(0x06, 0x18, 0x2E));
            c.hline(0, 35, W, rgb(0x33, 0x22, 0x55));
            // Traffic-light dots
            c.fill_rounded(10, 12, 12, 12, palette::ROSE);
            c.fill_rounded(26, 12, 12, 12, palette::AMBER);
            c.fill_rounded(42, 12, 12, 12, rgb(0x22,0xC5,0x5E));
            c.centered_text(0, 10, W, "oxide-gui Counter Demo", palette::TEXT_DIM);

            // ── Subtitle ──
            let subtitle = "Arrow keys or click the buttons";
            c.centered_text(0, 58, W, subtitle, palette::TEXT_DIM);

            // ── Counter glow ring (pulsing with frame) ──
            let t = frame as f32;
            let pulse = ((t * 0.05).sin() * 0.5 + 0.5) as u32; // 0 or 0
            let _glow_size = 10 + pulse;
            let cx = (W - 200) / 2;
            let cy = 100u32;
            // Outer glow: dim color ring
            let glow_col = lerp_color(palette::DEEP_PURPLE, palette::PURPLE,
                                      ((t * 0.05).sin() * 127.0 + 128.0) as u32, 255);
            c.draw_rect(cx - 2, cy - 2, 204, 84, glow_col);
            c.draw_rect(cx - 1, cy - 1, 202, 82, lerp_color(glow_col, palette::WHITE, 1, 4));

            // Counter box with gradient background
            let box_col_l = counter_color_l(counter);
            let box_col_r = counter_color_r(counter);
            c.gradient_h(cx, cy, 200, 80, box_col_l, box_col_r);
            c.draw_rect(cx, cy, 200, 80, lerp_color(box_col_r, palette::WHITE, 1, 3));

            // Counter value — large and centered
            let mut buf = [0u8; 12];
            let s = fmt_u32(&mut buf, counter);
            // Draw 2x scale by drawing the text twice offset by 1px each direction
            let tw = (s.len() as u32) * 8;
            let tx = cx + (200 - tw) / 2;
            let ty = cy + 32;
            // Shadow
            c.draw_text(tx + 1, ty + 1, s, rgb(0, 0, 0));
            c.draw_text(tx, ty, s, palette::WHITE);

            // Counter label
            let label_col = match counter {
                0       => palette::TEXT_DIM,
                1..=10  => palette::CYAN,
                11..=50 => palette::TEAL,
                51..=99 => palette::AMBER,
                _       => palette::ROSE,
            };
            c.centered_text(cx, cy + 58, 200, counter_label(counter), label_col);

            // ── Buttons ──
            let hover_dec = mouse_in_btn(mouse_x, mouse_y, BTN_DEC_X, BTN_Y);
            let hover_inc = mouse_in_btn(mouse_x, mouse_y, BTN_INC_X, BTN_Y);

            draw_action_btn(&mut c, BTN_DEC_X, BTN_Y, "- Decrement",
                            rgb(0x30,0x08,0x20), palette::ROSE, hover_dec);
            draw_action_btn(&mut c, BTN_INC_X, BTN_Y, "+ Increment",
                            rgb(0x04,0x30,0x28), palette::TEAL, hover_inc);

            // Reset button
            let hover_rst = mouse_x >= 280 && mouse_x < 360
                         && mouse_y >= BTN_Y as i32 && mouse_y < (BTN_Y + BTN_H) as i32;
            c.gradient_h(280, BTN_Y, 80, BTN_H,
                         if hover_rst { rgb(0x20,0x20,0x44) } else { rgb(0x10,0x10,0x22) },
                         if hover_rst { rgb(0x30,0x30,0x66) } else { rgb(0x18,0x18,0x30) });
            c.draw_rect(280, BTN_Y, 80, BTN_H, palette::CARD_BORDER);
            c.centered_text(280, BTN_Y + (BTN_H - 16) / 2, 80, "Reset", palette::TEXT_DIM);

            // ── Footer ──
            c.gradient_h(0, H - 28, W, 28, rgb(0x0A,0x04,0x18), rgb(0x04,0x0A,0x18));
            c.hline(0, H - 28, W, rgb(0x22,0x22,0x38));
            c.centered_text(0, H - 20, W,
                "Q: quit   Arrow keys: change   R: reset",
                palette::TEXT_DIM);

            c.present();
        }

        while let Some(ev) = backend.poll_event() {
            match ev {
                Event::Close => return,
                Event::KeyDown(Key::Char('q')) | Event::KeyDown(Key::Escape) => return,
                Event::KeyDown(Key::Char('r')) => counter = 0,
                Event::KeyDown(Key::Up)   | Event::KeyDown(Key::Right) => counter += 1,
                Event::KeyDown(Key::Down) | Event::KeyDown(Key::Left) => {
                    counter = counter.saturating_sub(1);
                }
                Event::MouseMove { x, y } => { mouse_x = x; mouse_y = y; }
                Event::MouseButton { x, y, pressed: true, .. } => {
                    if mouse_in_btn(x, y, BTN_DEC_X, BTN_Y) {
                        counter = counter.saturating_sub(1);
                    }
                    if mouse_in_btn(x, y, BTN_INC_X, BTN_Y) {
                        counter += 1;
                    }
                    if x >= 280 && x < 360 && y >= BTN_Y as i32 && y < (BTN_Y + BTN_H) as i32 {
                        counter = 0;
                    }
                }
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn draw_action_btn(c: &mut Canvas<impl Backend>, x: u32, y: u32, label: &str,
                   dark: u32, bright: u32, hovered: bool) {
    let l = if hovered { lerp_color(dark, bright, 1, 3) } else { dark };
    let r = if hovered { bright } else { lerp_color(dark, bright, 1, 2) };
    c.gradient_h(x, y, BTN_W, BTN_H, l, r);
    c.draw_rect(x, y, BTN_W, BTN_H,
                if hovered { bright } else { lerp_color(dark, bright, 1, 4) });
    c.centered_text(x, y + (BTN_H - 16) / 2, BTN_W, label, palette::WHITE);
}

fn mouse_in_btn(mx: i32, my: i32, bx: u32, by: u32) -> bool {
    mx >= bx as i32 && mx < (bx + BTN_W) as i32
 && my >= by as i32 && my < (by + BTN_H) as i32
}

fn counter_color_l(v: u32) -> u32 {
    match v {
        0       => rgb(0x10,0x10,0x22),
        1..=10  => rgb(0x04,0x28,0x30),
        11..=50 => rgb(0x04,0x28,0x20),
        51..=99 => rgb(0x30,0x20,0x00),
        _       => rgb(0x30,0x04,0x10),
    }
}
fn counter_color_r(v: u32) -> u32 {
    match v {
        0       => rgb(0x20,0x20,0x40),
        1..=10  => palette::TEAL,
        11..=50 => palette::ELECTRIC_BLUE,
        51..=99 => palette::AMBER,
        _       => palette::ROSE,
    }
}
fn counter_label(v: u32) -> &'static str {
    match v {
        0       => "zero",
        1..=10  => "low",
        11..=50 => "medium",
        51..=99 => "high",
        _       => "over 100!",
    }
}

fn fmt_u32<'a>(buf: &'a mut [u8; 12], mut v: u32) -> &'a str {
    let mut i = buf.len();
    if v == 0 { i -= 1; buf[i] = b'0'; }
    while v > 0 { i -= 1; buf[i] = b'0' + (v % 10) as u8; v /= 10; }
    std::str::from_utf8(&buf[i..]).unwrap_or("?")
}
