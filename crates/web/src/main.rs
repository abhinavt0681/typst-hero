//! The server binary: an Axum web server that renders Leptos pages to HTML.

// Leptos encodes the whole view tree in types; release builds of nested
// pages need a deeper limit than the default 128.
#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum::routing::get;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use tower_http::compression::CompressionLayer;
    use typst_hero_core::fonts::FONT_URL_PREFIX;
    use typst_hero_web::app::{App, shell};

    let conf = get_configuration(None).expect("cargo-leptos config");
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);

    // Font files, compressed on the fly (brotli/gzip) and cached for a year.
    let fonts = Router::new()
        .route(&format!("{FONT_URL_PREFIX}/{{name}}"), get(serve_font))
        .layer(CompressionLayer::new());

    let app = Router::new()
        .leptos_routes(&options, routes, {
            let options = options.clone();
            move || shell(options.clone())
        })
        .merge(fonts)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(options);

    println!("typst-hero listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind address");
    axum::serve(listener, app.into_make_service()).await.expect("server");
}

#[cfg(feature = "ssr")]
async fn serve_font(axum::extract::Path(name): axum::extract::Path<String>) -> axum::response::Response {
    use axum::http::{StatusCode, header};
    use axum::response::IntoResponse;

    match typst_hero_core::fonts::embedded_font_file(&name) {
        Some(data) => (
            [
                (header::CONTENT_TYPE, if data.starts_with(b"OTTO") { "font/otf" } else { "font/ttf" }),
                (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
            ],
            data,
        )
            .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // The browser entry point is `hydrate()` in lib.rs.
}
