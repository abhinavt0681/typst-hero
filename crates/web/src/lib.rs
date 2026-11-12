//! typst-hero-web: the Leptos app.
//!
//! The same components run twice:
//! - on the server (`ssr` feature) to produce HTML, which search engines and
//!   link previews can read, and which shows up instantly;
//! - in the browser (`hydrate` feature, compiled to WebAssembly) to attach
//!   interactivity to that HTML. From then on, Typst compiles locally.

pub mod app;
mod challenge;
mod lesson;
mod time;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
