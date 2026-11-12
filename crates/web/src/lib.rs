//! typst-hero-web: the Leptos app.
//!
//! The same components run twice:
//! - on the server (`ssr` feature) to produce HTML, which search engines and
//!   link previews can read, and which shows up instantly;
//! - in the browser (`hydrate` feature, compiled to WebAssembly) to attach
//!   interactivity to that HTML. From then on, Typst compiles locally.

// Leptos encodes the whole view tree in types; release builds of nested
// pages need a deeper limit than the default 128.
#![recursion_limit = "256"]

pub mod app;
mod challenge;
mod drill;
mod editor;
pub mod fonts;
mod lesson;
mod practice;
mod progress;
mod settings;
mod stats;
mod time;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
