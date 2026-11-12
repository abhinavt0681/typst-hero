//! The interactive challenge: reproduce the target, goal after goal.
//!
//! Every keystroke recompiles the learner's code with the real Typst compiler
//! (running in the browser as WebAssembly) and compares it with the target.

use leptos::html;
use leptos::prelude::*;
use leptos_router::components::A;
use typst_hero_core::rng::Rng;
use typst_hero_core::{Diagnostic, Goal, Lesson, Rendered, Verdict, compile_str, judge};

use crate::fonts::use_fonts_ready;
use crate::progress::{Run, use_progress};
use crate::time::now_ms;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Challenge,
    Stats,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Ready,
    Running,
    Done,
}

/// A solved goal, kept for the summary screen.
#[derive(Clone)]
struct Solved {
    goal: Goal,
    code: String,
    efficient: bool,
}

#[component]
pub fn Challenge(lesson: &'static Lesson, next_slug: Option<&'static str>) -> impl IntoView {
    // Seed 1 on first render so the server and browser show the same preview.
    // Starting the challenge re-seeds from the clock.
    let rng = StoredValue::new(Rng::new(1));
    let draw = move |prev: Option<&Goal>| loop {
        let goal = rng.try_update_value(|r| (lesson.generate)(r)).expect("rng alive");
        if prev != Some(&goal) {
            break goal;
        }
    };

    let goal = RwSignal::new(draw(None));
    let code = RwSignal::new(String::new());
    let phase = RwSignal::new(Phase::Ready);
    let solved = RwSignal::new(Vec::<Solved>::new());
    let started_at = StoredValue::new(0.0);
    let elapsed_ms = RwSignal::new(0.0);
    let editor = NodeRef::<html::Textarea>::new();
    let store = use_progress();
    let tab = RwSignal::new(Tab::Challenge);

    // Recompile when fonts arrive, too (`ready.track()`).
    let ready = use_fonts_ready();
    let target = Memo::new(move |_| {
        ready.track();
        goal.with(|g| compile_str(&g.ideal).ok())
    });
    let output = Memo::new(move |_| {
        ready.track();
        code.with(|c| compile_str(c))
    });

    let start = move || {
        rng.set_value(Rng::new(now_ms() as u64));
        let g = draw(None);
        code.set(g.start.clone());
        goal.set(g);
        solved.set(Vec::new());
        started_at.set_value(now_ms());
        phase.set(Phase::Running);
        request_animation_frame(move || {
            if let Some(el) = editor.get() {
                let _ = el.focus();
            }
        });
    };

    // Called after every edit: if the output matches, record it and move on.
    let check = move || {
        let (Some(t), Ok(o)) = (target.get_untracked(), output.get_untracked()) else { return };
        let g = goal.get_untracked();
        let c = code.get_untracked();
        if let Verdict::Solved { efficient } = judge(&t, &o, &c, &g.ideal) {
            solved.update(|s| s.push(Solved { goal: g.clone(), code: c, efficient }));
            if solved.with_untracked(Vec::len) >= lesson.goals {
                let ms = now_ms() - started_at.get_value();
                elapsed_ms.set(ms);
                let efficient = solved.with_untracked(|s| s.iter().filter(|x| x.efficient).count());
                store.record(lesson.slug, Run { ms, efficient, goals: lesson.goals, at: now_ms() });
                phase.set(Phase::Done);
            } else {
                let next = draw(Some(&g));
                code.set(next.start.clone());
                goal.set(next);
            }
        }
    };

    let progress = move || format!("width: {}%", 100 * solved.with(Vec::len) / lesson.goals);

    view! {
        <section class="challenge">
            <div class="tabs" role="tablist">
                <button role="tab" class:active=move || tab.get() == Tab::Challenge on:click=move |_| tab.set(Tab::Challenge)>
                    "Challenge"
                </button>
                <button role="tab" class:active=move || tab.get() == Tab::Stats on:click=move |_| tab.set(Tab::Stats)>
                    "Stats"
                </button>
            </div>
            {move || if tab.get() == Tab::Stats {
                return view! { <ChallengeStats slug=lesson.slug /> }.into_any();
            } else { match phase.get() {
                Phase::Ready => view! {
                    <div class="ready">
                        <Show
                            when=move || ready.get()
                            fallback=|| view! { <div class="pane placeholder">"Loading the Typst compiler…"</div> }
                        >
                            <Pane label="Target" rendered=Signal::derive(move || target.get().ok_or_else(Vec::new)) />
                        </Show>
                        <button class="button" disabled=move || !ready.get() on:click=move |_| start()>
                            {move || if ready.get() { "▶ Start challenge" } else { "Loading…" }}
                        </button>
                        <p class="muted">{lesson.goals} " goals · a new one appears as soon as you match the target"</p>
                    </div>
                }.into_any(),
                Phase::Running => view! {
                    <div class="progress"><div class="bar" style=progress></div></div>
                    <div class="panes">
                        <Pane label="Target" rendered=Signal::derive(move || target.get().ok_or_else(Vec::new)) />
                        <Pane label="Your output" rendered=output.into() />
                    </div>
                    <textarea
                        class="editor"
                        node_ref=editor
                        spellcheck="false"
                        autocomplete="off"
                        prop:value=move || code.get()
                        on:input=move |ev| {
                            code.set(event_target_value(&ev));
                            check();
                        }
                    ></textarea>
                    <div class="statusline">
                        <span class="mode">"TYPST"</span>
                        <span>"goal " {move || solved.with(Vec::len) + 1} " / " {lesson.goals}</span>
                        <button class="link" on:click=move |_| start()>"↻ restart"</button>
                    </div>
                }.into_any(),
                Phase::Done => view! { <Summary solved elapsed_ms next_slug on_again=Callback::new(move |_| start()) /> }.into_any(),
            }}}
        </section>
    }
}

