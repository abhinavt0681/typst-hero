//! The server binary: an Axum web server that renders Leptos pages to HTML.

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use typst_hero_web::app::{App, shell};

    let conf = get_configuration(None).expect("cargo-leptos config");
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&options, routes, {
            let options = options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(options);

    println!("typst-hero listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind address");
    axum::serve(listener, app.into_make_service()).await.expect("server");
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // The browser entry point is `hydrate()` in lib.rs.
}
