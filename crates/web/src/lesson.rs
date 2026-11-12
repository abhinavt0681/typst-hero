//! A lesson page: intro, syntax chips with examples, then the challenge.

use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};
use typst_hero_core::exercise::Chip;
use typst_hero_core::lessons::{LESSONS, lesson, module_of};
use typst_hero_core::skills;
use typst_hero_core::{Lesson, compile_str};

use crate::challenge::Challenge;
use crate::fonts::{use_fonts_ready, use_math_ready};

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
        <Title text=lesson.title.clone() />
        <article class="lesson">
            <p class="eyebrow">{module_of(lesson).map(|m| m.title.as_str())} " · lesson " {index + 1}</p>
            <h1>{lesson.title.as_str()}</h1>
            {lesson.intro.iter().map(|p| view! { <p>{rich(p.as_str())}</p> }).collect_view()}

            <div class="chips">
                {lesson.chips.iter().map(|chip| view! { <ChipRow chip /> }).collect_view()}
            </div>

            <h2>"Now it's your turn!"</h2>
            <ol class="task">
                {lesson.task.iter().map(|t| view! { <li>{rich(t.as_str())}</li> }).collect_view()}
            </ol>
            <Challenge lesson next_slug=next.map(|l| l.slug.as_str()) />

            <nav class="pager">
                {prev.map(|l| view! {
                    <A href=format!("/lessons/{}", l.slug) attr:class="pager-link">
                        <small>"Back " <kbd>"Ctrl+k"</kbd></small>
                        <span>{l.title.as_str()}</span>
                    </A>
                })}
                <span class="spacer"></span>
                {next.map(|l| view! {
                    <A href=format!("/lessons/{}", l.slug) attr:class="pager-link right">
                        <small>"Next " <kbd>"Ctrl+j"</kbd></small>
                        <span>{l.title.as_str()}</span>
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
                <code class="chip">{chip.syntax.as_str()}</code>
                <span class="meaning">"– " {chip.meaning.as_str()}</span>
                {chip.skill.as_deref().and_then(skills::skill).map(|s| view! {
                    <a class="docs-link" href=skills::docs_url(s) target="_blank" rel="noopener" title="Official Typst documentation">"docs ↗"</a>
                })}
                <button class="toggle" on:click=move |_| open.update(|o| *o = !*o)>
                    {move || if open.get() { "▪ hide example" } else { "▶ show example" }}
                </button>
            </div>
            <Show when=move || open.get()>
                <Example code=chip.example.as_str() />
            </Show>
        </div>
    }
}

/// The example types itself out while the preview updates live, so you see
/// exactly when the markup takes effect (like VimHero's replayed keystrokes).
#[component]
fn Example(code: &'static str) -> impl IntoView {
    const MS_PER_CHAR: u64 = 45;
    // Examples with math wait for the math font as well.
    let base_ready = use_fonts_ready();
    let math_ready = use_math_ready();
    let has_math = code.contains('$');
    let ready = Signal::derive(move || base_ready.get() && (!has_math || math_ready.get()));
    let chars: Vec<char> = code.chars().collect();
    let total = chars.len();
    let shown = RwSignal::new(0usize);
    // The last prefix that compiled. Half-typed markup like `*bold` is an
    // error, so the preview keeps showing the previous good state.
    let last_good = RwSignal::new(None::<String>);

    let tick = move || {
        if !ready.get_untracked() || shown.get_untracked() >= total {
            return;
        }
        let n = shown.get_untracked() + 1;
        shown.set(n);
        let prefix: String = chars[..n].iter().collect();
        if let Ok(r) = compile_str(&prefix) {
            last_good.set(r.pages.into_iter().next());
        }
    };
    if let Ok(handle) = set_interval_with_handle(tick, std::time::Duration::from_millis(MS_PER_CHAR)) {
        on_cleanup(move || handle.clear());
    }

    let typed = move || code.chars().take(shown.get()).collect::<String>();
    let done = move || shown.get() >= total;
    view! {
        <div class="example">
            <div class="source">
                <div class="source-head">
                    <span class="muted">"input"</span>
                    <button class="toggle" on:click=move |_| { shown.set(0); last_good.set(None); }>"↻ replay"</button>
                </div>
                <pre>{typed}<span class="cursor" class:idle=done></span></pre>
            </div>
            {move || match last_good.get() {
                Some(svg) => view! { <div class="page" inner_html=svg></div> }.into_any(),
                None => view! { <div class="page placeholder">{move || if ready.get() { "" } else { "Loading…" }}</div> }.into_any(),
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
