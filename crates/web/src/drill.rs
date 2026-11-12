//! The drill loop shared by lesson challenges and the Practice page:
//! show a target, compile the learner's code on every keystroke, and move to
//! a fresh goal the moment it matches.

use std::collections::BTreeMap;

use leptos::prelude::*;
use typst_hero_core::rng::Rng;
use typst_hero_core::{Diagnostic, Goal, Rendered, Verdict, compile_str, judge};

use crate::editor::CodeEditor;
use crate::fonts::{use_fonts_ready, use_math_ready};
use crate::progress::{ProgressStore, use_progress};
use crate::settings::use_settings;
use crate::stats::SkillRow;
use crate::time::now_ms;

pub type Generator = fn(&mut Rng) -> Goal;

/// A solved goal, kept for the results screen.
#[derive(Clone)]
pub struct Solved {
    pub goal: Goal,
    pub code: String,
    pub efficient: bool,
}

/// All state of one drill session. `Copy`, so it can be moved into any
/// closure (it's just a bundle of signal handles).
#[derive(Clone, Copy)]
pub struct Drill {
    rng: StoredValue<Rng>,
    pool: StoredValue<Vec<Generator>>,
    store: ProgressStore,
    started_at: StoredValue<f64>,
    pub goal: RwSignal<Goal>,
    pub code: RwSignal<String>,
    /// Bumped whenever the editor's text must be replaced (new goal, restart).
    pub reset: RwSignal<u64>,
    pub solved: RwSignal<Vec<Solved>>,
    /// XP earned per skill during this session.
    pub gained: RwSignal<BTreeMap<String, u32>>,
    pub target: Memo<Option<Rendered>>,
    pub output: Memo<Result<Rendered, Vec<Diagnostic>>>,
}

impl Drill {
    /// Seeded with 1 so the server and browser render the same first goal;
    /// `start` re-seeds from the clock.
    pub fn new(pool: Vec<Generator>) -> Self {
        assert!(!pool.is_empty(), "a drill needs at least one generator");
        let mut rng = Rng::new(1);
        let first = (pool[rng.below(pool.len())])(&mut rng);
        let goal = RwSignal::new(first);
        let code = RwSignal::new(String::new());
        // Recompile when fonts arrive, too (`track()`).
        let ready = use_fonts_ready();
        let math_ready = use_math_ready();
        Self {
            rng: StoredValue::new(rng),
            pool: StoredValue::new(pool),
            store: use_progress(),
            started_at: StoredValue::new(0.0),
            goal,
            code,
            reset: RwSignal::new(0),
            solved: RwSignal::new(Vec::new()),
            gained: RwSignal::new(BTreeMap::new()),
            target: Memo::new(move |_| {
                ready.track();
                math_ready.track();
                goal.with(|g| compile_str(&g.ideal).ok())
            }),
            output: Memo::new(move |_| {
                ready.track();
                math_ready.track();
                code.with(|c| compile_str(c))
            }),
        }
    }

    pub fn set_pool(&self, pool: Vec<Generator>) {
        if !pool.is_empty() {
            self.pool.set_value(pool);
        }
    }

    /// A random goal from the pool, different from `prev`.
    fn draw(&self, prev: Option<&Goal>) -> Goal {
        let pool = self.pool.get_value();
        loop {
            let goal = self
                .rng
                .try_update_value(|r| (pool[r.below(pool.len())])(r))
                .expect("drill disposed");
            if prev != Some(&goal) {
                return goal;
            }
        }
    }

    fn show(&self, goal: Goal) {
        self.code.set(goal.start.clone());
        self.goal.set(goal);
        self.reset.update(|r| *r += 1);
    }

    /// Start (or restart) a session with a fresh first goal.
    pub fn start(&self) {
        self.rng.set_value(Rng::new(now_ms() as u64));
        self.solved.set(Vec::new());
        self.gained.set(BTreeMap::new());
        self.started_at.set_value(now_ms());
        let goal = self.draw(None);
        self.show(goal);
    }

    /// Called after every edit. If the output matches the target, records
    /// the goal (and XP), shows the next one and returns true.
    pub fn check(&self) -> bool {
        let (Some(target), Ok(output)) = (self.target.get_untracked(), self.output.get_untracked()) else {
            return false;
        };
        let goal = self.goal.get_untracked();
        let code = self.code.get_untracked();
        let Verdict::Solved { efficient } = judge(&goal, &target, &output, &code) else { return false };

        let earned = self.store.solve_goal(&goal.features, efficient);
        self.gained.update(|m| earned.into_iter().for_each(|s| *m.entry(s).or_default() += 1));
        let next = self.draw(Some(&goal));
        self.solved.update(|s| s.push(Solved { goal, code, efficient }));
        self.show(next);
        true
    }

    pub fn solved_count(&self) -> usize {
        self.solved.with(Vec::len)
    }

    pub fn efficient_count(&self) -> usize {
        self.solved.with_untracked(|s| s.iter().filter(|x| x.efficient).count())
    }

    pub fn elapsed_ms(&self) -> f64 {
        now_ms() - self.started_at.get_value()
    }
}

