//! OxideOS Desktop — GNOME-inspired widget showcase.
//! Run with:  cargo run --example widgets_demo -p oxide-gui-linux

use oxide_gui_linux::MinifbBackend;
use oxide_gui_core::{Backend, Canvas, Event, Key};
use oxide_gui_core::color::{palette, rgb, lerp_color};

// ── Layout ────────────────────────────────────────────────────────────────────

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
const TAB_W: u32 = 152;

// GNOME accent
const ACCENT: u32 = palette::GNOME_BLUE;

// ── Dock / App data ───────────────────────────────────────────────────────────

const DOCK_ICONS: &[(&str, u32, u8)] = &[
    ("Fi", palette::TEAL,          0),
    ("Tm", rgb(0x22, 0xC5, 0x5E),  1),
    ("Br", palette::ELECTRIC_BLUE, 0),
    ("St", palette::ORANGE,        0),
    ("Ap", palette::PURPLE,        3),
];
const TABS: &[&str] = &["Overview", "Terminal", "Apps", "Settings", "Widgets"];

// ── State ─────────────────────────────────────────────────────────────────────

struct AppState {
    tab:            usize,
    mouse_x:        i32,
    mouse_y:        i32,
    frame:          u32,
    toast_msg:      &'static str,
    toast_until:    u32,
    // Settings toggles: Night Mode, Animations, Night Light, Auto-rotate, File History, Location
    settings:       [bool; 6],
    // Widget-tab interactive toggles
    demo_toggles:   [bool; 4],
    // Apps search query (static for now, shows UI)
    apps_search:    bool, // true = search bar shows as focused
}

