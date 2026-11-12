//! A lesson page: intro, syntax chips with examples, then the challenge.

use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};
use typst_hero_core::exercise::Chip;
use typst_hero_core::lessons::{LESSONS, lesson};
use typst_hero_core::{Lesson, compile_str};

use crate::challenge::Challenge;
use crate::fonts::use_fonts_ready;

#[component]
pub fn LessonPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    // Ctrl+j / Ctrl+k move to the next / previous lesson, like VimHero.
    let navigate = use_navigate();
    let handle = window_event_listener(ev::keydown, move |e| {
        if !e.ctrl_key() {
            return;
        }
        let Some((i, _)) = lesson(&slug()) else { return };
        let target = match e.key().as_str() {
            "j" => LESSONS.get(i + 1),
            "k" => i.checked_sub(1).and_then(|p| LESSONS.get(p)),
            _ => return,
        };
        if let Some(l) = target {
            e.prevent_default();
            navigate(&format!("/lessons/{}", l.slug), Default::default());
        }
    });
    on_cleanup(move || handle.remove());

    // Rebuilds the whole view (and resets the challenge) when the slug changes.
    move || match lesson(&slug()) {
        Some((index, lesson)) => view! { <LessonView index lesson /> }.into_any(),
        None => view! { <p>"No such lesson."</p> }.into_any(),
    }
}

#[component]
fn LessonView(index: usize, lesson: &'static Lesson) -> impl IntoView {
    let prev = index.checked_sub(1).and_then(|i| LESSONS.get(i));
    let next = LESSONS.get(index + 1);
    view! {
        <Title text=lesson.title />
        <article class="lesson">
            <p class="eyebrow">{lesson.module} " · lesson " {index + 1}</p>
            <h1>{lesson.title}</h1>
            {lesson.intro.iter().map(|p| view! { <p>{rich(p)}</p> }).collect_view()}

            <div class="chips">
                {lesson.chips.iter().map(|chip| view! { <ChipRow chip /> }).collect_view()}
            </div>

            <h2>"Now it's your turn!"</h2>
            <ol class="task">
                {lesson.task.iter().map(|t| view! { <li>{rich(t)}</li> }).collect_view()}
            </ol>
            <Challenge lesson next_slug=next.map(|l| l.slug) />

            <nav class="pager">
                {prev.map(|l| view! {
                    <A href=format!("/lessons/{}", l.slug) attr:class="pager-link">
                        <small>"Back " <kbd>"Ctrl+k"</kbd></small>
                        <span>{l.title}</span>
                    </A>
                })}
                <span class="spacer"></span>
                {next.map(|l| view! {
                    <A href=format!("/lessons/{}", l.slug) attr:class="pager-link right">
                        <small>"Next " <kbd>"Ctrl+j"</kbd></small>
                        <span>{l.title}</span>
                    </A>
                })}
            </nav>
        </article>
    }
}

#[component]
fn ChipRow(chip: &'static Chip) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <div class="chip-block" class:open=move || open.get()>
            <div class="chip-row">
                <code class="chip">{chip.syntax}</code>
                <span class="meaning">"– " {chip.meaning}</span>
                <button class="toggle" on:click=move |_| open.update(|o| *o = !*o)>
                    {move || if open.get() { "▪ hide example" } else { "▶ show example" }}
                </button>
            </div>
            <Show when=move || open.get()>
                <Example code=chip.example />
            </Show>
        </div>
    }
}

/// Source on the left, its rendering on the right.
#[component]
fn Example(code: &'static str) -> impl IntoView {
    let ready = use_fonts_ready();
    let svg = move || {
        ready.get().then(|| compile_str(code).ok().and_then(|r| r.pages.into_iter().next())).flatten()
    };
    view! {
        <div class="example">
            <pre class="source">{code}</pre>
            {move || match svg() {
                Some(svg) => view! { <div class="page" inner_html=svg></div> }.into_any(),
                None => view! { <div class="page placeholder">"Loading…"</div> }.into_any(),
            }}
        </div>
    }
}

/// Renders text, showing `backticked` parts as inline code.
fn rich(text: &'static str) -> impl IntoView {
    text.split('`')
        .enumerate()
        .map(|(i, part)| if i % 2 == 1 { view! { <code>{part}</code> }.into_any() } else { part.into_any() })
        .collect_view()
}
