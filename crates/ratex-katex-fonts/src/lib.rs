//! KaTeX TrueType fonts embedded at compile time for `ratex-svg` / `ratex-render`
//! `embed-fonts` features. Files live under this crate's `fonts/` so `cargo publish`
//! tarballs stay self-contained.

use std::borrow::Cow;

static FONTS: &[(&str, &[u8])] = &[
    (
        "KaTeX_AMS-Regular.ttf",
        include_bytes!("../fonts/KaTeX_AMS-Regular.ttf"),
    ),
    (
        "KaTeX_Caligraphic-Bold.ttf",
        include_bytes!("../fonts/KaTeX_Caligraphic-Bold.ttf"),
    ),
    (
        "KaTeX_Caligraphic-Regular.ttf",
        include_bytes!("../fonts/KaTeX_Caligraphic-Regular.ttf"),
    ),
    (
        "KaTeX_Fraktur-Bold.ttf",
        include_bytes!("../fonts/KaTeX_Fraktur-Bold.ttf"),
    ),
    (
        "KaTeX_Fraktur-Regular.ttf",
        include_bytes!("../fonts/KaTeX_Fraktur-Regular.ttf"),
    ),
    (
        "KaTeX_Main-Bold.ttf",
        include_bytes!("../fonts/KaTeX_Main-Bold.ttf"),
    ),
    (
        "KaTeX_Main-BoldItalic.ttf",
        include_bytes!("../fonts/KaTeX_Main-BoldItalic.ttf"),
    ),
    (
        "KaTeX_Main-Italic.ttf",
        include_bytes!("../fonts/KaTeX_Main-Italic.ttf"),
    ),
    (
        "KaTeX_Main-Regular.ttf",
        include_bytes!("../fonts/KaTeX_Main-Regular.ttf"),
    ),
    (
        "KaTeX_Math-BoldItalic.ttf",
        include_bytes!("../fonts/KaTeX_Math-BoldItalic.ttf"),
    ),
    (
        "KaTeX_Math-Italic.ttf",
        include_bytes!("../fonts/KaTeX_Math-Italic.ttf"),
    ),
    (
        "KaTeX_SansSerif-Bold.ttf",
        include_bytes!("../fonts/KaTeX_SansSerif-Bold.ttf"),
    ),
    (
        "KaTeX_SansSerif-Italic.ttf",
        include_bytes!("../fonts/KaTeX_SansSerif-Italic.ttf"),
    ),
    (
        "KaTeX_SansSerif-Regular.ttf",
        include_bytes!("../fonts/KaTeX_SansSerif-Regular.ttf"),
    ),
    (
        "KaTeX_Script-Regular.ttf",
        include_bytes!("../fonts/KaTeX_Script-Regular.ttf"),
    ),
    (
        "KaTeX_Size1-Regular.ttf",
        include_bytes!("../fonts/KaTeX_Size1-Regular.ttf"),
    ),
    (
        "KaTeX_Size2-Regular.ttf",
        include_bytes!("../fonts/KaTeX_Size2-Regular.ttf"),
    ),
    (
        "KaTeX_Size3-Regular.ttf",
        include_bytes!("../fonts/KaTeX_Size3-Regular.ttf"),
    ),
    (
        "KaTeX_Size4-Regular.ttf",
        include_bytes!("../fonts/KaTeX_Size4-Regular.ttf"),
    ),
    (
        "KaTeX_Typewriter-Regular.ttf",
        include_bytes!("../fonts/KaTeX_Typewriter-Regular.ttf"),
    ),
];

/// Returns embedded font bytes for a KaTeX TTF filename (e.g. `KaTeX_Main-Regular.ttf`).
pub fn ttf_bytes(filename: &str) -> Option<Cow<'static, [u8]>> {
    FONTS
        .iter()
        .find(|(name, _)| *name == filename)
        .map(|(_, bytes)| Cow::Borrowed(*bytes))
}
