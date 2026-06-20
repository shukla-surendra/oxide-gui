//! Color type and helpers.
//!
//! All colors are stored as `0xAA_RR_GG_BB` (alpha, red, green, blue),
//! matching OxideOS's native framebuffer format.  The alpha channel is
//! treated as fully-opaque (0xFF) for all blending in the default backends.

/// ARGB color value: `0xAA_RR_GG_BB`.
pub type Color = u32;

/// Construct a fully-opaque color from red, green, blue byte components.
#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Construct a color with an explicit alpha channel.
#[inline]
pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Color {
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Extract the red component.
#[inline] pub const fn red(c: Color)   -> u8 { ((c >> 16) & 0xFF) as u8 }
/// Extract the green component.
#[inline] pub const fn green(c: Color) -> u8 { ((c >>  8) & 0xFF) as u8 }
/// Extract the blue component.
#[inline] pub const fn blue(c: Color)  -> u8 { (c & 0xFF) as u8 }
/// Extract the alpha component.
#[inline] pub const fn alpha(c: Color) -> u8 { ((c >> 24) & 0xFF) as u8 }

/// Strip alpha and return the color in `0x00_RR_GG_BB` format (for minifb).
#[inline] pub const fn to_rgb24(c: Color) -> u32 { c & 0x00FFFFFF }

/// Linearly interpolate between two colors.  `t` in `0..=steps`.
#[inline]
pub fn lerp_color(a: Color, b: Color, t: u32, steps: u32) -> Color {
    if steps == 0 { return a; }
    let ar = ((a >> 16) & 0xFF) as u32;
    let ag = ((a >>  8) & 0xFF) as u32;
    let ab = ( a        & 0xFF) as u32;
    let br = ((b >> 16) & 0xFF) as u32;
    let bg = ((b >>  8) & 0xFF) as u32;
    let bb = ( b        & 0xFF) as u32;
    let r  = if br >= ar { ar + (br - ar) * t / steps } else { ar - (ar - br) * t / steps };
    let g  = if bg >= ag { ag + (bg - ag) * t / steps } else { ag - (ag - bg) * t / steps };
    let b2 = if bb >= ab { ab + (bb - ab) * t / steps } else { ab - (ab - bb) * t / steps };
    0xFF000000 | (r << 16) | (g << 8) | b2
}

// ── Named palette ─────────────────────────────────────────────────────────────

pub mod palette {
    use super::rgb;

    // ── Neutrals ──
    pub const BLACK:          u32 = rgb(0x00, 0x00, 0x00);
    pub const WHITE:          u32 = rgb(0xFF, 0xFF, 0xFF);
    pub const DARK_GRAY:      u32 = rgb(0x1E, 0x1E, 0x1E);
    pub const GRAY:           u32 = rgb(0x52, 0x52, 0x52);
    pub const LIGHT_GRAY:     u32 = rgb(0xCC, 0xCC, 0xCC);

    // ── Standard ──
    pub const RED:            u32 = rgb(0xF1, 0x4C, 0x4C);
    pub const GREEN:          u32 = rgb(0x40, 0xC0, 0x40);
    pub const BLUE:           u32 = rgb(0x00, 0x7A, 0xCC);
    pub const YELLOW:         u32 = rgb(0xFF, 0xD7, 0x00);
    pub const CYAN:           u32 = rgb(0x4E, 0xC9, 0xB0);
    pub const ACCENT:         u32 = rgb(0x4E, 0xC9, 0xB0);

    // ── Vibrant accents ──
    pub const PURPLE:         u32 = rgb(0x8B, 0x5C, 0xF6);
    pub const DEEP_PURPLE:    u32 = rgb(0x4C, 0x1D, 0x95);
    pub const PINK:           u32 = rgb(0xEC, 0x4E, 0x99);
    pub const ROSE:           u32 = rgb(0xFF, 0x40, 0x81);
    pub const ORANGE:         u32 = rgb(0xFB, 0x8C, 0x00);
    pub const AMBER:          u32 = rgb(0xFF, 0xCA, 0x28);
    pub const TEAL:           u32 = rgb(0x00, 0xBF, 0xA5);
    pub const INDIGO:         u32 = rgb(0x3D, 0x5A, 0xFE);
    pub const NEON_CYAN:      u32 = rgb(0x00, 0xFF, 0xE5);
    pub const ELECTRIC_BLUE:  u32 = rgb(0x00, 0xB4, 0xFF);
    pub const GNOME_BLUE:     u32 = rgb(0x35, 0x84, 0xE4);

    // ── Deep-space surfaces ──
    pub const SURFACE:        u32 = rgb(0x0D, 0x0D, 0x16);
    pub const SURFACE2:       u32 = rgb(0x13, 0x13, 0x22);
    pub const CARD_BG:        u32 = rgb(0x1A, 0x1A, 0x2E);
    pub const CARD_BORDER:    u32 = rgb(0x2C, 0x2C, 0x4A);

    // ── Legacy theme aliases (kept for backwards compat) ──
    pub const DARK_BG:        u32 = rgb(0x1E, 0x1E, 0x1E);
    pub const PANEL_BG:       u32 = rgb(0x25, 0x25, 0x26);
    pub const TOOLBAR_BG:     u32 = rgb(0x3C, 0x3C, 0x3C);
    pub const STATUS_BG:      u32 = rgb(0x00, 0x7A, 0xCC);
    pub const TEXT:           u32 = rgb(0xD4, 0xD4, 0xD4);
    pub const TEXT_DIM:       u32 = rgb(0x85, 0x85, 0x85);
    pub const DIVIDER:        u32 = rgb(0x3F, 0x3F, 0x46);

    /// GNOME / Adwaita dark theme.
    ///
    /// Canonical colors for a GNOME-style desktop: flat dark headerbars,
    /// a near-black top panel, the Adwaita blue accent (#3584e4), and
    /// uniform translucent-style window-control buttons. Consumers (e.g. an
    /// OS compositor) should source window-chrome and panel colors from here
    /// so the whole desktop stays cohesive.
    pub mod adwaita {
        use super::rgb;

        // Surfaces
        pub const WINDOW_BG:           u32 = rgb(0x24, 0x24, 0x24); // window content background
        pub const HEADERBAR:           u32 = rgb(0x30, 0x30, 0x30); // focused titlebar
        pub const HEADERBAR_UNFOCUSED: u32 = rgb(0x28, 0x28, 0x28); // unfocused recedes toward window bg
        pub const HEADERBAR_BORDER:    u32 = rgb(0x1B, 0x1B, 0x1B); // 1px line under the headerbar
        pub const WINDOW_BORDER:       u32 = rgb(0x14, 0x14, 0x14); // dark 1px outline around the window
        pub const WINDOW_BORDER_FOCUSED: u32 = rgb(0x44, 0x44, 0x44); // slightly lifted when focused

        // Text
        pub const TITLE:           u32 = rgb(0xFF, 0xFF, 0xFF); // focused title
        pub const TITLE_UNFOCUSED: u32 = rgb(0x90, 0x90, 0x90); // dimmed when unfocused

        // Accent
        pub const ACCENT:          u32 = rgb(0x35, 0x84, 0xE4); // Adwaita blue

        // Window-control buttons (uniform circular, GNOME style)
        pub const BTN_BG:            u32 = rgb(0x3A, 0x3A, 0x3A); // button disc on a focused headerbar
        pub const BTN_BG_UNFOCUSED:  u32 = rgb(0x32, 0x32, 0x32);
        pub const BTN_GLYPH:         u32 = rgb(0xDC, 0xDC, 0xDC); // –, ▢ glyphs
        pub const CLOSE_GLYPH:       u32 = rgb(0xF2, 0xF2, 0xF2); // × glyph (slightly brighter)

        // Top panel (GNOME Shell top bar — essentially black)
        pub const PANEL_BG:        u32 = rgb(0x1B, 0x1B, 0x1B);
        pub const PANEL_TEXT:      u32 = rgb(0xFF, 0xFF, 0xFF);
        pub const PANEL_TEXT_DIM:  u32 = rgb(0x9A, 0x9A, 0x9A);
        pub const PANEL_BORDER:    u32 = rgb(0x00, 0x00, 0x00);
    }
}
