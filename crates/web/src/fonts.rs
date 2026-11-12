//! Getting fonts into the browser.
//!
//! The browser build of the compiler has no fonts built in. After the page
//! hydrates, we download the few font files lessons need (the shell also
//! preloads them, so they download in parallel with the WebAssembly) and
//! register them. Until then, nothing that needs compiling is shown.

use leptos::prelude::*;
use typst_hero_core::fonts::FONT_URL_PREFIX;

/// `true` once fonts are registered and the compiler can render text.
#[derive(Clone, Copy)]
pub struct FontsReady(pub RwSignal<bool>);

pub fn font_url(name: &str) -> String {
    format!("{FONT_URL_PREFIX}/{name}")
}

/// Call once at the top of the app. Server-side and during hydration the
/// signal is `false`, so both render the same "loading" markup.
pub fn provide_fonts() {
    let ready = RwSignal::new(false);
    provide_context(FontsReady(ready));

    // Effects only run in the browser.
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            for name in typst_hero_core::fonts::BASE_FONTS {
                match fetch_bytes(&font_url(name)).await {
                    Ok(bytes) => {
                        typst_hero_core::fonts::add_font(bytes);
                    }
                    Err(err) => leptos::logging::error!("failed to load font {name}: {err:?}"),
                }
            }
            ready.set(true);
        });
    });
}

pub fn use_fonts_ready() -> RwSignal<bool> {
    expect_context::<FontsReady>().0
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
