//! Wall-clock time, which only exists in the browser.

/// Milliseconds since the Unix epoch. Returns 0 on the server, where timing
/// is never needed (challenges only run in the browser).
pub fn now_ms() -> f64 {
    #[cfg(feature = "hydrate")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(feature = "hydrate"))]
    {
        0.0
    }
}
