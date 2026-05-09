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

// ── Named palette ─────────────────────────────────────────────────────────────

pub mod palette {
    use super::rgb;

    pub const BLACK:       u32 = rgb(0x00, 0x00, 0x00);
    pub const WHITE:       u32 = rgb(0xFF, 0xFF, 0xFF);
    pub const DARK_GRAY:   u32 = rgb(0x1E, 0x1E, 0x1E);
    pub const GRAY:        u32 = rgb(0x52, 0x52, 0x52);
    pub const LIGHT_GRAY:  u32 = rgb(0xCC, 0xCC, 0xCC);
    pub const RED:         u32 = rgb(0xF1, 0x4C, 0x4C);
    pub const GREEN:       u32 = rgb(0x40, 0xC0, 0x40);
    pub const BLUE:        u32 = rgb(0x00, 0x7A, 0xCC);
    pub const YELLOW:      u32 = rgb(0xFF, 0xD7, 0x00);
    pub const CYAN:        u32 = rgb(0x4E, 0xC9, 0xB0);
    pub const ACCENT:      u32 = rgb(0x4E, 0xC9, 0xB0);
    pub const DARK_BG:     u32 = rgb(0x1E, 0x1E, 0x1E);
    pub const PANEL_BG:    u32 = rgb(0x25, 0x25, 0x26);
    pub const TOOLBAR_BG:  u32 = rgb(0x3C, 0x3C, 0x3C);
    pub const STATUS_BG:   u32 = rgb(0x00, 0x7A, 0xCC);
    pub const TEXT:        u32 = rgb(0xD4, 0xD4, 0xD4);
    pub const TEXT_DIM:    u32 = rgb(0x85, 0x85, 0x85);
    pub const DIVIDER:     u32 = rgb(0x3F, 0x3F, 0x46);
}
