//! OxideOS Desktop — vibrant GNOME-style showcase.
//! Run with:  cargo run --example widgets_demo -p oxide-gui-linux

use oxide_gui_linux::MinifbBackend;
use oxide_gui_core::{Backend, Canvas, Event, Key};
use oxide_gui_core::color::{palette, rgb, lerp_color};

// ── Layout constants ──────────────────────────────────────────────────────────

const W: u32 = 1280;
const H: u32 = 800;
const TOPBAR_H: u32 = 40;
const DOCK_W: u32 = 68;
const TAB_H: u32 = 36;
const STATUS_H: u32 = 24;
const CONTENT_X: u32 = DOCK_W;
const CONTENT_Y: u32 = TOPBAR_H + TAB_H;
const CONTENT_W: u32 = W - DOCK_W;
const CONTENT_H: u32 = H - TOPBAR_H - TAB_H - STATUS_H;
const STATUS_Y: u32 = H - STATUS_H;

const ICON_SIZE: u32 = 44;
const ICON_X: u32 = (DOCK_W - ICON_SIZE) / 2;
const TAB_W: u32 = 160;

// ── App / icon data ───────────────────────────────────────────────────────────

const DOCK_ICONS: &[(&str, u32, u8)] = &[
    ("Fi", palette::TEAL,          0),
    ("Tm", rgb(0x22, 0xC5, 0x5E),  1),
    ("Br", palette::ELECTRIC_BLUE, 0),
    ("St", palette::ORANGE,        0),
    ("Ap", palette::PURPLE,        3),
];

