//! The Stats page: activity heatmap, streak, and every skill's level,
//! modelled on VimHero's stats page.

use leptos::prelude::*;
use leptos_meta::Title;
use typst_hero_core::lessons::TAUGHT;
use typst_hero_core::skills::{self, Band, Group, SKILLS};

use crate::progress::{Progress, use_progress};
use crate::time::{Day, today};

#[component]
pub fn StatsPage() -> impl IntoView {
    let store = use_progress();
    view! {
        <Title text="Stats" />
        <section class="stats-page">
            <h1>"Stats"</h1>
            // Saved progress and today's date only exist in the browser.
            {move || {
                if !store.loaded.get() {
                    return view! { <p class="muted">"Loading your progress…"</p> }.into_any();
                }
                let today = today();
                store.with(|p| view! { <Overview progress=p.clone() today /> }).into_any()
            }}
        </section>
    }
}

#[component]
fn Overview(progress: Progress, today: Day) -> impl IntoView {
    let count = |band: Band| SKILLS.iter().filter(|s| skills::band(progress.xp(s.id)) == band).count();
    let started = SKILLS.len() - count(Band::Unstarted);
    view! {
        <Heatmap progress=progress.clone() today />
        <p class="activity-line">
            <span class="accent">{progress.streak(today)} "-day streak"</span>
            " · " {progress.total_sessions()} " sessions"
            " · " {progress.total_goals()} " goals"
            " · " {progress.lessons_completed()} " lessons completed"
        </p>

        <div class="bands">
            <div><small>"started"</small><strong>{started}<span class="muted">"/" {SKILLS.len()}</span></strong></div>
            <div><small>"learning"</small><strong>{count(Band::Learning)}</strong><small>"below level 4"</small></div>
            <div><small>"practiced"</small><strong>{count(Band::Practiced)}</strong><small>"levels 4–5"</small></div>
            <div><small>"fluent"</small><strong>{count(Band::Fluent)}</strong><small>"levels 6–10"</small></div>
        </div>
        <p class="muted small">
            "Each skill levels up on its own. Solve a goal as efficiently as the ideal solution to earn "
            "1 xp for every skill it uses."
        </p>

        {Group::ALL.into_iter().map(|group| {
            let in_group: Vec<_> = SKILLS.iter().filter(|s| s.group == group).collect();
            let begun = in_group.iter().filter(|s| progress.xp(s.id) > 0).count();
            view! {
                <h2 class="group-title">{group.title()} <span class="muted">" · " {begun} " of " {in_group.len()}</span></h2>
                <div class="skill-table">
                    {in_group.into_iter().map(|s| view! {
                        <SkillRow id=s.id.to_string() xp=progress.xp(s.id) plus=0 />
                    }).collect_view()}
                </div>
            }
        }).collect_view()}
    }
}

/// One skill: its syntax, name, level bar, level and XP. `plus` > 0 shows
/// XP just earned (challenge summary) instead of the total.
#[component]
pub fn SkillRow(id: String, xp: u32, plus: u32) -> impl IntoView {
    let Some(skill) = skills::skill(&id) else { return ().into_any() };
    let taught = TAUGHT.contains(skill.id);
    let width = format!("width: {:.0}%", skills::level_progress(xp) * 100.0);
    view! {
        <div class="skill-row" class:untaught=!taught>
            <a class="chip small" href=skills::docs_url(skill) target="_blank" rel="noopener" title="Official docs">
                {skill.syntax}
            </a>
            <span class="skill-name">{skill.name}</span>
            <span class="level-bar"><span class="fill" style=width></span></span>
            <span class="lv">"lv " {skills::level(xp)}</span>
            {if plus > 0 {
                view! { <span class="xp">"+" {plus}</span> }.into_any()
            } else if taught {
                view! { <span class="muted">{xp} " xp"</span> }.into_any()
            } else {
                view! { <span class="soon-tag">"coming soon"</span> }.into_any()
            }}
        </div>
    }
    .into_any()
}

/// A year of activity, one square per day, like GitHub's contribution graph.
#[component]
fn Heatmap(progress: Progress, today: Day) -> impl IntoView {
    const WEEKS: i64 = 53;
    // Start on the Monday 52 weeks before this week's Monday.
    let start = today.plus(-i64::from(today.weekday()) - (WEEKS - 1) * 7);
    const MONTHS: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];

    let mut cells = Vec::new();
    let mut months = Vec::new();
    let mut last_month = 0;
    for week in 0..WEEKS {
        let monday = start.plus(week * 7);
        let (_, month, _) = monday.ymd();
        if month != last_month {
            if week < WEEKS - 2 {
                months.push((week, MONTHS[month as usize - 1]));
            }
            last_month = month;
        }
        for dow in 0..7 {
            let day = monday.plus(dow);
            if day > today {
                break;
            }
            let goals = progress.days.get(&day.key()).map_or(0, |a| a.goals);
            let level = match goals {
                0 => 0,
                1..=4 => 1,
                5..=14 => 2,
                15..=29 => 3,
                _ => 4,
            };
            cells.push((week, dow, level, format!("{goals} goals on {}", day.key())));
        }
    }

    view! {
        <div class="heatmap-scroll">
            <div class="heatmap">
                {months.into_iter().map(|(w, m)| view! {
                    <span class="month" style=format!("grid-column: {}; grid-row: 1", w + 2)>{m}</span>
                }).collect_view()}
                {[(0, "mon"), (2, "wed"), (4, "fri")].into_iter().map(|(d, name)| view! {
                    <span class="dow" style=format!("grid-column: 1; grid-row: {}", d + 2)>{name}</span>
                }).collect_view()}
                {cells.into_iter().map(|(w, d, level, title)| view! {
                    <span class=format!("cell l{level}") title=title style=format!("grid-column: {}; grid-row: {}", w + 2, d + 2)></span>
                }).collect_view()}
            </div>
        </div>
        <div class="heatmap-legend muted">
            "less" {(0..5).map(|l| view! { <span class=format!("cell l{l}")></span> }).collect_view()} "more"
        </div>
    }
}
