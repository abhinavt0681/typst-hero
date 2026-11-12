//! Font registry.
//!
//! On the server (and in tests) all of Typst's bundled fonts are embedded in
//! the binary (`embedded-fonts` feature). In the browser that would add ~10 MB
//! to the download, so the browser build starts with no fonts and fetches just
//! the files it needs from the server (see `BASE_FONTS`), registering them
//! with `add_font`.

use std::sync::{Arc, LazyLock, RwLock};

use typst::foundations::Bytes;
use typst::text::{Font, FontBook, FontInfo, FontStyle};
use typst::utils::LazyHash;

/// URL prefix the server serves font files under. Versioned, so files can be
/// cached forever and a Typst upgrade busts the cache.
pub const FONT_URL_PREFIX: &str = "/fonts/typst-0.15.1";

/// Fonts every lesson needs: body text (regular, bold, italic, bold italic)
/// and monospace for `raw` code.
pub const BASE_FONTS: &[&str] = &[
    "libertinus-serif-400-normal",
    "libertinus-serif-400-italic",
    "libertinus-serif-700-normal",
    "libertinus-serif-700-italic",
    "dejavu-sans-mono-400-normal",
];

/// Needed only by math lessons.
pub const MATH_FONTS: &[&str] = &["new-computer-modern-math-400-normal"];

/// An immutable snapshot of the registered fonts. Each compilation holds an
/// `Arc` to one snapshot, so adding fonts never disturbs a running compile.
pub struct FontSet {
    pub book: LazyHash<FontBook>,
    pub fonts: Vec<Font>,
}

impl FontSet {
    fn new(fonts: Vec<Font>) -> Self {
        Self {
            book: LazyHash::new(FontBook::from_fonts(&fonts)),
            fonts,
        }
    }
}

static REGISTRY: LazyLock<RwLock<Arc<FontSet>>> = LazyLock::new(|| {
    #[cfg(feature = "embedded-fonts")]
    let fonts = embedded_fonts().into_iter().map(|(_, font)| font).collect();
    #[cfg(not(feature = "embedded-fonts"))]
    let fonts = Vec::new();
    RwLock::new(Arc::new(FontSet::new(fonts)))
});

/// The current set of fonts.
pub fn current() -> Arc<FontSet> {
    REGISTRY.read().expect("font registry poisoned").clone()
}

/// Register a font file (OTF/TTF bytes). Returns how many faces were added;
/// faces that are already registered are skipped.
pub fn add_font(data: Vec<u8>) -> usize {
    let new: Vec<Font> = Font::iter(Bytes::new(data)).collect();
    let mut registry = REGISTRY.write().expect("font registry poisoned");
    let mut fonts = registry.fonts.clone();
    let before = fonts.len();
    for font in new {
        if !fonts.iter().any(|f| f.info() == font.info()) {
            fonts.push(font);
        }
    }
    let added = fonts.len() - before;
    if added > 0 {
        *registry = Arc::new(FontSet::new(fonts));
    }
    added
}

/// A stable, URL-friendly name for a font face, e.g.
/// `libertinus-serif-700-italic`.
pub fn font_name(info: &FontInfo) -> String {
    let family = info.family.to_lowercase().replace(' ', "-");
    let style = match info.variant.style {
        FontStyle::Normal => "normal",
        FontStyle::Italic => "italic",
        FontStyle::Oblique => "oblique",
    };
    format!("{family}-{}-{style}", info.variant.weight.to_number())
}

/// All embedded font faces with their names (server side).
#[cfg(feature = "embedded-fonts")]
pub fn embedded_fonts() -> Vec<(String, Font)> {
    typst_assets::fonts()
        .flat_map(|data| Font::iter(Bytes::new(data)))
        .map(|font| (font_name(font.info()), font))
        .collect()
}

/// The raw file for a named face, for the server to send to browsers.
#[cfg(feature = "embedded-fonts")]
pub fn embedded_font_file(name: &str) -> Option<&'static [u8]> {
    typst_assets::fonts()
        .find(|data| Font::iter(Bytes::new(*data)).any(|font| font_name(font.info()) == name))
}
