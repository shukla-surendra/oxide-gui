//! Widgets demo — desktop-like layout similar to OxideOS.
//! Run with:  cargo run --example widgets_demo -p oxide-gui-linux

use oxide_gui_linux::MinifbBackend;
use oxide_gui_core::{Backend, Canvas, Event, Key};
use oxide_gui_core::color::{palette, rgb};

const W: u32 = 900;
const H: u32 = 600;
const TASKBAR_H: u32 = 36;
const SIDEBAR_W: u32 = 200;

fn main() {
    let mut backend = MinifbBackend::new("oxide-gui — Widgets Demo", W, H)
        .expect("failed to open window");

    let mut selected_tab = 0usize;
    const TABS: &[&str] = &["Overview", "Files", "Settings"];

    loop {
        if !backend.is_open() { break; }

        // ── Draw ──────────────────────────────────────────────────────────
        {
            let mut canvas = Canvas::new(&mut backend);

            canvas.clear(palette::DARK_BG);

            // Taskbar
            canvas.fill_rect(0, 0, W, TASKBAR_H, palette::TOOLBAR_BG);
            canvas.divider_h(0, TASKBAR_H - 1, W);
            canvas.draw_text(12, 10, "OxideOS", palette::CYAN);
            canvas.right_text(0, 10, W - 12, "14:32  Fri 9 May", palette::TEXT_DIM);

            // Sidebar
            canvas.fill_rect(0, TASKBAR_H, SIDEBAR_W, H - TASKBAR_H, rgb(0x25,0x25,0x26));
            canvas.vline(SIDEBAR_W, TASKBAR_H, H - TASKBAR_H, palette::DIVIDER);
            canvas.draw_text(12, TASKBAR_H + 12, "PLACES", palette::TEXT_DIM);
            canvas.divider_h(0, TASKBAR_H + 28, SIDEBAR_W);

            let places = &["/ Root","/bin","/dev","/disk","/tmp"];
            for (i, &label) in places.iter().enumerate() {
                let iy = TASKBAR_H + 32 + i as u32 * 24;
                if i == 0 {
                    canvas.fill_rect(0, iy, SIDEBAR_W, 24, rgb(0x37,0x37,0x3D));
                    canvas.fill_rect(0, iy, 2, 24, palette::ACCENT);
                    canvas.draw_text(16, iy + 5, label, palette::CYAN);
                } else {
                    canvas.draw_text(16, iy + 5, label, palette::TEXT);
                }
            }

            // Tab bar
            let main_x = SIDEBAR_W + 1;
            let tab_h  = 30u32;
            for (i, &tab) in TABS.iter().enumerate() {
                let tx     = main_x + i as u32 * 120;
                let is_sel = i == selected_tab;
                canvas.fill_rect(tx, TASKBAR_H, 120, tab_h,
                                 if is_sel { palette::DARK_BG } else { rgb(0x2D,0x2D,0x30) });
                if is_sel { canvas.hline(tx, TASKBAR_H + tab_h - 2, 120, palette::CYAN); }
                canvas.centered_text(tx, TASKBAR_H + 7, 120, tab,
                                     if is_sel { palette::TEXT } else { palette::TEXT_DIM });
            }
            canvas.divider_h(main_x, TASKBAR_H + tab_h, W - main_x);

            let content_y = TASKBAR_H + tab_h + 1;
            match selected_tab {
                0 => draw_overview(&mut canvas, main_x, content_y),
                1 => draw_files   (&mut canvas, main_x, content_y),
                2 => draw_settings(&mut canvas, main_x, content_y),
                _ => {}
            }

            // Status bar
            canvas.fill_rect(0, H - 22, W, 22, palette::STATUS_BG);
            canvas.draw_text(12, H - 18, "oxide-gui v0.1.0   Tab: switch tabs   Q: quit",
                             palette::WHITE);

            canvas.present();
        } // canvas dropped

        // ── Events ────────────────────────────────────────────────────────
        while let Some(ev) = backend.poll_event() {
            match ev {
                Event::Close => return,
                Event::KeyDown(Key::Char('q')) | Event::KeyDown(Key::Escape) => return,
                Event::KeyDown(Key::Tab) => selected_tab = (selected_tab + 1) % TABS.len(),
                Event::MouseButton { x, y, pressed: true, .. } => {
                    let tx0 = (SIDEBAR_W + 1) as i32;
                    if y >= TASKBAR_H as i32 && y < (TASKBAR_H + 30) as i32 {
                        let rel = x - tx0;
                        if rel >= 0 {
                            let ti = (rel / 120) as usize;
                            if ti < TABS.len() { selected_tab = ti; }
                        }
                    }
                }
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn draw_overview(canvas: &mut Canvas<impl Backend>, x: u32, y: u32) {
    let cx = x + 20; let mut cy = y + 20; let row = 26u32;
    canvas.draw_text(cx, cy, "System Overview", palette::CYAN); cy += row + 4;
    canvas.divider_h(cx, cy, 400); cy += 12;
    let items = &[
        ("OS",      "OxideOS 0.1.0-dev (x86_64)"),
        ("Kernel",  "Rust no_std monolithic"),
        ("GUI lib", "oxide-gui-core v0.1.0"),
        ("Backend", "oxide-gui-linux (minifb)"),
        ("Memory",  "128 MB"), ("Disk", "No disk attached"),
    ];
    for &(label, value) in items {
        canvas.draw_text(cx, cy, label, palette::TEXT_DIM);
        canvas.draw_text(cx + 100, cy, value, palette::TEXT);
        cy += row;
    }
    cy += 10;
    canvas.draw_text(cx, cy, "CPU usage", palette::TEXT_DIM); cy += 20;
    canvas.progress_bar(cx, cy, 300, 14, 42, rgb(0x0D,0x1B,0x2A), palette::BLUE, palette::DIVIDER);
    canvas.draw_text(cx + 306, cy, "42%", palette::TEXT_DIM); cy += 24;
    canvas.draw_text(cx, cy, "RAM usage", palette::TEXT_DIM); cy += 20;
    canvas.progress_bar(cx, cy, 300, 14, 28, rgb(0x0D,0x1B,0x2A), palette::CYAN, palette::DIVIDER);
    canvas.draw_text(cx + 306, cy, "28%", palette::TEXT_DIM);
}

fn draw_files(canvas: &mut Canvas<impl Backend>, x: u32, y: u32) {
    let cx = x + 20; let mut cy = y + 20;
    canvas.draw_text(cx, cy, "File Browser  —  /", palette::CYAN); cy += 30;
    canvas.fill_rect(cx, cy, 560, 20, rgb(0x2D,0x2D,0x30));
    canvas.draw_text(cx + 4, cy + 3, "NAME", palette::TEXT_DIM);
    canvas.draw_text(cx + 360, cy + 3, "TYPE", palette::TEXT_DIM);
    canvas.draw_text(cx + 460, cy + 3, "SIZE", palette::TEXT_DIM);
    cy += 22;
    let entries = &[
        ("bin","DIR",""), ("dev","DIR",""), ("disk","DIR",""),
        ("proc","DIR",""), ("tmp","DIR",""), ("kernel.elf","FILE","1.2M"),
    ];
    for (i, &(name, kind, size)) in entries.iter().enumerate() {
        let row_bg = if i == 0 { rgb(0x09,0x47,0x71) }
                     else if i % 2 == 0 { palette::DARK_BG }
                     else { rgb(0x25,0x25,0x26) };
        canvas.fill_rect(cx, cy, 560, 20, row_bg);
        if i == 0 { canvas.fill_rect(cx, cy, 2, 20, palette::CYAN); }
        let col = if kind == "DIR" { palette::CYAN } else { palette::TEXT };
        canvas.draw_text(cx + 6, cy + 3, name, col);
        canvas.draw_text(cx + 360, cy + 3, kind, col);
        if !size.is_empty() { canvas.draw_text(cx + 460, cy + 3, size, palette::TEXT_DIM); }
        cy += 22;
    }
}

fn draw_settings(canvas: &mut Canvas<impl Backend>, x: u32, y: u32) {
    let cx = x + 20; let mut cy = y + 20;
    canvas.draw_text(cx, cy, "Settings", palette::CYAN); cy += 36;
    let sections: &[(&str, &[(&str, &str)])] = &[
        ("Display", &[("Resolution","1280 × 800"),("Refresh","60 Hz")]),
        ("Network", &[("Interface","RTL8139"),("IP","10.0.2.15"),("Gateway","10.0.2.2")]),
    ];
    for &(title, pairs) in sections {
        canvas.draw_text(cx, cy, title, palette::TEXT); cy += 4;
        canvas.divider_h(cx, cy, 400); cy += 12;
        for &(k, v) in pairs {
            canvas.draw_text(cx + 8, cy, k, palette::TEXT_DIM);
            canvas.draw_text(cx + 160, cy, v, palette::TEXT);
            cy += 22;
        }
        cy += 12;
    }
    canvas.button(cx,       cy, 120, 28, "Apply",
                  rgb(0x0D,0x5F,0xA0), palette::BLUE, palette::WHITE);
    canvas.button(cx + 130, cy, 120, 28, "Revert",
                  rgb(0x2D,0x2D,0x30), palette::DIVIDER, palette::TEXT);
}