/// Target and live output side by side, the editor, and the status line.
/// `status` is the middle of the status line ("goal 2 / 5", "0:42 left", …).
#[component]
pub fn DrillWorkspace(
    drill: Drill,
    status: Signal<String>,
    on_edit: Callback<()>,
    on_restart: Callback<()>,
) -> impl IntoView {
    let settings = use_settings();
    let vim_mode = RwSignal::new(String::new());
    let target = drill.target;
    let prompt = move || drill.goal.with(|g| g.prompt.clone());
    view! {
        {move || prompt().map(|p| view! { <div class="goal-prompt">{p}</div> })}
        <div class="panes">
            <Pane label="Target" rendered=Signal::derive(move || target.get().ok_or_else(Vec::new)) />
            <OutputPane output=drill.output />
        </div>
        <CodeEditor
            code=drill.code
            reset=drill.reset.into()
            on_edit
            vim=settings.vim()
            mode=vim_mode
            relative=settings.relative_numbers()
            on_relative=Callback::new(move |on| settings.update(|s| s.relative_numbers = on))
        />
        <div class="statusline">
            <span class="mode" data-mode=move || vim_mode.get()>
                {move || match vim_mode.get().as_str() {
                    "" => "EDIT".to_string(),
                    m => m.to_uppercase(),
                }}
            </span>
            <span>{status}</span>
            <label class="vim-toggle" title="Vim keybindings">
                <input
                    type="checkbox"
                    prop:checked=move || settings.vim().get()
                    on:change=move |ev| {
                        let on = event_target_checked(&ev);
                        settings.update(|s| s.vim = on);
                    }
                />
                "vim"
            </label>
            <Show when=move || settings.vim().get()>
                <label class="vim-toggle" title="Relative line numbers (or :set rnu)">
                    <input
                        type="checkbox"
                        prop:checked=move || settings.relative_numbers().get()
                        on:change=move |ev| {
                            let on = event_target_checked(&ev);
                            settings.update(|s| s.relative_numbers = on);
                        }
                    />
                    "rnu"
                </label>
            </Show>
            <button class="link" on:click=move |_| on_restart.run(())>"↻ restart"</button>
        </div>
    }
}

/// A labelled preview: the rendered page, or the errors.
#[component]
pub fn Pane(label: &'static str, rendered: Signal<Result<Rendered, Vec<Diagnostic>>>) -> impl IntoView {
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

/// The learner's output. While their code has an error (often just
/// half-typed markup), keep showing the last good render, faded, with the
/// error underneath, instead of flickering to an error message.
#[component]
fn OutputPane(output: Memo<Result<Rendered, Vec<Diagnostic>>>) -> impl IntoView {
    let last_good = Memo::new(move |prev: Option<&Option<Rendered>>| match output.get() {
        Ok(r) => Some(r),
        Err(_) => prev.cloned().flatten(),
    });
    view! {
        <figure class="pane">
            <figcaption>"Your output"</figcaption>
            {move || {
                let errors = output.get().err();
                let page = last_good.get().map(|r| view! {
                    <div class="page" class:stale=errors.is_some() inner_html=r.pages.join("")></div>
                });
                view! {
                    {page}
                    {errors.map(|errs| view! {
                        <ul class="errors">
                            {errs.into_iter().filter(|d| d.is_error).map(|d| view! {
                                <li>
                                    {d.position.map(|(l, c)| format!("line {l}:{c} · "))}
                                    {d.message}
                                    {d.hints.into_iter().map(|h| view! { <div class="hint">"hint: " {h}</div> }).collect_view()}
                                </li>
                            }).collect_view()}
                        </ul>
                    })}
                }
            }}
        </figure>
    }
}

/// End-of-session results: efficiency, time, XP per skill, and a review of
/// goals that had a shorter solution. `children` are the action buttons.
#[component]
pub fn Results(drill: Drill, title: &'static str, elapsed_ms: f64, children: Children) -> impl IntoView {
    let store = use_progress();
    let all = drill.solved.get_untracked();
    let gained = drill.gained.get_untracked();
    let efficient = all.iter().filter(|s| s.efficient).count();
    let inefficient: Vec<Solved> = all.iter().filter(|s| !s.efficient).cloned().collect();
    let total_xp: u32 = gained.values().sum();
    let secs = elapsed_ms / 1000.0;
    view! {
        <div class="summary">
            <h3>{title}</h3>
            <div class="stats-row">
                <div><small>"efficient goals"</small><strong>{efficient} <span class="muted">"/" {all.len()}</span></strong></div>
                <div><small>"time"</small><strong>{format!("{secs:.1}s")}</strong></div>
                <div><small>"xp"</small><strong class="xp">"+" {total_xp}</strong></div>
            </div>
            {(!gained.is_empty()).then(|| view! {
                <div class="skill-table">
                    {gained.into_iter().map(|(id, plus)| {
                        let xp = store.with(|p| p.xp(&id));
                        view! { <SkillRow id xp plus /> }
                    }).collect_view()}
                </div>
            })}
            {(!inefficient.is_empty()).then(|| view! {
                <p class="muted">{inefficient.len()} " goal(s) were not efficient enough to earn XP."</p>
                <h4>"Review: shorter solutions exist"</h4>
                {inefficient.into_iter().map(|s| view! {
                    <div class="review">
                        <div><span class="tag ideal">"ideal"</span><code>{s.goal.ideal.clone()}</code></div>
                        <div><span class="tag yours">"yours"</span><code>{s.code}</code></div>
                    </div>
                }).collect_view()}
            })}
            <div class="actions">{children()}</div>
        </div>
    }
}
