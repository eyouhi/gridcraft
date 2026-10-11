//! Fonts from the optional craft-fonts build input.
//!
//! Built with `CRAFT_FONTS_DIR` pointing at a craft-fonts checkout, [`CRAFT_FONTS`] holds the
//! fonts its `fonts/manifest.txt` lists (the web build embeds only the browser-relevant faces);
//! otherwise the table is empty and the app uses system fonts and egui's defaults only. See
//! craft-fonts `docs/integration.md` for the contract.
//!
//! This is what makes Han text render in the browser: wasm has no filesystem, so the
//! system-CJK fallback in [`crate::theme`] can never load there.

/// One font from the optional craft-fonts build input (empty unless built with `CRAFT_FONTS_DIR`).
pub struct CraftFont {
    pub family: &'static str,
    pub style: &'static str,
    /// ISO 15924 scripts the font is for, e.g. `"Jpan"` or `"Hans"`.
    pub scripts: &'static [&'static str],
    pub bytes: &'static [u8],
}

include!(concat!(env!("OUT_DIR"), "/craft_fonts.rs"));

/// `CRAFT_FONTS` is always defined, so callers can treat an empty slice as "not built with it".
pub fn is_empty() -> bool {
    CRAFT_FONTS.is_empty()
}

/// Registration key of the font at `index` in egui's font data.
pub fn key(index: usize) -> String {
    format!("craft-font-{index}")
}

/// The embedded fonts as fallback keys for a role, in preference order: sans faces before
/// document (serif) faces, Simplified Chinese before Japanese, and the role's weight first.
/// Latin text stops at the app's own faces before reaching these, so they only render script
/// coverage the system lacks — in the browser, everything.
pub fn fallbacks(bold: bool) -> Vec<String> {
    let mut order: Vec<(u8, u8, usize)> =
        CRAFT_FONTS.iter().enumerate().map(|(i, f)| (family_rank(f.family), style_rank(f.style, bold), i)).collect();
    order.sort_unstable();
    order.into_iter().map(|(_, _, i)| key(i)).collect()
}

/// UI CJK faces first (Hans before Jpan), Arabic, then the Japanese document (serif) faces.
fn family_rank(family: &str) -> u8 {
    match family {
        "Noto Sans CJK SC" => 0,
        "BIZ UDPGothic" => 1,
        "Noto Sans Arabic" => 2,
        "BIZ UDMincho" => 3,
        "Shippori Mincho" => 4,
        _ => 5,
    }
}

/// The face matching the role's weight first; a family missing that weight still falls back.
fn style_rank(style: &str, bold: bool) -> u8 {
    u8::from(style.eq_ignore_ascii_case("Bold") != bold)
}