impl AppState {
    fn new() -> Self {
        Self {
            tab: 0, mouse_x: 0, mouse_y: 0, frame: 0,
            toast_msg: "", toast_until: 0,
            settings: [true, true, false, false, true, false],
            demo_toggles: [false, true, true, false],
            apps_search: false,
        }
    }
    fn show_toast(&mut self, msg: &'static str) {
        self.toast_msg  = msg;
        self.toast_until = self.frame + 200; // ~3.3 s at 60 fps
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let mut backend = MinifbBackend::new("OxideOS — oxide-gui desktop", W, H)
        .expect("failed to open window");
    let mut st = AppState::new();

    loop {
        if !backend.is_open() { break; }
        st.frame = st.frame.wrapping_add(1);

        {
            let mut c = Canvas::new(&mut backend);
            c.clear(palette::SURFACE);
            draw_topbar(&mut c, &st);
            draw_dock(&mut c, &st);
            draw_tabs(&mut c, &st);

            match st.tab {
                0 => draw_overview(&mut c, &st),
                1 => draw_terminal(&mut c, &st),
                2 => draw_apps(&mut c, &st),
                3 => draw_settings(&mut c, &st),
                4 => draw_widgets(&mut c, &st),
                _ => {}
            }

            draw_statusbar(&mut c);

            // Overlay toast
            if st.frame < st.toast_until {
                let tw = 480u32;
                c.toast((W - tw) / 2, STATUS_Y - 56, tw, st.toast_msg, ACCENT);
            }

            c.present();
        }

        // ── Events ────────────────────────────────────────────────────────
        while let Some(ev) = backend.poll_event() {
            match ev {
                Event::Close => return,
                Event::KeyDown(Key::Char('q')) | Event::KeyDown(Key::Escape) => return,
                Event::KeyDown(Key::Tab) => st.tab = (st.tab + 1) % TABS.len(),
                Event::MouseMove { x, y } => { st.mouse_x = x; st.mouse_y = y; }
                Event::MouseButton { x, y, pressed: true, .. } => {
                    handle_click(&mut st, x, y);
                }
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn handle_click(st: &mut AppState, x: i32, y: i32) {
    // Tab bar clicks
    let rel = x - CONTENT_X as i32;
    if y >= TOPBAR_H as i32 && y < (TOPBAR_H + TAB_H) as i32 && rel >= 0 {
        let ti = rel as usize / TAB_W as usize;
        if ti < TABS.len() { st.tab = ti; return; }
    }

    // Overview quick-action buttons → show toast
    if st.tab == 0 {
        let qy = (CONTENT_Y + 20 + 310 + 20) as i32;
        if y >= qy && y < qy + 38 {
            let lx = (CONTENT_X + 20) as i32;
            let msgs = ["Opened new window", "Opened Terminal", "Opened Files",
                        "Opened Settings", "Suspending..."];
            for (i, &msg) in msgs.iter().enumerate() {
                let bx = lx + i as i32 * 192;
                if x >= bx && x < bx + 180 { st.show_toast(msg); }
            }
        }
    }

    // Settings tab toggle rows
    if st.tab == 3 {
        let card_y = (CONTENT_Y + 110) as i32;
        let cw = ((CONTENT_W - 60) / 2) as i32;
        let lx = (CONTENT_X + 20) as i32;
        let rx = lx + cw + 20;
        // Left column rows 0,1 have toggles
        for (row, tidx) in [(0usize, 0usize), (1, 1)] {
            let ry = card_y + row as i32 * 52;
            if y >= ry && y < ry + 52 && x >= lx && x < lx + cw {
                st.settings[tidx] = !st.settings[tidx];
                st.show_toast(if st.settings[tidx] { "Setting enabled" } else { "Setting disabled" });
            }
        }
        // Right column rows 2,3 have toggles
        for (row, tidx) in [(2usize, 2usize), (3, 3)] {
            let ry = card_y + row as i32 * 52;
            if y >= ry && y < ry + 52 && x >= rx && x < rx + cw {
                st.settings[tidx] = !st.settings[tidx];
                st.show_toast(if st.settings[tidx] { "Setting enabled" } else { "Setting disabled" });
            }
        }
        // Right column second group rows 1,2 → settings[4,5]
        let card2_y = card_y + 4 * 52 + 48;
        for (row, tidx) in [(1usize, 4usize), (2, 5)] {
            let ry = card2_y + row as i32 * 52;
            if y >= ry && y < ry + 52 && x >= rx && x < rx + cw {
                st.settings[tidx] = !st.settings[tidx];
                st.show_toast(if st.settings[tidx] { "Setting enabled" } else { "Setting disabled" });
            }
        }
    }

    // Widgets tab: demo toggle clicks
    if st.tab == 4 {
        let ty = (CONTENT_Y + 80) as i32;
        for i in 0..4usize {
            let tx = (CONTENT_X + 20 + i as u32 * 140) as i32;
            if x >= tx && x < tx + 52 && y >= ty && y < ty + 26 {
                st.demo_toggles[i] = !st.demo_toggles[i];
                st.show_toast(if st.demo_toggles[i] { "Toggle ON" } else { "Toggle OFF" });
            }
        }
        // Apps search bar click
        if x >= (CONTENT_X + 20) as i32 && x < (CONTENT_X + 440) as i32
        && y >= ty + 120 && y < ty + 156 { st.apps_search = !st.apps_search; }
    }
}

// ── Topbar ────────────────────────────────────────────────────────────────────

fn draw_topbar(c: &mut Canvas<impl Backend>, _st: &AppState) {
    c.gradient_h(0, 0, W, TOPBAR_H, rgb(0x0E, 0x0E, 0x1C), rgb(0x16, 0x0A, 0x28));
    c.hline(0, TOPBAR_H - 1, W, rgb(0x30, 0x20, 0x50));

    // Workspace dots
    for (i, &col) in [palette::GNOME_BLUE, palette::PURPLE, palette::TEAL].iter().enumerate() {
        c.fill_round4(12 + i as u32 * 14, 14, 10, 10, col);
    }
    c.draw_text(56, 12, "OxideOS", palette::TEXT);
    c.vline(128, 8, 24, rgb(0x33, 0x22, 0x55));
    c.draw_text(140, 12, "System Monitor", palette::TEXT_DIM);

    // Clock
    let mut tb = [0u8; 8];
    let ts = real_time(&mut tb);
    let tw = (ts.len() as u32) * 8;
    c.draw_text((W - tw) / 2, 12, ts, palette::TEXT);

    // Battery
    let rx = W - 260;
    let batt = 78u32;
    c.draw_rect(rx, 13, 28, 14, palette::TEXT_DIM);
    c.fill_rect(rx + 28, 17, 3, 6, palette::TEXT_DIM);
    c.fill_rect(rx + 1, 14, 26 * batt / 100, 12, palette::TEAL);

    // Avatar
    c.avatar(W - 44, 6, 28, "SU", palette::GNOME_BLUE);
}

// ── Dock ──────────────────────────────────────────────────────────────────────

fn draw_dock(c: &mut Canvas<impl Backend>, st: &AppState) {
    let dy = TOPBAR_H;
    let dh = H - TOPBAR_H - STATUS_H;
    c.gradient_v(0, dy, DOCK_W, dh, rgb(0x10, 0x10, 0x1E), rgb(0x08, 0x08, 0x12));
    c.vline(DOCK_W - 1, dy, dh, rgb(0x20, 0x20, 0x36));

    let active = match st.tab { 1 => 1, 2 => 4, 3 => 3, _ => 0 };

    for (i, &(label, bg, notif)) in DOCK_ICONS.iter().enumerate() {
        let iy = dy + 10 + i as u32 * (ICON_SIZE + 8);
        let hovered = st.mouse_x >= ICON_X as i32
                   && st.mouse_x < (ICON_X + ICON_SIZE) as i32
                   && st.mouse_y >= iy as i32
                   && st.mouse_y < (iy + ICON_SIZE) as i32;
        let tile_bg = if hovered { lerp_color(bg, palette::WHITE, 1, 5) } else { bg };

        c.icon_tile(ICON_X, iy, ICON_SIZE, tile_bg, label);

        if i == active {
            c.accent_bar(0, iy + 4, ICON_SIZE - 8, ACCENT);
        } else if hovered {
            c.accent_bar(0, iy + 12, ICON_SIZE - 24, palette::TEXT_DIM);
        }

        if notif > 0 {
            let bx = ICON_X + ICON_SIZE - 12;
            let by = iy + 1;
            c.fill_round4(bx, by, 12, 12, palette::ROSE);
            let s = ["0","1","2","3","4","5","6","7","8","9"][notif as usize % 10];
            c.centered_text(bx, by + 1, 12, s, palette::WHITE);
        }
    }

    let sep_y = dy + 10 + DOCK_ICONS.len() as u32 * (ICON_SIZE + 8) + 4;
    c.hline(8, sep_y, DOCK_W - 16, rgb(0x22, 0x22, 0x38));

    let py = dy + dh - ICON_SIZE - 10;
    let ph = st.mouse_x >= ICON_X as i32 && st.mouse_x < (ICON_X + ICON_SIZE) as i32
          && st.mouse_y >= py as i32 && st.mouse_y < (py + ICON_SIZE) as i32;
    c.icon_tile(ICON_X, py, ICON_SIZE, if ph { palette::ROSE } else { rgb(0x35, 0x10, 0x18) }, "Pw");
}

// ── Tab bar ───────────────────────────────────────────────────────────────────

fn draw_tabs(c: &mut Canvas<impl Backend>, st: &AppState) {
    let ty = TOPBAR_H;
    c.fill_rect(CONTENT_X, ty, CONTENT_W, TAB_H, palette::SURFACE2);
    c.hline(CONTENT_X, ty + TAB_H - 1, CONTENT_W, rgb(0x20, 0x20, 0x36));

    for (i, &tab) in TABS.iter().enumerate() {
        let tx  = CONTENT_X + i as u32 * TAB_W;
        let sel = i == st.tab;
        let hov = !sel
               && st.mouse_x >= tx as i32 && st.mouse_x < (tx + TAB_W) as i32
               && st.mouse_y >= ty as i32  && st.mouse_y < (ty + TAB_H) as i32;

        let bg = if sel { palette::SURFACE } else if hov { rgb(0x1A, 0x1A, 0x2C) } else { palette::SURFACE2 };
        c.fill_rect(tx, ty, TAB_W, TAB_H, bg);
        c.centered_text(tx, ty + (TAB_H - 16) / 2, TAB_W, tab,
                        if sel { palette::TEXT } else { palette::TEXT_DIM });
        if sel {
            c.gradient_h(tx, ty + TAB_H - 3, TAB_W, 3, ACCENT, palette::ELECTRIC_BLUE);
        }
    }
}

// ── Status bar ────────────────────────────────────────────────────────────────

fn draw_statusbar(c: &mut Canvas<impl Backend>) {
    c.gradient_h(0, STATUS_Y, W, STATUS_H, rgb(0x10, 0x06, 0x20), rgb(0x06, 0x10, 0x20));
    c.hline(0, STATUS_Y, W, rgb(0x28, 0x18, 0x44));
    c.draw_text(14, STATUS_Y + 4,
        "oxide-gui v0.1.0   Tab: next tab   Q: quit   Click toggles/buttons to interact",
        palette::TEXT_DIM);
    c.right_text(0, STATUS_Y + 4, W - 12, "OxideOS 0.1.0-dev", rgb(0x44, 0x33, 0x66));
}

// ── Overview tab ──────────────────────────────────────────────────────────────

fn draw_overview(c: &mut Canvas<impl Backend>, st: &AppState) {
    let t   = st.frame as f32;
    let cpu = (50.0 + 20.0 * (t * 0.04).sin()) as u32;
    let ram = (30.0 +  8.0 * (t * 0.02).sin()) as u32;
    let gpu = (60.0 + 22.0 * (t * 0.07).sin()) as u32;
    let net = (25.0 + 25.0 * (t * 0.12).sin().abs()) as u32;

    let pad = 20u32;
    let cw  = (CONTENT_W - pad * 3) / 2;
    let cx0 = CONTENT_X + pad;
    let cx1 = cx0 + cw + pad;
    let cy  = CONTENT_Y + pad;
    let ch  = 310u32;

    // Left: System Info
    c.shadow_panel(cx0, cy, cw, ch, palette::CARD_BG, palette::CARD_BORDER);
    c.gradient_h(cx0, cy, cw, 34, palette::DEEP_PURPLE, palette::PURPLE);
    c.draw_text(cx0 + 10, cy + 9, "System Overview", palette::WHITE);

    let kx = cx0 + 14; let vx = cx0 + 170; let mut ky = cy + 46;
    let info = [
        ("OS",       "OxideOS 0.1.0-dev (x86_64)"),
        ("Kernel",   "Rust no_std monolithic"),
        ("GUI lib",  "oxide-gui-core v0.1.0"),
        ("Backend",  "oxide-gui-linux (minifb)"),
        ("Display",  "1280 x 800  60 Hz"),
        ("Memory",   "128 MB total"),
        ("Free mem", "94 MB available"),
        ("Uptime",   uptime_str(st.frame)),
        ("Sessions", "1 active"),
        ("Users",    "surendra (active)"),
    ];
    for &(k, v) in &info {
        c.draw_text(kx, ky, k, palette::TEXT_DIM);
        c.draw_text(vx, ky, v, palette::TEXT);
        ky += 24;
    }

    // Right: Resource Monitor
    c.shadow_panel(cx1, cy, cw, ch, palette::CARD_BG, palette::CARD_BORDER);
    c.gradient_h(cx1, cy, cw, 34, palette::TEAL, palette::ELECTRIC_BLUE);
    c.draw_text(cx1 + 10, cy + 9, "Resource Monitor", rgb(0x08, 0x08, 0x12));

    let bx = cx1 + 14; let bw = cw - 90; let bh = 16u32;
    let mut by = cy + 52;
    for &(label, pct, fl, fr) in &[
        ("CPU", cpu, palette::INDIGO,       palette::ELECTRIC_BLUE),
        ("RAM", ram, palette::DEEP_PURPLE,  palette::PURPLE),
        ("GPU", gpu, palette::ORANGE,       palette::AMBER),
        ("Net", net, rgb(0x00,0x50,0x38),   palette::TEAL),
    ] {
        c.draw_text(bx, by, label, palette::TEXT_DIM);
        c.gradient_progress(bx, by + 18, bw, bh, pct,
                            rgb(0x08,0x08,0x14), fl, fr, palette::CARD_BORDER);
        let mut nb = [0u8; 4];
        c.draw_text(bx + bw + 6, by + 18, fmt_pct(&mut nb, pct), palette::TEXT_DIM);
        by += 52;
    }
    // CPU mini-history chart
    c.draw_text(bx, by + 4, "CPU history", palette::TEXT_DIM); by += 22;
    for i in 0..40u32 {
        let s_t = (st.frame as f32 * 0.04) - (40 - i) as f32 * 0.08;
        let pct = ((50.0 + 20.0 * s_t.sin()) as u32).min(bh * 2);
        let bh2 = (pct * (bh * 2) / 100).max(2);
        c.fill_rect(bx + i * 7, by + bh * 2 - bh2, 5, bh2,
                    lerp_color(palette::INDIGO, palette::ELECTRIC_BLUE, i, 39));
    }

    // Quick-action buttons
    let qy = cy + ch + pad;
    for (i, &(label, fl, fr)) in [
        ("New Window", palette::INDIGO,          palette::ELECTRIC_BLUE),
        ("Terminal",   rgb(0x06,0x56,0x2A),      palette::TEAL),
        ("Files",      rgb(0x00,0x44,0x4A),      palette::CYAN),
        ("Settings",   rgb(0x54,0x28,0x00),      palette::ORANGE),
        ("Suspend",    rgb(0x38,0x06,0x14),       palette::ROSE),
    ].iter().enumerate() {
        let bx2 = cx0 + i as u32 * 192;
        let hov  = st.mouse_x >= bx2 as i32 && st.mouse_x < (bx2 + 180) as i32
                && st.mouse_y >= qy as i32    && st.mouse_y < (qy + 38) as i32;
        let l = if hov { lerp_color(fl, fr, 1, 3) } else { fl };
        c.gradient_h(bx2, qy, 180, 38, l, fr);
        c.draw_rect(bx2, qy, 180, 38, palette::CARD_BORDER);
        c.centered_text(bx2, qy + 11, 180, label, palette::WHITE);
    }
}

// ── Terminal tab ──────────────────────────────────────────────────────────────

fn draw_terminal(c: &mut Canvas<impl Backend>, st: &AppState) {
    let tx = CONTENT_X + 20; let ty = CONTENT_Y + 16;
    let tw = CONTENT_W - 40; let th = CONTENT_H - 32;
    let term_bg = rgb(0x08, 0x0C, 0x10);

    c.shadow_panel(tx, ty, tw, th, term_bg, rgb(0x20, 0x44, 0x44));
    // GNOME-style headerbar for terminal window
    c.gnome_headerbar(tx, ty, tw, "bash  —  surendra@oxideos:~", false, rgb(0x12, 0x20, 0x20));

    let lx = tx + 14; let mut ly = ty + 56; let lh = 18u32;
    let pu = "surendra@oxideos"; let pp = ":~"; let ps = "$ ";

    draw_prompt(c, lx, ly, pu, pp, ps, "uname -a"); ly += lh;
    c.draw_text(lx, ly, "OxideOS 0.1.0-dev x86_64 Rust Kernel #1 Sat May 10 2026",
                palette::TEXT_DIM); ly += lh + 4;

    draw_prompt(c, lx, ly, pu, pp, ps, "ls -la /"); ly += lh;
    for &(perm, usr, grp, sz, dt, name, nc) in &[
        ("drwxr-xr-x","root","root","0","Jan  1 00:00",".",         palette::ELECTRIC_BLUE),
        ("drwxr-xr-x","root","root","0","Jan  1 00:00","..",        palette::ELECTRIC_BLUE),
        ("drwxr-xr-x","root","root","0","Jan  1 00:00","bin",       palette::ELECTRIC_BLUE),
        ("drwxr-xr-x","root","root","0","Jan  1 00:00","dev",       palette::ELECTRIC_BLUE),
        ("drwxr-xr-x","root","root","0","Jan  1 00:00","proc",      palette::TEXT_DIM),
        ("drwxrwxrwt","root","root","0","Jan  1 00:00","tmp",       palette::TEAL),
        ("-rwxr-xr-x","root","root","1.2M","Jan 1 00:00","kernel.elf",palette::AMBER),
    ] {
        c.draw_text(lx,        ly, perm, rgb(0x40,0x88,0x40));
        c.draw_text(lx + 104, ly, usr,  palette::TEXT_DIM);
        c.draw_text(lx + 160, ly, grp,  palette::TEXT_DIM);
        c.draw_text(lx + 216, ly, sz,   palette::TEXT_DIM);
        c.draw_text(lx + 256, ly, dt,   palette::TEXT_DIM);
        c.draw_text(lx + 368, ly, name, nc);
        ly += lh;
    }
    ly += 4;

    draw_prompt(c, lx, ly, pu, pp, ps, "cat /proc/meminfo"); ly += lh;
    for &(k, v, vc) in &[
        ("MemTotal:","131072 kB", palette::CYAN),
        ("MemFree:", " 94208 kB", palette::TEAL),
        ("Cached:  "," 18432 kB", palette::TEXT_DIM),
    ] {
        c.draw_text(lx, ly, k, palette::TEXT_DIM);
        c.draw_text(lx + 104, ly, v, vc);
        ly += lh;
    }
    ly += 4;

    // Current prompt + blinking cursor
    draw_prompt_only(c, lx, ly, pu, pp, ps);
    let cur_x = lx + (pu.len() + pp.len() + ps.len()) as u32 * 8;
    if (st.frame / 30) % 2 == 0 {
        c.fill_rect(cur_x, ly, 8, 16, palette::TEAL);
    }
}

fn draw_prompt(c: &mut Canvas<impl Backend>, x: u32, y: u32,
               user: &str, path: &str, sym: &str, cmd: &str) {
    let mut cx = x;
    c.draw_text(cx, y, user, palette::TEAL);        cx += (user.len() as u32) * 8;
    c.draw_text(cx, y, path, palette::WHITE);        cx += (path.len() as u32) * 8;
    c.draw_text(cx, y, sym,  rgb(0x22,0xC5,0x5E));  cx += (sym.len()  as u32) * 8;
    c.draw_text(cx, y, cmd,  palette::WHITE);
}
fn draw_prompt_only(c: &mut Canvas<impl Backend>, x: u32, y: u32,
                    user: &str, path: &str, sym: &str) {
    let mut cx = x;
    c.draw_text(cx, y, user, palette::TEAL);        cx += (user.len() as u32) * 8;
    c.draw_text(cx, y, path, palette::WHITE);        cx += (path.len() as u32) * 8;
    c.draw_text(cx, y, sym,  rgb(0x22,0xC5,0x5E));
    let _ = cx;
}

// ── Apps tab ──────────────────────────────────────────────────────────────────

fn draw_apps(c: &mut Canvas<impl Backend>, st: &AppState) {
    let pad = 20u32;
    let sx  = CONTENT_X + pad;
    let _sy = CONTENT_Y + pad;

    // GNOME-style headerbar for the apps view
    c.gnome_headerbar(CONTENT_X, CONTENT_Y, CONTENT_W,
                      "All Applications", false, rgb(0x10, 0x10, 0x1E));

    // Search bar
    let hbar_h = 48u32;
    c.search_bar(sx, CONTENT_Y + hbar_h + 8, 440, "", st.apps_search, ACCENT);
    c.draw_text(sx + 460, CONTENT_Y + hbar_h + 18, "(click to toggle focus)", palette::TEXT_DIM);

    let apps: &[(&str, &str, u32, u32)] = &[
        ("Fi","Files",        palette::TEAL,         palette::ELECTRIC_BLUE),
        ("Tm","Terminal",     rgb(0x10,0x58,0x28),   rgb(0x22,0xC5,0x5E)),
        ("Br","Browser",      palette::INDIGO,       palette::ELECTRIC_BLUE),
        ("Ma","Mail",         palette::DEEP_PURPLE,  palette::PURPLE),
        ("Ca","Calendar",     rgb(0x58,0x14,0x00),   palette::ORANGE),
        ("Mu","Music",        rgb(0x48,0x08,0x24),   palette::PINK),
        ("Vi","Video",        rgb(0x40,0x04,0x04),   palette::ROSE),
        ("Tx","Text Editor",  rgb(0x18,0x20,0x58),   palette::INDIGO),
        ("Ap","App Store",    rgb(0x28,0x08,0x48),   palette::PURPLE),
        ("St","Settings",     rgb(0x48,0x24,0x00),   palette::AMBER),
        ("Sc","Screenshot",   rgb(0x00,0x34,0x40),   palette::TEAL),
        ("Ab","About",        rgb(0x10,0x10,0x28),   palette::TEXT_DIM),
    ];

    let tile_w = 176u32; let tile_h = 88u32; let cols = 4u32; let gap = 18u32;
    let grid_y = CONTENT_Y + hbar_h + 54;

    c.draw_text(sx, grid_y - 14, "Installed  (12)", palette::TEXT_DIM);

    for (i, &(icon, name, fl, fr)) in apps.iter().enumerate() {
        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        let ax  = sx + col * (tile_w + gap);
        let ay  = grid_y + row * (tile_h + gap + 20);
        let hov = st.mouse_x >= ax as i32 && st.mouse_x < (ax + tile_w) as i32
               && st.mouse_y >= ay as i32 && st.mouse_y < (ay + tile_h + 18) as i32;

        if hov {
            c.shadow_panel(ax, ay, tile_w, tile_h, fl, palette::CARD_BORDER);
        } else {
            c.gradient_h(ax, ay, tile_w, tile_h, fl, fr);
            c.draw_rect(ax, ay, tile_w, tile_h, palette::CARD_BORDER);
        }
        // Icon letter on white circle
        let is = 36u32;
        let ix = ax + (tile_w - is) / 2; let iy = ay + (tile_h - is) / 2;
        c.fill_round4(ix - 2, iy - 2, is + 4, is + 4, rgb(0,0,0));
        c.fill_round4(ix, iy, is, is, palette::WHITE);
        c.centered_text(ix, iy + (is - 16) / 2, is, icon, fl);
        // Name
        c.centered_text(ax, ay + tile_h + 4, tile_w, name,
                        if hov { palette::TEXT } else { palette::TEXT_DIM });
    }
}

// ── Settings tab ─────────────────────────────────────────────────────────────

fn draw_settings(c: &mut Canvas<impl Backend>, st: &AppState) {
    let pad = 20u32;
    let cw  = (CONTENT_W - pad * 3) / 2;
    let lx  = CONTENT_X + pad;
    let rx  = lx + cw + pad;

    // Page title + search
    c.draw_text(lx, CONTENT_Y + 10, "Settings", palette::TEXT);
    c.search_bar(lx + 120, CONTENT_Y + 6, 380, "", false, ACCENT);

    let card_y  = CONTENT_Y + 110;

    // ── Left column: Appearance group ─────────────────────────────────────
    c.draw_text(lx, card_y - 22, "Appearance", palette::TEXT_DIM);
    c.fill_round4(lx, card_y, cw, 4 * 52, palette::CARD_BG);
    c.draw_rect(lx, card_y, cw, 4 * 52, palette::CARD_BORDER);

    let hov = |row: u32| -> bool {
        st.mouse_y >= (card_y + row * 52) as i32
     && st.mouse_y <  (card_y + row * 52 + 52) as i32
     && st.mouse_x >= lx as i32 && st.mouse_x < (lx + cw) as i32
    };
    let mut ry = card_y;
    ry += c.action_row_toggle(lx, ry, cw, "Night Mode",
        "Use dark color scheme", st.settings[0], ACCENT, hov(0), false);
    ry += c.action_row_toggle(lx, ry, cw, "Animations",
        "Smooth interface transitions", st.settings[1], ACCENT, hov(1), false);
    ry += c.action_row(lx, ry, cw, "Color Profile",
        "Display gamut", "sRGB", hov(2), false);
    c.action_row(lx, ry, cw, "Accent Color",
        "Button and highlight color", "GNOME Blue", hov(3), true);

    // ── Left column: Fonts group ──────────────────────────────────────────
    let fonts_y = card_y + 4 * 52 + 28;
    c.draw_text(lx, fonts_y - 22, "Fonts", palette::TEXT_DIM);
    c.fill_round4(lx, fonts_y, cw, 2 * 52, palette::CARD_BG);
    c.draw_rect(lx, fonts_y, cw, 2 * 52, palette::CARD_BORDER);
    let fhov = |row: u32| -> bool {
        st.mouse_y >= (fonts_y + row * 52) as i32
     && st.mouse_y <  (fonts_y + row * 52 + 52) as i32
     && st.mouse_x >= lx as i32 && st.mouse_x < (lx + cw) as i32
    };
    c.action_row_nav(lx, fonts_y,        cw, "Document Font",  "Cantarell 11", fhov(0), false);
    c.action_row_nav(lx, fonts_y + 52,   cw, "Monospace Font", "Monospace 11", fhov(1), true);

    // ── Right column: Display group ───────────────────────────────────────
    c.draw_text(rx, card_y - 22, "Display", palette::TEXT_DIM);
    c.fill_round4(rx, card_y, cw, 4 * 52, palette::CARD_BG);
    c.draw_rect(rx, card_y, cw, 4 * 52, palette::CARD_BORDER);
    let rhov = |row: u32| -> bool {
        st.mouse_y >= (card_y + row * 52) as i32
     && st.mouse_y <  (card_y + row * 52 + 52) as i32
     && st.mouse_x >= rx as i32 && st.mouse_x < (rx + cw) as i32
    };
    let mut rry = card_y;
    rry += c.action_row(rx, rry, cw, "Resolution", "Screen size in pixels", "1280 × 800", rhov(0), false);
    rry += c.action_row(rx, rry, cw, "Refresh Rate", "Display update frequency", "60 Hz", rhov(1), false);
    rry += c.action_row_toggle(rx, rry, cw, "Night Light",
        "Reduce blue light after sunset", st.settings[2], palette::AMBER, rhov(2), false);
    c.action_row_toggle(rx, rry, cw, "Auto-rotate",
        "Rotate display with device", st.settings[3], ACCENT, rhov(3), true);

    // ── Right column: Privacy group ───────────────────────────────────────
    let priv_y = fonts_y;
    c.draw_text(rx, priv_y - 22, "Privacy", palette::TEXT_DIM);
    c.fill_round4(rx, priv_y, cw, 3 * 52, palette::CARD_BG);
    c.draw_rect(rx, priv_y, cw, 3 * 52, palette::CARD_BORDER);
    let phov = |row: u32| -> bool {
        st.mouse_y >= (priv_y + row * 52) as i32
     && st.mouse_y <  (priv_y + row * 52 + 52) as i32
     && st.mouse_x >= rx as i32 && st.mouse_x < (rx + cw) as i32
    };
    c.action_row_nav(rx, priv_y,        cw, "Screen Lock", "Lock after 5 minutes", phov(0), false);
    c.action_row_toggle(rx, priv_y + 52,  cw, "File History",
        "Remember recently used files", st.settings[4], ACCENT, phov(1), false);
    c.action_row_toggle(rx, priv_y + 104, cw, "Location Services",
        "Allow apps to request location", st.settings[5], ACCENT, phov(2), true);

    // Apply / Revert
    let by = priv_y + 3 * 52 + 16;
    let happ = st.mouse_x >= lx as i32 && st.mouse_x < (lx + 160) as i32
            && st.mouse_y >= by as i32  && st.mouse_y < (by + 36) as i32;
    let href = st.mouse_x >= (lx + 172) as i32 && st.mouse_x < (lx + 332) as i32
            && st.mouse_y >= by as i32           && st.mouse_y < (by + 36) as i32;
    c.gradient_h(lx, by, 160, 36,
        if happ { palette::ELECTRIC_BLUE } else { ACCENT },
        if happ { palette::NEON_CYAN } else { palette::ELECTRIC_BLUE });
    c.draw_rect(lx, by, 160, 36, palette::CARD_BORDER);
    c.centered_text(lx, by + 10, 160, "Apply Changes", palette::WHITE);
    c.fill_rect(lx + 172, by, 160, 36,
        if href { rgb(0x28,0x28,0x40) } else { palette::CARD_BG });
    c.draw_rect(lx + 172, by, 160, 36, palette::CARD_BORDER);
    c.centered_text(lx + 172, by + 10, 160, "Revert", palette::TEXT_DIM);
}

// ── Widgets gallery tab ───────────────────────────────────────────────────────

fn draw_widgets(c: &mut Canvas<impl Backend>, st: &AppState) {
    let lx  = CONTENT_X + 24;
    let rx  = CONTENT_X + CONTENT_W / 2 + 12;
    let col_w = CONTENT_W / 2 - 36;
    let mut ly = CONTENT_Y + 16;
    let mut ry = CONTENT_Y + 16;

    // ── LEFT COLUMN ───────────────────────────────────────────────────────

    // Toggle Switches
    c.draw_text(lx, ly, "Toggle Switches", palette::TEXT_DIM); ly += 22;
    c.fill_round4(lx, ly, col_w, 52, palette::CARD_BG);
    c.draw_rect(lx, ly, col_w, 52, palette::CARD_BORDER);
    let accents = [palette::TEXT_DIM, ACCENT, palette::TEAL, palette::ROSE];
    let labels  = ["Off", "Blue", "Teal", "Rose"];
    for (i, (&acc, &lbl)) in accents.iter().zip(labels.iter()).enumerate() {
        let tx = lx + 16 + i as u32 * 140;
        c.toggle_switch(tx, ly + 13, st.demo_toggles[i], acc);
        c.draw_text(tx + 60, ly + 18, lbl, palette::TEXT_DIM);
    }
    c.draw_text(lx + 8, ly + 40, "Click to toggle", palette::TEXT_DIM);
    ly += 72;

    // Chips / Badges
    c.draw_text(lx, ly, "Chips & Badges", palette::TEXT_DIM); ly += 22;
    c.fill_round4(lx, ly, col_w, 38, palette::CARD_BG);
    c.draw_rect(lx, ly, col_w, 38, palette::CARD_BORDER);
    let chips: &[(&str, u32, u32)] = &[
        ("Active",  ACCENT,          palette::WHITE),
        ("Pending", palette::AMBER,  rgb(0x10,0x10,0x10)),
        ("Error",   palette::ROSE,   palette::WHITE),
        ("Success", palette::TEAL,   palette::WHITE),
        ("12",      palette::PURPLE, palette::WHITE),
        ("New",     palette::ORANGE, palette::WHITE),
    ];
    let mut cx2 = lx + 10;
    for &(text, bg, fg) in chips {
        c.chip(cx2, ly + 8, text, bg, fg);
        cx2 += (text.len() as u32) * 8 + 28;
    }
    ly += 58;

    // Search Entry
    c.draw_text(lx, ly, "Search Entry", palette::TEXT_DIM); ly += 22;
    c.search_bar(lx, ly,      col_w, "",          false, ACCENT); ly += 46;
    c.search_bar(lx, ly,      col_w, "oxide-gui", true,  ACCENT); ly += 46;

    // Spinners
    c.draw_text(lx, ly, "Spinner (animated)", palette::TEXT_DIM); ly += 28;
    c.fill_round4(lx, ly, col_w, 60, palette::CARD_BG);
    c.draw_rect(lx, ly, col_w, 60, palette::CARD_BORDER);
    c.spinner(lx + 30,  ly + 30, st.frame, ACCENT);
    c.spinner(lx + 80,  ly + 30, st.frame, palette::TEAL);
    c.spinner(lx + 130, ly + 30, st.frame, palette::PURPLE);
    c.spinner(lx + 180, ly + 30, st.frame + 16, palette::ROSE);
    c.draw_text(lx + 210, ly + 22, "Multiple independent spinners", palette::TEXT_DIM);

    // ── RIGHT COLUMN ─────────────────────────────────────────────────────

    // AdwPreferencesGroup example
    c.draw_text(rx, ry, "AdwPreferencesGroup", palette::TEXT_DIM); ry += 22;
    let group_rows = 5u32;
    c.fill_round4(rx, ry, col_w, group_rows * 52, palette::CARD_BG);
    c.draw_rect(rx, ry, col_w, group_rows * 52, palette::CARD_BORDER);

    let ghov = |row: u32| -> bool {
        st.mouse_y >= (ry + row * 52) as i32
     && st.mouse_y <  (ry + row * 52 + 52) as i32
     && st.mouse_x >= rx as i32 && st.mouse_x < (rx + col_w) as i32
    };
    let mut grow = ry;
    grow += c.action_row_toggle(rx, grow, col_w, "Night Mode",
        "Use dark color scheme", true, ACCENT, ghov(0), false);
    grow += c.action_row(rx, grow, col_w, "Color Profile",
        "Display gamut mapping", "sRGB", ghov(1), false);
    grow += c.action_row_toggle(rx, grow, col_w, "Animations",
        "Smooth interface transitions", false, ACCENT, ghov(2), false);
    grow += c.action_row_nav(rx, grow, col_w, "Keyboard Shortcuts",
        "View and edit key bindings", ghov(3), false);
    c.action_row_nav(rx, grow, col_w, "Extensions",
        "Manage installed extensions", ghov(4), true);
    ry += group_rows * 52 + 24;

    // Avatars
    c.draw_text(rx, ry, "AdwAvatar", palette::TEXT_DIM); ry += 22;
    c.fill_round4(rx, ry, col_w, 72, palette::CARD_BG);
    c.draw_rect(rx, ry, col_w, 72, palette::CARD_BORDER);
    let avatars: &[(&str, u32)] = &[
        ("SU", ACCENT),
        ("JD", palette::TEAL),
        ("MK", palette::PURPLE),
        ("AW", palette::ORANGE),
        ("RB", palette::ROSE),
    ];
    for (i, &(init, col)) in avatars.iter().enumerate() {
        c.avatar(rx + 14 + i as u32 * 56, ry + 12, 48, init, col);
    }
    ry += 92;

    // Toast notification
    c.draw_text(rx, ry, "Toast Notification", palette::TEXT_DIM); ry += 22;
    c.toast(rx, ry, col_w, "Application installed successfully", palette::TEAL);
    ry += 60;
    c.toast(rx, ry, col_w, "Firmware update requires restart", palette::AMBER);
    ry += 60;
    c.toast(rx, ry, col_w, "Connection failed — check network", palette::ROSE);
    ry += 60;

    // AdwHeaderBar example
    c.draw_text(rx, ry, "AdwHeaderBar", palette::TEXT_DIM); ry += 22;
    c.fill_round4(rx, ry, col_w, 96, rgb(0x12, 0x12, 0x20));
    c.draw_rect(rx, ry, col_w, 96, palette::CARD_BORDER);
    c.gnome_headerbar(rx + 1, ry + 4, col_w - 2, "Preferences", true, rgb(0x1A, 0x1A, 0x2E));
    // Show a single action row inside the fake window
    c.action_row(rx + 1, ry + 52, col_w - 2,
        "Dark Mode", "Switch appearance", "On", false, true);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn real_time<'a>(buf: &'a mut [u8; 8]) -> &'a str {
    use std::time::{SystemTime, UNIX_EPOCH};
    let s  = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let h  = (s / 3600) % 24;
    let m  = (s / 60) % 60;
    let sc = s % 60;
    buf[0] = b'0' + (h  / 10) as u8; buf[1] = b'0' + (h  % 10) as u8;
    buf[2] = b':';
    buf[3] = b'0' + (m  / 10) as u8; buf[4] = b'0' + (m  % 10) as u8;
    buf[5] = b':';
    buf[6] = b'0' + (sc / 10) as u8; buf[7] = b'0' + (sc % 10) as u8;
    std::str::from_utf8(buf).unwrap_or("--:--:--")
}

fn uptime_str(frame: u32) -> &'static str {
    match frame / 3600 { 0 => "0h 0m", 1 => "0h 1m", _ => "0h 1m+" }
}

fn fmt_pct<'a>(buf: &'a mut [u8; 4], v: u32) -> &'a str {
    let v  = v.min(100);
    let mut i = buf.len();
    buf[i - 1] = b'%'; i -= 1;
    let mut n  = v;
    if n == 0 { i -= 1; buf[i] = b'0'; }
    while n > 0 { i -= 1; buf[i] = b'0' + (n % 10) as u8; n /= 10; }
    std::str::from_utf8(&buf[i..]).unwrap_or("?")
}