const TABS: &[&str] = &["Overview", "Terminal", "Apps", "Settings"];

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let mut backend = MinifbBackend::new("OxideOS — oxide-gui desktop", W, H)
        .expect("failed to open window");

    let mut selected_tab  = 0usize;
    let mut _selected_file = 0usize;
    let mut mouse_x = 0i32;
    let mut mouse_y = 0i32;
    let mut frame   = 0u32;

    loop {
        if !backend.is_open() { break; }
        frame = frame.wrapping_add(1);

        {
            let mut c = Canvas::new(&mut backend);
            c.clear(palette::SURFACE);

            draw_topbar(&mut c, frame);
            draw_dock(&mut c, selected_tab, mouse_x, mouse_y);
            draw_tabs(&mut c, selected_tab, mouse_x, mouse_y);

            match selected_tab {
                0 => draw_overview(&mut c, frame),
                1 => draw_terminal(&mut c, frame),
                2 => draw_apps(&mut c, mouse_x, mouse_y),
                3 => draw_settings(&mut c, mouse_x, mouse_y),
                _ => {}
            }

            draw_statusbar(&mut c);
            c.present();
        }

        while let Some(ev) = backend.poll_event() {
            match ev {
                Event::Close => return,
                Event::KeyDown(Key::Char('q')) | Event::KeyDown(Key::Escape) => return,
                Event::KeyDown(Key::Tab) => {
                    selected_tab = (selected_tab + 1) % TABS.len();
                }
                Event::MouseMove { x, y } => { mouse_x = x; mouse_y = y; }
                Event::MouseButton { x, y, pressed: true, .. } => {
                    // Tab clicks
                    let rel = x - CONTENT_X as i32;
                    if y >= TOPBAR_H as i32 && y < (TOPBAR_H + TAB_H) as i32 && rel >= 0 {
                        let ti = rel as usize / TAB_W as usize;
                        if ti < TABS.len() { selected_tab = ti; }
                    }
                    // File row clicks in terminal tab (repurposed as file browser)
                    if selected_tab == 0 {
                        let ry = y - (CONTENT_Y + 136) as i32;
                        if ry >= 0 && x >= (CONTENT_X + 20) as i32 {
                            let row = ry as usize / 22;
                            if row < 6 { _selected_file = row; }
                        }
                    }
                }
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

// ── Section renderers ─────────────────────────────────────────────────────────

fn draw_topbar(c: &mut Canvas<impl Backend>, frame: u32) {
    // Gradient background: deep navy → deep purple
    c.gradient_h(0, 0, W, TOPBAR_H, rgb(0x0E, 0x0E, 0x1C), rgb(0x1A, 0x0C, 0x2E));
    c.hline(0, TOPBAR_H - 1, W, rgb(0x33, 0x22, 0x55));

    // Workspace dots
    let dot_colors = [palette::CYAN, palette::PURPLE, palette::ORANGE];
    for (i, &col) in dot_colors.iter().enumerate() {
        let dx = 14 + i as u32 * 14;
        c.fill_rounded(dx, 16, 8, 8, col);
    }

    // App name
    c.draw_text(58, 12, "OxideOS", palette::TEXT);

    // Separator
    c.vline(130, 8, 24, rgb(0x33, 0x22, 0x55));

    // Current app title
    c.draw_text(142, 12, "System Monitor", palette::TEXT_DIM);

    // Clock (centered)
    let mut time_buf = [0u8; 8];
    let time_str = real_time(&mut time_buf);
    let tw = (time_str.len() as u32) * 8;
    c.draw_text((W - tw) / 2, 12, time_str, palette::TEXT);

    // Right indicators
    let rx = W - 230;

    // Battery bar
    let batt = 75u32 + (frame / 240) % 25;
    c.draw_rect(rx, 13, 28, 14, palette::TEXT_DIM);
    c.fill_rect(rx + 28, 17, 3, 6, palette::TEXT_DIM);
    let batt_fill = 26 * batt / 100;
    let batt_col = if batt > 30 { palette::TEAL } else { palette::ROSE };
    c.fill_rect(rx + 1, 14, batt_fill, 12, batt_col);
    c.draw_text(rx + 36, 12, "  )))  vol  surendra", palette::TEXT_DIM);
}

fn draw_dock(c: &mut Canvas<impl Backend>, active_tab: usize, mx: i32, my: i32) {
    let dock_y = TOPBAR_H;
    let dock_h = H - TOPBAR_H - STATUS_H;

    // Dock background
    c.gradient_v(0, dock_y, DOCK_W, dock_h, rgb(0x10, 0x10, 0x1E), rgb(0x0A, 0x0A, 0x14));
    c.vline(DOCK_W - 1, dock_y, dock_h, rgb(0x22, 0x22, 0x38));

    // App icons
    let active_dock = match active_tab { 1 => 1, 2 => 4, 3 => 3, _ => 0 };

    for (i, &(label, bg, notif)) in DOCK_ICONS.iter().enumerate() {
        let iy = dock_y + 10 + i as u32 * (ICON_SIZE + 8);
        let hovered = mx >= ICON_X as i32 && mx < (ICON_X + ICON_SIZE) as i32
                   && my >= iy as i32 && my < (iy + ICON_SIZE) as i32;
        let tile_bg = if hovered {
            lerp_color(bg, palette::WHITE, 1, 5)
        } else {
            bg
        };
        c.icon_tile(ICON_X, iy, ICON_SIZE, tile_bg, label);

        // Active indicator bar on left edge
        if i == active_dock {
            c.accent_bar(0, iy + 4, ICON_SIZE - 8, palette::CYAN);
        } else if hovered {
            c.accent_bar(0, iy + 12, ICON_SIZE - 24, palette::TEXT_DIM);
        }

        // Notification badge
        if notif > 0 {
            let bx = ICON_X + ICON_SIZE - 10;
            let by = iy + 2;
            c.fill_rounded(bx, by, 10, 10, palette::ROSE);
            let s = if notif < 10 { &["0","1","2","3","4","5","6","7","8","9"][notif as usize] } else { "+" };
            c.centered_text(bx, by + 1, 10, s, palette::WHITE);
        }
    }

    // Separator
    let sep_y = dock_y + 10 + DOCK_ICONS.len() as u32 * (ICON_SIZE + 8) + 4;
    c.hline(8, sep_y, DOCK_W - 16, rgb(0x22, 0x22, 0x38));

    // Power icon at bottom
    let power_y = dock_y + dock_h - ICON_SIZE - 10;
    let power_hovered = mx >= ICON_X as i32 && mx < (ICON_X + ICON_SIZE) as i32
                     && my >= power_y as i32 && my < (power_y + ICON_SIZE) as i32;
    c.icon_tile(ICON_X, power_y, ICON_SIZE, if power_hovered { palette::ROSE } else { rgb(0x3A, 0x12, 0x1A) }, "Pw");
}

fn draw_tabs(c: &mut Canvas<impl Backend>, selected: usize, mx: i32, my: i32) {
    let ty = TOPBAR_H;
    c.fill_rect(CONTENT_X, ty, CONTENT_W, TAB_H, palette::SURFACE2);
    c.hline(CONTENT_X, ty + TAB_H - 1, CONTENT_W, rgb(0x22, 0x22, 0x38));

    for (i, &tab) in TABS.iter().enumerate() {
        let tx = CONTENT_X + i as u32 * TAB_W;
        let hovered = mx >= tx as i32 && mx < (tx + TAB_W) as i32
                   && my >= ty as i32 && my < (ty + TAB_H) as i32;
        let is_sel  = i == selected;

        let bg = if is_sel { palette::SURFACE }
                 else if hovered { rgb(0x1C, 0x1C, 0x2C) }
                 else { palette::SURFACE2 };

        c.fill_rect(tx, ty, TAB_W, TAB_H, bg);

        let tc = if is_sel { palette::TEXT } else { palette::TEXT_DIM };
        c.centered_text(tx, ty + (TAB_H - 16) / 2, TAB_W, tab, tc);

        // Active indicator: gradient underline
        if is_sel {
            c.gradient_h(tx, ty + TAB_H - 3, TAB_W, 3, palette::PURPLE, palette::ELECTRIC_BLUE);
        }
    }
}

fn draw_statusbar(c: &mut Canvas<impl Backend>) {
    c.gradient_h(0, STATUS_Y, W, STATUS_H, rgb(0x1A, 0x08, 0x30), rgb(0x06, 0x18, 0x30));
    c.hline(0, STATUS_Y, W, rgb(0x33, 0x22, 0x55));
    c.draw_text(14, STATUS_Y + 4,
        "oxide-gui v0.1.0   Tab: switch view   Q: quit   Click tabs or dock icons",
        palette::TEXT_DIM);
    c.right_text(0, STATUS_Y + 4, W - 12, "OxideOS 0.1.0-dev", rgb(0x44, 0x33, 0x66));
}

// ── Overview tab ─────────────────────────────────────────────────────────────

fn draw_overview(c: &mut Canvas<impl Backend>, frame: u32) {
    let t = frame as f32;
    let cpu = (50.0 + 20.0 * (t * 0.04).sin()) as u32;
    let ram = (30.0 + 8.0  * (t * 0.02).sin()) as u32;
    let gpu = (60.0 + 22.0 * (t * 0.07).sin()) as u32;
    let net = (25.0 + 25.0 * (t * 0.12).sin().abs()) as u32;

    let pad   = 20u32;
    let cw    = (CONTENT_W - pad * 3) / 2;
    let cx0   = CONTENT_X + pad;
    let cx1   = cx0 + cw + pad;
    let cy    = CONTENT_Y + pad;
    let ch    = 310u32;

    // ── Left card: System Info ────────────────────────────────────────────
    c.shadow_panel(cx0, cy, cw, ch, palette::CARD_BG, palette::CARD_BORDER);
    c.gradient_h(cx0, cy, cw, 34, palette::DEEP_PURPLE, palette::PURPLE);
    c.draw_text(cx0 + 10, cy + 9, "System Overview", palette::WHITE);

    let mut ky = cy + 46;
    let kx = cx0 + 14;
    let vx = cx0 + 170;
    let row = 24u32;

    let info = [
        ("OS",        "OxideOS 0.1.0-dev (x86_64)"),
        ("Kernel",    "Rust no_std monolithic"),
        ("GUI lib",   "oxide-gui-core v0.1.0"),
        ("Backend",   "oxide-gui-linux (minifb)"),
        ("Display",   "1280 x 800  60 Hz"),
        ("Uptime",    uptime_str(frame)),
        ("Memory",    "128 MB total"),
        ("Free mem",  "94 MB available"),
        ("Processes", "14 running"),
        ("Users",     "surendra (active)"),
    ];
    for &(k, v) in &info {
        c.draw_text(kx, ky, k, palette::TEXT_DIM);
        c.draw_text(vx, ky, v, palette::TEXT);
        ky += row;
    }

    // ── Right card: Resource Monitor ─────────────────────────────────────
    c.shadow_panel(cx1, cy, cw, ch, palette::CARD_BG, palette::CARD_BORDER);
    c.gradient_h(cx1, cy, cw, 34, palette::TEAL, palette::ELECTRIC_BLUE);
    c.draw_text(cx1 + 10, cy + 9, "Resource Monitor", rgb(0x0A, 0x0A, 0x12));

    let bx = cx1 + 14;
    let bw = cw - 90;
    let mut by = cy + 52;
    let bh = 16u32;
    let brow = 52u32;

    let bars: &[(&str, u32, u32, u32)] = &[
        ("CPU",  cpu,  palette::INDIGO,        palette::ELECTRIC_BLUE),
        ("RAM",  ram,  palette::DEEP_PURPLE,   palette::PURPLE),
        ("GPU",  gpu,  palette::ORANGE,        palette::AMBER),
        ("Net",  net,  rgb(0x00,0x60,0x40),    palette::TEAL),
    ];
    for &(label, pct, fl, fr) in bars {
        c.draw_text(bx, by, label, palette::TEXT_DIM);
        let bar_y = by + 18;
        c.gradient_progress(bx, bar_y, bw, bh, pct,
                            rgb(0x08, 0x08, 0x14), fl, fr, palette::CARD_BORDER);
        let mut nbuf = [0u8; 4];
        let ns = fmt_pct(&mut nbuf, pct);
        c.draw_text(bx + bw + 6, bar_y, ns, palette::TEXT_DIM);
        by += brow;
    }

    // Mini CPU history chart
    c.draw_text(bx, by + 4, "CPU history", palette::TEXT_DIM);
    by += 22;
    for i in 0..40u32 {
        let sample_t = (frame as f32 * 0.04) - (40 - i) as f32 * 0.08;
        let pct = ((50.0 + 20.0 * sample_t.sin()) as u32).min(bh * 2);
        let bar_h = (pct * (bh * 2) / 100).max(2);
        let bar_col = lerp_color(palette::INDIGO, palette::ELECTRIC_BLUE, i, 39);
        c.fill_rect(bx + i * 7, by + bh * 2 - bar_h, 5, bar_h, bar_col);
    }

    // ── Quick action buttons ──────────────────────────────────────────────
    let qy  = cy + ch + pad;
    let bw2 = 176u32;
    let bh2 = 38u32;
    let actions: &[(&str, u32, u32)] = &[
        ("New Window", palette::INDIGO,        palette::ELECTRIC_BLUE),
        ("Terminal",   rgb(0x06,0x60,0x30),    palette::TEAL),
        ("Files",      rgb(0x00,0x50,0x50),    palette::CYAN),
        ("Settings",   rgb(0x60,0x30,0x00),    palette::ORANGE),
        ("Suspend",    rgb(0x40,0x06,0x16),    palette::ROSE),
    ];
    for (i, &(label, fl, fr)) in actions.iter().enumerate() {
        let bx2 = cx0 + i as u32 * (bw2 + 14);
        c.gradient_h(bx2, qy, bw2, bh2, fl, fr);
        c.draw_rect(bx2, qy, bw2, bh2, palette::CARD_BORDER);
        c.centered_text(bx2, qy + (bh2 - 16) / 2, bw2, label, palette::WHITE);
    }
}

// ── Terminal tab ──────────────────────────────────────────────────────────────

fn draw_terminal(c: &mut Canvas<impl Backend>, frame: u32) {
    let tx = CONTENT_X + 20;
    let ty = CONTENT_Y + 16;
    let tw = CONTENT_W - 40;
    let th = CONTENT_H - 32;
    let term_bg = rgb(0x08, 0x0C, 0x10);

    // Terminal window chrome
    c.shadow_panel(tx, ty, tw, th, term_bg, rgb(0x22, 0x44, 0x44));

    // Title bar
    c.gradient_h(tx, ty, tw, 28, rgb(0x10, 0x20, 0x20), rgb(0x08, 0x18, 0x18));
    // Traffic-light close/min/max buttons
    c.fill_rounded(tx + 10, ty + 8, 12, 12, palette::ROSE);
    c.fill_rounded(tx + 26, ty + 8, 12, 12, palette::AMBER);
    c.fill_rounded(tx + 42, ty + 8, 12, 12, rgb(0x22,0xC5,0x5E));
    c.centered_text(tx, ty + 6, tw, "bash  —  surendra@oxideos:~", palette::TEXT_DIM);

    // Terminal content
    let lx = tx + 12;
    let mut ly = ty + 36;
    let lh = 18u32;

    let prompt_user = "surendra@oxideos";
    let prompt_path = ":~";
    let prompt_sym  = "$ ";

    // Command 1
    draw_prompt(c, lx, ly, prompt_user, prompt_path, prompt_sym, "uname -a"); ly += lh;
    c.draw_text(lx, ly, "OxideOS 0.1.0-dev x86_64 Rust Kernel #1 Fri May 9 2026", palette::TEXT_DIM); ly += lh;
    ly += 4;

    // Command 2
    draw_prompt(c, lx, ly, prompt_user, prompt_path, prompt_sym, "ls -la /"); ly += lh;
    let entries = [
        ("drwxr-xr-x", "root", "root",   "0", "Jan  1 00:00", ".",          palette::ELECTRIC_BLUE),
        ("drwxr-xr-x", "root", "root",   "0", "Jan  1 00:00", "..",         palette::ELECTRIC_BLUE),
        ("drwxr-xr-x", "root", "root",   "0", "Jan  1 00:00", "bin",        palette::ELECTRIC_BLUE),
        ("drwxr-xr-x", "root", "root",   "0", "Jan  1 00:00", "dev",        palette::ELECTRIC_BLUE),
        ("drwxr-xr-x", "root", "root",   "0", "Jan  1 00:00", "proc",       palette::TEXT_DIM),
        ("drwxrwxrwt", "root", "root",   "0", "Jan  1 00:00", "tmp",        palette::TEAL),
        ("-rwxr-xr-x", "root", "root", "1.2M","Jan  1 00:00", "kernel.elf", palette::AMBER),
    ];
    for &(perms, user, grp, size, date, name, nc) in &entries {
        c.draw_text(lx,        ly, perms, rgb(0x44,0x88,0x44));
        c.draw_text(lx + 104, ly, user, palette::TEXT_DIM);
        c.draw_text(lx + 160, ly, grp,  palette::TEXT_DIM);
        c.draw_text(lx + 216, ly, size, palette::TEXT_DIM);
        c.draw_text(lx + 248, ly, date, palette::TEXT_DIM);
        c.draw_text(lx + 360, ly, name, nc);
        ly += lh;
    }
    ly += 4;

    // Command 3
    draw_prompt(c, lx, ly, prompt_user, prompt_path, prompt_sym, "cat /proc/meminfo"); ly += lh;
    c.draw_text(lx,       ly, "MemTotal:", palette::TEXT_DIM);
    c.draw_text(lx + 100, ly, "131072 kB", palette::CYAN); ly += lh;
    c.draw_text(lx,       ly, "MemFree:",  palette::TEXT_DIM);
    c.draw_text(lx + 100, ly, " 94208 kB", palette::TEAL); ly += lh;
    c.draw_text(lx,       ly, "Cached:",   palette::TEXT_DIM);
    c.draw_text(lx + 100, ly, " 18432 kB", palette::TEXT_DIM); ly += lh;
    ly += 4;

    // Current prompt + blinking cursor
    draw_prompt_only(c, lx, ly, prompt_user, prompt_path, prompt_sym);
    let cursor_x = lx + (prompt_user.len() + prompt_path.len() + prompt_sym.len()) as u32 * 8;
    if (frame / 30) % 2 == 0 {
        c.fill_rect(cursor_x, ly, 8, 16, palette::TEAL);
    }
}

fn draw_prompt(c: &mut Canvas<impl Backend>, x: u32, y: u32,
               user: &str, path: &str, sym: &str, cmd: &str) {
    let mut cx = x;
    c.draw_text(cx, y, user, palette::TEAL);       cx += (user.len() as u32) * 8;
    c.draw_text(cx, y, path, palette::WHITE);       cx += (path.len() as u32) * 8;
    c.draw_text(cx, y, sym,  rgb(0x22,0xC5,0x5E)); cx += (sym.len()  as u32) * 8;
    c.draw_text(cx, y, cmd,  palette::WHITE);
}

fn draw_prompt_only(c: &mut Canvas<impl Backend>, x: u32, y: u32,
                    user: &str, path: &str, sym: &str) {
    let mut cx = x;
    c.draw_text(cx, y, user, palette::TEAL);       cx += (user.len() as u32) * 8;
    c.draw_text(cx, y, path, palette::WHITE);       cx += (path.len() as u32) * 8;
    c.draw_text(cx, y, sym,  rgb(0x22,0xC5,0x5E));
    let _ = cx;
}

// ── Apps tab ──────────────────────────────────────────────────────────────────

fn draw_apps(c: &mut Canvas<impl Backend>, mx: i32, my: i32) {
    let apps: &[(&str, &str, u32, u32)] = &[
        ("Fi", "Files",        palette::TEAL,          palette::ELECTRIC_BLUE),
        ("Tm", "Terminal",     rgb(0x10,0x60,0x28),    rgb(0x22,0xC5,0x5E)),
        ("Br", "Browser",      palette::INDIGO,        palette::ELECTRIC_BLUE),
        ("Ma", "Mail",         palette::DEEP_PURPLE,   palette::PURPLE),
        ("Ca", "Calendar",     rgb(0x60,0x14,0x00),    palette::ORANGE),
        ("Mu", "Music",        rgb(0x50,0x08,0x28),    palette::PINK),
        ("Vi", "Video",        rgb(0x44,0x04,0x04),    palette::ROSE),
        ("Tx", "Text Editor",  rgb(0x20,0x28,0x60),    palette::INDIGO),
        ("Ap", "App Store",    rgb(0x30,0x08,0x50),    palette::PURPLE),
        ("St", "Settings",     rgb(0x50,0x28,0x00),    palette::AMBER),
        ("Sc", "Screenshot",   rgb(0x00,0x38,0x44),    palette::TEAL),
        ("Ab", "About",        rgb(0x10,0x10,0x28),    palette::TEXT_DIM),
    ];

    let tile_w = 180u32;
    let tile_h = 90u32;
    let cols    = 4u32;
    let gap     = 20u32;
    let pad     = 28u32;
    let start_x = CONTENT_X + pad;
    let start_y = CONTENT_Y + pad;

    // Category header
    c.draw_text(start_x, start_y - 4, "All Applications", palette::TEXT_DIM);
    c.hline(start_x, start_y + 14, CONTENT_W - pad * 2, palette::CARD_BORDER);

    let grid_y = start_y + 24;

    for (i, &(icon, name, fl, fr)) in apps.iter().enumerate() {
        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        let ax  = start_x + col * (tile_w + gap);
        let ay  = grid_y  + row * (tile_h + gap + 22);

        let hovered = mx >= ax as i32 && mx < (ax + tile_w) as i32
                   && my >= ay as i32 && my < (ay + tile_h + 20) as i32;

        if hovered {
            c.shadow_panel(ax, ay, tile_w, tile_h, fl, palette::CARD_BORDER);
        } else {
            c.gradient_h(ax, ay, tile_w, tile_h, fl, fr);
            c.draw_rect(ax, ay, tile_w, tile_h, palette::CARD_BORDER);
        }

        // Large icon letter centered
        let icon_size = 40u32;
        let icon_x = ax + (tile_w - icon_size) / 2;
        let icon_y = ay + (tile_h - icon_size) / 2;
        c.fill_rounded(icon_x, icon_y, icon_size, icon_size, rgb(0, 0, 0));
        c.fill_rect(icon_x + 1, icon_y + 1, icon_size - 2, icon_size - 2, rgb(0xFF,0xFF,0xFF));
        // Draw icon letters using the bg gradient color
        c.centered_text(icon_x, icon_y + (icon_size - 16) / 2, icon_size, icon, fl);

        // App name below tile
        c.centered_text(ax, ay + tile_h + 4, tile_w, name,
                        if hovered { palette::TEXT } else { palette::TEXT_DIM });
    }
}

// ── Settings tab ──────────────────────────────────────────────────────────────

fn draw_settings(c: &mut Canvas<impl Backend>, mx: i32, my: i32) {
    let pad  = 20u32;
    let cw   = (CONTENT_W - pad * 3) / 2;
    let cx0  = CONTENT_X + pad;
    let cx1  = cx0 + cw + pad;

    // ── Display card ─────────────────────────────────────────────────────
    let cy = CONTENT_Y + pad;
    let ch = 220u32;
    c.shadow_panel(cx0, cy, cw, ch, palette::CARD_BG, palette::CARD_BORDER);
    c.gradient_h(cx0, cy, cw, 30, palette::INDIGO, palette::ELECTRIC_BLUE);
    c.draw_text(cx0 + 10, cy + 7, "Display", palette::WHITE);

    let kx = cx0 + 16;
    let vx = cx0 + 200;
    let mut ky = cy + 42;
    let row = 28u32;

    let display_opts: &[(&str, &str, bool)] = &[
        ("Resolution",    "1280 x 800",   false),
        ("Refresh rate",  "60 Hz",        false),
        ("Scale factor",  "1.0 x",        false),
        ("Night mode",    "Enabled",      true),
        ("Compositor",    "oxide-wm 0.1", false),
        ("Color profile", "sRGB",         false),
    ];
    for &(k, v, on) in display_opts {
        c.draw_text(kx, ky, k, palette::TEXT_DIM);
        c.draw_text(vx, ky, v, palette::TEXT);
        if on {
            c.fill_rounded(cx0 + cw - 44, ky, 36, 16, palette::TEAL);
            c.fill_rounded(cx0 + cw - 46 + 20, ky + 2, 12, 12, palette::WHITE);
        }
        ky += row;
    }

    // ── Network card ─────────────────────────────────────────────────────
    c.shadow_panel(cx1, cy, cw, ch, palette::CARD_BG, palette::CARD_BORDER);
    c.gradient_h(cx1, cy, cw, 30, rgb(0x00,0x40,0x60), palette::ELECTRIC_BLUE);
    c.draw_text(cx1 + 10, cy + 7, "Network", palette::WHITE);

    let kx2 = cx1 + 16;
    let vx2 = cx1 + 200;
    let mut ky2 = cy + 42;

    let net_opts: &[(&str, &str, bool)] = &[
        ("Interface",  "RTL8139",    false),
        ("IP address", "10.0.2.15",  false),
        ("Gateway",    "10.0.2.2",   false),
        ("DNS",        "8.8.8.8",    false),
        ("Firewall",   "Active",     true),
        ("VPN",        "Disabled",   false),
    ];
    for &(k, v, on) in net_opts {
        c.draw_text(kx2, ky2, k, palette::TEXT_DIM);
        c.draw_text(vx2, ky2, v, palette::TEXT);
        if on {
            c.fill_rounded(cx1 + cw - 44, ky2, 36, 16, palette::TEAL);
            c.fill_rounded(cx1 + cw - 46 + 20, ky2 + 2, 12, 12, palette::WHITE);
        }
        ky2 += row;
    }

    // ── Audio card ───────────────────────────────────────────────────────
    let cy2 = cy + ch + pad;
    let ch2 = 160u32;
    c.shadow_panel(cx0, cy2, cw, ch2, palette::CARD_BG, palette::CARD_BORDER);
    c.gradient_h(cx0, cy2, cw, 30, rgb(0x40,0x08,0x30), palette::PINK);
    c.draw_text(cx0 + 10, cy2 + 7, "Audio", palette::WHITE);

    let mut ay = cy2 + 42;
    let audio_opts: &[(&str, &str, bool)] = &[
        ("Output",   "Built-in speakers", false),
        ("Volume",   "75%",               false),
        ("Mic",      "Enabled",           true),
        ("Bluetooth","Disabled",          false),
    ];
    for &(k, v, on) in audio_opts {
        c.draw_text(kx, ay, k, palette::TEXT_DIM);
        c.draw_text(vx, ay, v, palette::TEXT);
        if on {
            c.fill_rounded(cx0 + cw - 44, ay, 36, 16, palette::TEAL);
            c.fill_rounded(cx0 + cw - 46 + 20, ay + 2, 12, 12, palette::WHITE);
        }
        ay += row;
    }

    // ── Security card ────────────────────────────────────────────────────
    c.shadow_panel(cx1, cy2, cw, ch2, palette::CARD_BG, palette::CARD_BORDER);
    c.gradient_h(cx1, cy2, cw, 30, rgb(0x30,0x10,0x00), palette::ORANGE);
    c.draw_text(cx1 + 10, cy2 + 7, "Security", palette::WHITE);

    let mut sy = cy2 + 42;
    let sec_opts: &[(&str, &str, bool)] = &[
        ("Screen lock",   "After 5 min", false),
        ("Encryption",    "AES-256",     false),
        ("Auto-updates",  "Enabled",     true),
        ("SSH daemon",    "Disabled",    false),
    ];
    for &(k, v, on) in sec_opts {
        c.draw_text(kx2, sy, k, palette::TEXT_DIM);
        c.draw_text(vx2, sy, v, palette::TEXT);
        if on {
            c.fill_rounded(cx1 + cw - 44, sy, 36, 16, palette::TEAL);
            c.fill_rounded(cx1 + cw - 46 + 20, sy + 2, 12, 12, palette::WHITE);
        }
        sy += row;
    }

    // ── Action buttons ───────────────────────────────────────────────────
    let by = cy2 + ch2 + pad;
    let hover_apply = mx >= cx0 as i32 && mx < (cx0 + 140) as i32
                   && my >= by as i32  && my < (by + 36) as i32;
    let hover_rev   = mx >= (cx0 + 152) as i32 && mx < (cx0 + 292) as i32
                   && my >= by as i32  && my < (by + 36) as i32;

    c.gradient_h(cx0, by, 140, 36,
                 if hover_apply { palette::ELECTRIC_BLUE } else { palette::INDIGO },
                 if hover_apply { palette::NEON_CYAN } else { palette::ELECTRIC_BLUE });
    c.draw_rect(cx0, by, 140, 36, palette::CARD_BORDER);
    c.centered_text(cx0, by + 10, 140, "Apply Changes", palette::WHITE);

    c.fill_rect(cx0 + 152, by, 140, 36,
                if hover_rev { rgb(0x28,0x28,0x40) } else { palette::CARD_BG });
    c.draw_rect(cx0 + 152, by, 140, 36, palette::CARD_BORDER);
    c.centered_text(cx0 + 152, by + 10, 140, "Revert", palette::TEXT_DIM);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn real_time<'a>(buf: &'a mut [u8; 8]) -> &'a str {
    use std::time::{SystemTime, UNIX_EPOCH};
    let s = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let h = (s / 3600) % 24;
    let m = (s / 60) % 60;
    let sc = s % 60;
    buf[0] = b'0' + (h  / 10) as u8; buf[1] = b'0' + (h  % 10) as u8;
    buf[2] = b':';
    buf[3] = b'0' + (m  / 10) as u8; buf[4] = b'0' + (m  % 10) as u8;
    buf[5] = b':';
    buf[6] = b'0' + (sc / 10) as u8; buf[7] = b'0' + (sc % 10) as u8;
    std::str::from_utf8(buf).unwrap_or("--:--:--")
}

fn uptime_str(frame: u32) -> &'static str {
    match frame / 3600 {
        0 => "0h 0m",
        1 => "0h 1m",
        _ => "0h 1m+",
    }
}

fn fmt_pct<'a>(buf: &'a mut [u8; 4], v: u32) -> &'a str {
    let v = v.min(100);
    let mut i = buf.len();
    buf[i - 1] = b'%'; i -= 1;
    let mut n = v;
    if n == 0 { i -= 1; buf[i] = b'0'; }
    while n > 0 { i -= 1; buf[i] = b'0' + (n % 10) as u8; n /= 10; }
    std::str::from_utf8(&buf[i..]).unwrap_or("?")
}
