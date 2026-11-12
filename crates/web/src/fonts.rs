//! Getting fonts into the browser.
//!
//! The browser build of the compiler has no fonts built in. After the page
//! hydrates, we download the few font files lessons need (the shell also
//! preloads them, so they download in parallel with the WebAssembly) and
//! register them. Until then, nothing that needs compiling is shown.

use leptos::prelude::*;
use typst_hero_core::fonts::FONT_URL_PREFIX;

/// `base` is true once text fonts are registered and the compiler can
/// render text; `math` once the math font is too. The math font is large
/// (~1.3 MB), so it loads in the background after the text fonts.
#[derive(Clone, Copy)]
pub struct FontsReady {
    pub base: RwSignal<bool>,
    pub math: RwSignal<bool>,
}

pub fn font_url(name: &str) -> String {
    format!("{FONT_URL_PREFIX}/{name}")
}

/// Call once at the top of the app. Server-side and during hydration the
/// signal is `false`, so both render the same "loading" markup.
pub fn provide_fonts() {
    let ready = RwSignal::new(false);
    let math = RwSignal::new(false);
    provide_context(FontsReady { base: ready, math });

    // Effects only run in the browser.
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            load(typst_hero_core::fonts::BASE_FONTS).await;
            ready.set(true);
            load(typst_hero_core::fonts::MATH_FONTS).await;
            math.set(true);
        });
    });
}

pub fn use_fonts_ready() -> RwSignal<bool> {
    expect_context::<FontsReady>().base
}

pub fn use_math_ready() -> RwSignal<bool> {
    expect_context::<FontsReady>().math
}

#[cfg(feature = "hydrate")]
async fn load(names: &[&str]) {
    for name in names {
        match fetch_bytes(&font_url(name)).await {
            Ok(bytes) => {
                typst_hero_core::fonts::add_font(bytes);
            }
            Err(err) => leptos::logging::error!("failed to load font {name}: {err:?}"),
        }
    }
}

#[cfg(feature = "hydrate")]
async fn fetch_bytes(url: &str) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or("no window")?;
    let response: web_sys::Response = JsFuture::from(window.fetch_with_str(url)).await?.dyn_into()?;
    if !response.ok() {
        return Err(format!("HTTP {}", response.status()).into());
    }
    let buffer = JsFuture::from(response.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buffer).to_vec())
}
