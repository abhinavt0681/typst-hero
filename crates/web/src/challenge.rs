//! A lesson's challenge: a fixed number of goals from the lesson's generator.
//!
//! Every keystroke recompiles the learner's code with the real Typst compiler
//! (running in the browser as WebAssembly) and compares it with the target.

use leptos::prelude::*;
use leptos_router::components::A;
use typst_hero_core::Lesson;

use crate::drill::{Drill, DrillWorkspace, Pane, Results};
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

#[component]
pub fn Challenge(lesson: &'static Lesson, next_slug: Option<&'static str>) -> impl IntoView {
    let drill = Drill::new(vec![lesson.generate]);
    let phase = RwSignal::new(Phase::Ready);
    let tab = RwSignal::new(Tab::Challenge);
    let elapsed_ms = RwSignal::new(0.0);
    let ready = use_fonts_ready();
    let store = use_progress();

    let start = move || {
        drill.start();
        phase.set(Phase::Running);
    };

    let check = move || {
        if drill.check() && drill.solved.with_untracked(Vec::len) >= lesson.goals {
            let ms = drill.elapsed_ms();
            elapsed_ms.set(ms);
            let run = Run { ms, efficient: drill.efficient_count(), goals: lesson.goals, at: now_ms() };
            store.finish_run(&lesson.slug, run);
            phase.set(Phase::Done);
        }
    };

    let progress = move || format!("width: {}%", 100 * drill.solved_count() / lesson.goals);
    let status = Signal::derive(move || format!("goal {} / {}", drill.solved_count() + 1, lesson.goals));
    let target = drill.target;

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
                view! { <ChallengeStats slug=lesson.slug.as_str() /> }.into_any()
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
                    <DrillWorkspace drill status on_edit=Callback::new(move |_| check()) on_restart=Callback::new(move |_| start()) />
                }.into_any(),
                Phase::Done => view! {
                    <Results drill title="Challenge complete" elapsed_ms=elapsed_ms.get_untracked()>
                        <button class="button secondary" on:click=move |_| start()>"↻ Practice again"</button>
                        {next_slug.map(|slug| view! { <A href=format!("/lessons/{slug}") attr:class="button">"Next lesson →"</A> })}
                    </Results>
                }.into_any(),
            }}}
        </section>
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
