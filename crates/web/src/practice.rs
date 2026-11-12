//! The Practice page: drill any mix of lessons, by goal count, against the
//! clock, or endlessly, like VimHero's practice mode.

use std::collections::BTreeSet;

use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use typst_hero_core::Lesson;
use typst_hero_core::lessons::{COURSE, LESSONS};

use crate::drill::{Drill, DrillWorkspace, Generator, Results};
use crate::fonts::use_fonts_ready;
use crate::progress::use_progress;
use crate::time::now_ms;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Goals,
    Time,
    Endless,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Setup,
    Running,
    Done,
}

const GOAL_COUNTS: [usize; 4] = [5, 15, 30, 50];
const TIME_LIMITS: [u32; 4] = [30, 60, 120, 300];

/// Review lessons mix other lessons' goals, so they'd only add duplicates.
fn drillable() -> impl Iterator<Item = &'static Lesson> {
    LESSONS.iter().filter(|l| !l.slug.ends_with("review"))
}

#[component]
pub fn PracticePage() -> impl IntoView {
    let selected = RwSignal::new(drillable().map(|l| l.slug.clone()).collect::<BTreeSet<_>>());
    let mode = RwSignal::new(Mode::Goals);
    let goal_count = RwSignal::new(15usize);
    let time_limit = RwSignal::new(60u32);
    let phase = RwSignal::new(Phase::Setup);
    let elapsed_ms = RwSignal::new(0.0);
    let clock = RwSignal::new(0.0);
    let ready = use_fonts_ready();
    let store = use_progress();

    let pool = move || -> Vec<Generator> {
        selected.with(|sel| drillable().filter(|l| sel.contains(&l.slug)).map(|l| l.generate).collect())
    };
    let drill = Drill::new(drillable().map(|l| l.generate).collect());

    let start = move || {
        drill.set_pool(pool());
        drill.start();
        clock.set(now_ms());
        phase.set(Phase::Running);
    };
    let finish = move || {
        if phase.get_untracked() != Phase::Running {
            return;
        }
        elapsed_ms.set(drill.elapsed_ms());
        if drill.solved.with_untracked(|s| !s.is_empty()) {
            store.finish_session();
        }
        phase.set(Phase::Done);
    };
    let check = move || {
        if drill.check() && mode.get_untracked() == Mode::Goals && drill.solved.with_untracked(Vec::len) >= goal_count.get_untracked() {
            finish();
        }
    };

    // A clock for the timer; it ends timed sessions. Effects only run in the
    // browser, so the server never starts an interval.
    Effect::new(move |_| {
        let handle = set_interval_with_handle(
            move || {
                if phase.get_untracked() != Phase::Running {
                    return;
                }
                clock.set(now_ms());
                if mode.get_untracked() == Mode::Time && drill.elapsed_ms() >= f64::from(time_limit.get_untracked()) * 1000.0 {
                    finish();
                }
            },
            std::time::Duration::from_millis(200),
        );
        on_cleanup(move || {
            if let Ok(h) = handle {
                h.clear();
            }
        });
    });

    // Enter starts a session from the setup or results screen.
    let keys = window_event_listener(ev::keydown, move |e| {
        if e.key() == "Enter" && phase.get_untracked() != Phase::Running && ready.get_untracked() {
            e.prevent_default();
            start();
        }
    });
    on_cleanup(move || keys.remove());

    // Changing settings mid-session goes back to setup.
    let reconfigure = move || phase.set(Phase::Setup);

    let remaining = move || {
        clock.track();
        (f64::from(time_limit.get()) - drill.elapsed_ms() / 1000.0).max(0.0)
    };
    let status = Signal::derive(move || {
        let n = drill.solved_count();
        match mode.get() {
            Mode::Goals => format!("goal {} / {}", n + 1, goal_count.get()),
            Mode::Time => {
                let left = remaining().ceil() as u32;
                format!("{}:{:02} left · {n} solved", left / 60, left % 60)
            }
            Mode::Endless => format!("{n} solved"),
        }
    });
    let progress = move || {
        let fraction = match mode.get() {
            Mode::Goals => drill.solved_count() as f64 / goal_count.get() as f64,
            Mode::Time => 1.0 - remaining() / f64::from(time_limit.get()),
            Mode::Endless => 0.0,
        };
        format!("width: {:.1}%", fraction * 100.0)
    };

    view! {
        <Title text="Practice" />
        <section class="practice">
            <h1>"Practice"</h1>
            <p class="muted">"Drill what you've learned until it sticks. Efficient solutions earn XP."</p>

            <div class="practice-bar">
                <div class="seg">
                    {COURSE.modules.iter().map(|m| view! {
                        <span class="seg-label">{m.title.as_str()}</span>
                        {m.lessons.iter().map(|&i| &LESSONS[i]).filter(|l| !l.slug.ends_with("review")).map(|l| {
                            let slug = l.slug.clone();
                            let slug2 = slug.clone();
                            view! {
                                <button
                                    class="seg-btn"
                                    class:on=move || selected.with(|s| s.contains(&slug))
                                    on:click=move |_| {
                                        selected.update(|s| {
                                            if !s.remove(&slug2) { s.insert(slug2.clone()); }
                                            if s.is_empty() { s.insert(slug2.clone()); }
                                        });
                                        reconfigure();
                                    }
                                >{l.title.as_str()}</button>
                            }
                        }).collect_view()}
                    }).collect_view()}
                </div>
                <div class="seg">
                    {[(Mode::Goals, "goals"), (Mode::Time, "time"), (Mode::Endless, "endless")].into_iter().map(|(m, name)| view! {
                        <button class="seg-btn" class:on=move || mode.get() == m on:click=move |_| { mode.set(m); reconfigure(); }>{name}</button>
                    }).collect_view()}
                </div>
                <Show when=move || mode.get() == Mode::Goals>
                    <div class="seg">
                        {GOAL_COUNTS.into_iter().map(|n| view! {
                            <button class="seg-btn" class:on=move || goal_count.get() == n on:click=move |_| { goal_count.set(n); reconfigure(); }>{n}</button>
                        }).collect_view()}
                    </div>
                </Show>
                <Show when=move || mode.get() == Mode::Time>
                    <div class="seg">
                        {TIME_LIMITS.into_iter().map(|t| view! {
                            <button class="seg-btn" class:on=move || time_limit.get() == t on:click=move |_| { time_limit.set(t); reconfigure(); }>
                                {if t < 60 { format!("{t}s") } else { format!("{}m", t / 60) }}
                            </button>
                        }).collect_view()}
                    </div>
                </Show>
            </div>

            <section class="challenge">
                {move || match phase.get() {
                    Phase::Setup => view! {
                        <div class="ready">
                            <p class="muted">"Pick lessons and a mode above."</p>
                            <button class="button" disabled=move || !ready.get() on:click=move |_| start()>
                                {move || if ready.get() { "▶ Start practice" } else { "Loading the Typst compiler…" }}
                            </button>
                            <p class="muted small"><kbd>"Enter"</kbd> " to start"</p>
                        </div>
                    }.into_any(),
                    Phase::Running => view! {
                        <div class="progress"><div class="bar" style=progress></div></div>
                        <DrillWorkspace drill status on_edit=Callback::new(move |_| check()) on_restart=Callback::new(move |_| start()) />
                        <div class="actions">
                            <button class="link" on:click=move |_| finish()>"■ finish now"</button>
                        </div>
                    }.into_any(),
                    Phase::Done => view! {
                        <Results drill title="Practice complete" elapsed_ms=elapsed_ms.get_untracked()>
                            <button class="button" on:click=move |_| start()>"↻ Practice again"</button>
                            <span class="muted small"><kbd>"Enter"</kbd> " to go again"</span>
                        </Results>
                    }.into_any(),
                }}
            </section>
        </section>
    }
}
