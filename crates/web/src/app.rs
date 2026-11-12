//! Page shell, routes and shared layout.

use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};
use typst_hero_core::fonts::BASE_FONTS;
use typst_hero_core::lessons::{COURSE, LESSONS};
use typst_hero_core::projects::PROJECTS;

use crate::fonts::{font_url, provide_fonts};
use crate::lesson::LessonPage;
use crate::progress::{provide_progress, use_progress};
use crate::settings::provide_settings;
use crate::practice::PracticePage;
use crate::project::ProjectPage;
use crate::stats::StatsPage;

/// The outer HTML document, rendered on the server.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                // Start downloading fonts right away, alongside the WebAssembly.
                {BASE_FONTS
                    .iter()
                    .map(|name| view! { <link rel="preload" href=font_url(name) r#as="fetch" crossorigin="anonymous" /> })
                    .collect_view()}
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_fonts();
    provide_progress();
    provide_settings();
    view! {
        <Stylesheet id="leptos" href="/pkg/typst-hero.css" />
        <Title formatter=|t: String| if t.is_empty() { "typst-hero".into() } else { format!("{t} | typst-hero") } />
        <Meta name="description" content="Learn Typst by doing: interactive lessons and drills that build real fluency." />
        <Router>
            <Header />
            <div class="layout">
                <Sidebar />
                <main class="content">
                    <Routes fallback=|| view! { <p>"Page not found."</p> }>
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=(StaticSegment("lessons"), ParamSegment("slug")) view=LessonPage />
                        <Route path=StaticSegment("stats") view=StatsPage />
                        <Route path=StaticSegment("practice") view=PracticePage />
                        <Route path=(StaticSegment("projects"), ParamSegment("slug")) view=ProjectPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="topbar">
            <A href="/" attr:class="logo">
                <span class="logo-mark">"#"</span>
                "typst-hero"
            </A>
            <nav>
                <A href="/lessons/emphasis">"Lessons"</A>
                <A href="/practice">"Practice"</A>
                <A href="/stats">"Stats"</A>
            </nav>
        </header>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let store = use_progress();
    view! {
        <aside class="sidebar">
            {COURSE
                .modules
                .iter()
                .map(|m| view! {
                    <p class="module">{m.title.as_str()}</p>
                    <ol>
                        {m.lessons
                            .iter()
                            .map(|&i| {
                                let l = &LESSONS[i];
                                let slug = l.slug.as_str();
                                view! {
                                    <li class:done=move || store.with(|p| p.completed(slug))>
                                        <A href=format!("/lessons/{slug}")>{l.title.as_str()}</A>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ol>
                })
                .collect_view()}
            <p class="module">"Projects"</p>
            <ul class="projects">
                {PROJECTS
                    .iter()
                    .map(|p| {
                        let slug = p.slug.as_str();
                        let total = p.steps.len();
                        view! {
                            <li class:done=move || store.with(|s| s.projects.get(slug).is_some_and(|x| x.step >= total))>
                                <A href=format!("/projects/{slug}")>{p.title.as_str()}</A>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </aside>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="" />
        <section class="hero">
            <h1>"Learn Typst. Make it stick."</h1>
            <p>
                "Short lessons, a real editor, and drills that are different every time. "
                "Earn XP only when your solution is as clean as the ideal one."
            </p>
            <A href="/lessons/emphasis" attr:class="button">"Start learning"</A>
        </section>
    }
}