/// A labelled preview: the rendered page, or the errors.
#[component]
fn Pane(label: &'static str, rendered: Signal<Result<Rendered, Vec<Diagnostic>>>) -> impl IntoView {
    view! {
        <figure class="pane">
            <figcaption>{label}</figcaption>
            {move || match rendered.get() {
                Ok(r) => view! { <div class="page" inner_html=r.pages.join("")></div> }.into_any(),
                Err(errs) => view! {
                    <ul class="errors">
                        {errs.into_iter().filter(|d| d.is_error).map(|d| view! {
                            <li>
                                {d.position.map(|(l, c)| format!("line {l}:{c} · "))}
                                {d.message}
                                {d.hints.into_iter().map(|h| view! { <div class="hint">"hint: " {h}</div> }).collect_view()}
                            </li>
                        }).collect_view()}
                    </ul>
                }.into_any(),
            }}
        </figure>
    }
}

#[component]
fn Summary(
    solved: RwSignal<Vec<Solved>>,
    elapsed_ms: RwSignal<f64>,
    next_slug: Option<&'static str>,
    on_again: Callback<()>,
) -> impl IntoView {
    let all = solved.get_untracked();
    let efficient = all.iter().filter(|s| s.efficient).count();
    let inefficient: Vec<Solved> = all.iter().filter(|s| !s.efficient).cloned().collect();
    let secs = elapsed_ms.get_untracked() / 1000.0;
    view! {
        <div class="summary">
            <h3>"Challenge complete"</h3>
            <div class="stats-row">
                <div><small>"efficient goals"</small><strong>{efficient} <span class="muted">"/" {all.len()}</span></strong></div>
                <div><small>"time"</small><strong>{format!("{secs:.1}s")}</strong></div>
            </div>
            {(!inefficient.is_empty()).then(|| view! {
                <h4>"Review: shorter solutions exist"</h4>
                {inefficient.into_iter().map(|s| view! {
                    <div class="review">
                        <div><span class="tag ideal">"ideal"</span><code>{s.goal.ideal.clone()}</code></div>
                        <div><span class="tag yours">"yours"</span><code>{s.code}</code></div>
                    </div>
                }).collect_view()}
            })}
            <div class="actions">
                <button class="button secondary" on:click=move |_| on_again.run(())>"↻ Practice again"</button>
                {next_slug.map(|slug| view! { <A href=format!("/lessons/{slug}") attr:class="button">"Next lesson →"</A> })}
            </div>
        </div>
    }
}

/// Best time, recent average and a chart of every completion, like VimHero.
#[component]
fn ChallengeStats(slug: &'static str) -> impl IntoView {
    let store = use_progress();
    move || {
        let times: Vec<f64> = store.with(|p| p.runs(slug).iter().map(|r| r.ms / 1000.0).collect());
        if times.is_empty() {
            return view! { <p class="muted empty">"Finish the challenge once to see your stats here."</p> }.into_any();
        }
        let best = times.iter().copied().fold(f64::INFINITY, f64::min);
        let recent = &times[times.len().saturating_sub(15)..];
        let average = recent.iter().sum::<f64>() / recent.len() as f64;
        view! {
            <div class="stats-row">
                <div><small>"best time"</small><strong>{format!("{best:.1}s")}</strong></div>
                <div><small>"recent average"</small><strong>{format!("{average:.1}s")}</strong><small>"last " {recent.len()} " runs"</small></div>
                <div><small>"completions"</small><strong>{times.len()}</strong></div>
            </div>
            <TimeChart times />
        }
        .into_any()
    }
}

/// A small line chart of completion times (lower is better), drawn as SVG.
#[component]
fn TimeChart(times: Vec<f64>) -> impl IntoView {
    const W: f64 = 600.0;
    const H: f64 = 180.0;
    const PAD: f64 = 28.0;
    let max = times.iter().copied().fold(1.0, f64::max) * 1.15;
    let step = if times.len() > 1 { (W - 2.0 * PAD) / (times.len() - 1) as f64 } else { 0.0 };
    let points: Vec<(f64, f64)> = times
        .iter()
        .enumerate()
        .map(|(i, t)| (PAD + i as f64 * step, H - PAD - (t / max) * (H - 2.0 * PAD)))
        .collect();
    let line = points.iter().map(|(x, y)| format!("{x:.1},{y:.1}")).collect::<Vec<_>>().join(" ");
    view! {
        <svg class="chart" viewBox=format!("0 0 {W} {H}") role="img" aria-label="Completion times, lower is better">
            <line x1=PAD y1=H - PAD x2=W - PAD y2=H - PAD class="axis" />
            <text x=4 y=PAD + 4.0 class="label">{format!("{max:.0}s")}</text>
            <text x=4 y=H - PAD + 4.0 class="label">"0s"</text>
            <polyline points=line class="series" />
            {points.into_iter().map(|(x, y)| view! { <circle cx=x cy=y r=3 class="dot" /> }).collect_view()}
            <text x=W / 2.0 y=H - 6.0 class="label" text-anchor="middle">"completion times (lower is better)"</text>
        </svg>
    }
}
