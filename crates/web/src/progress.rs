//! The learner's progress, saved in the browser (localStorage).
//!
//! Guests need no account: progress lives in their browser. When accounts
//! arrive (milestone 4), this is what gets synced to the server.

use std::collections::BTreeMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::time::{Day, today};

#[cfg(feature = "hydrate")]
const STORAGE_KEY: &str = "typst-hero/progress/v1";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Progress {
    pub lessons: BTreeMap<String, LessonProgress>,
    /// XP per skill id (see `typst_hero_core::skills`).
    pub skills: BTreeMap<String, u32>,
    /// Activity per local calendar day ("2026-11-13"), for the heatmap and streak.
    pub days: BTreeMap<String, Activity>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LessonProgress {
    pub runs: Vec<Run>,
}

/// One completed challenge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Run {
    /// Time taken, in milliseconds.
    pub ms: f64,
    pub efficient: usize,
    pub goals: usize,
    /// When it finished (milliseconds since the Unix epoch).
    pub at: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Activity {
    pub goals: u32,
    pub sessions: u32,
}

impl Progress {
    pub fn runs(&self, slug: &str) -> &[Run] {
        self.lessons.get(slug).map_or(&[], |l| &l.runs)
    }

    pub fn completed(&self, slug: &str) -> bool {
        !self.runs(slug).is_empty()
    }

    pub fn xp(&self, skill: &str) -> u32 {
        self.skills.get(skill).copied().unwrap_or(0)
    }

    /// A goal was solved on `day`. Like VimHero, only efficient solutions
    /// earn XP: +1 for every skill the goal exercises. Returns the skills
    /// that gained XP.
    pub fn solve_goal(&mut self, day: Day, skills: &[&str], efficient: bool) -> Vec<String> {
        self.days.entry(day.key()).or_default().goals += 1;
        if !efficient {
            return Vec::new();
        }
        skills
            .iter()
            .map(|s| {
                *self.skills.entry(s.to_string()).or_default() += 1;
                s.to_string()
            })
            .collect()
    }

    /// A practice session ended (no lesson attached).
    pub fn finish_session(&mut self, day: Day) {
        self.days.entry(day.key()).or_default().sessions += 1;
    }

    pub fn finish_run(&mut self, day: Day, slug: &str, run: Run) {
        self.days.entry(day.key()).or_default().sessions += 1;
        self.lessons.entry(slug.to_string()).or_default().runs.push(run);
    }

    /// Consecutive active days ending today. A streak survives until the end
    /// of the day after the last activity, so an unfinished today doesn't
    /// break it.
    pub fn streak(&self, today: Day) -> u32 {
        let active = |d: Day| self.days.get(&d.key()).is_some_and(|a| a.goals > 0);
        let mut day = if active(today) { today } else { today.plus(-1) };
        let mut count = 0;
        while active(day) {
            count += 1;
            day = day.plus(-1);
        }
        count
    }

    pub fn total_goals(&self) -> u32 {
        self.days.values().map(|a| a.goals).sum()
    }

    pub fn total_sessions(&self) -> u32 {
        self.days.values().map(|a| a.sessions).sum()
    }

    pub fn lessons_completed(&self) -> usize {
        self.lessons.values().filter(|l| !l.runs.is_empty()).count()
    }
}

#[derive(Clone, Copy)]
pub struct ProgressStore {
    progress: RwSignal<Progress>,
    /// False until saved progress has been read (browser only). Pages that
    /// depend on the date or on saved data wait for this.
    pub loaded: RwSignal<bool>,
}

impl ProgressStore {
    pub fn with<T>(&self, f: impl FnOnce(&Progress) -> T) -> T {
        self.progress.with(f)
    }

    fn update<T>(&self, f: impl FnOnce(&mut Progress) -> T) -> T {
        let mut out = None;
        self.progress.update(|p| out = Some(f(p)));
        save(&self.progress.get_untracked());
        out.expect("update ran")
    }

    pub fn solve_goal(&self, skills: &[&str], efficient: bool) -> Vec<String> {
        self.update(|p| p.solve_goal(today(), skills, efficient))
    }

    pub fn finish_run(&self, slug: &str, run: Run) {
        self.update(|p| p.finish_run(today(), slug, run));
    }

    pub fn finish_session(&self) {
        self.update(|p| p.finish_session(today()));
    }
}

/// Call once at the top of the app. Starts empty (on the server and during
/// hydration), then loads saved progress in the browser.
pub fn provide_progress() {
    let progress = RwSignal::new(Progress::default());
    let loaded = RwSignal::new(false);
    provide_context(ProgressStore { progress, loaded });

    // Effects only run in the browser, after hydration.
    Effect::new(move |_| {
        if let Some(saved) = load() {
            progress.set(saved);
        }
        loaded.set(true);
    });
}

pub fn use_progress() -> ProgressStore {
    expect_context::<ProgressStore>()
}

#[cfg(feature = "hydrate")]
fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

#[cfg(feature = "hydrate")]
fn load() -> Option<Progress> {
    let json = storage()?.get_item(STORAGE_KEY).ok()??;
    serde_json::from_str(&json).ok()
}

#[cfg(feature = "hydrate")]
fn save(progress: &Progress) {
    if let (Some(storage), Ok(json)) = (storage(), serde_json::to_string(progress)) {
        // Fails only in private mode or when storage is full; progress then
        // simply isn't kept.
        let _ = storage.set_item(STORAGE_KEY, &json);
    }
}

#[cfg(not(feature = "hydrate"))]
fn load() -> Option<Progress> {
    None
}

#[cfg(not(feature = "hydrate"))]
fn save(_: &Progress) {}

#[cfg(test)]
mod tests {
    use super::*;

    const D: Day = Day::from_ymd(2026, 11, 13);

    #[test]
    fn progress_round_trips_through_json() {
        let mut p = Progress::default();
        p.finish_run(D, "emphasis", Run { ms: 12_345.0, efficient: 5, goals: 6, at: 1.0 });
        p.solve_goal(D, &["strong"], true);
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(serde_json::from_str::<Progress>(&json).unwrap(), p);
        assert!(p.completed("emphasis"));
        assert!(!p.completed("lists"));
    }

    #[test]
    fn old_saves_still_load() {
        // Saved before skills/days existed.
        let old = r#"{"lessons":{"lists":{"runs":[{"ms":1.0,"efficient":5,"goals":5,"at":1.0}]}}}"#;
        let p: Progress = serde_json::from_str(old).unwrap();
        assert!(p.completed("lists"));
        assert_eq!(p.xp("list"), 0);
    }

    #[test]
    fn only_efficient_goals_earn_xp() {
        let mut p = Progress::default();
        assert_eq!(p.solve_goal(D, &["strong", "emph"], true), ["strong", "emph"]);
        assert!(p.solve_goal(D, &["strong"], false).is_empty());
        assert_eq!(p.xp("strong"), 1);
        assert_eq!(p.total_goals(), 2);
    }

    #[test]
    fn streak_counts_consecutive_days() {
        let mut p = Progress::default();
        assert_eq!(p.streak(D), 0);
        p.solve_goal(D.plus(-1), &[], false);
        p.solve_goal(D.plus(-2), &[], false);
        p.solve_goal(D.plus(-4), &[], false);
        assert_eq!(p.streak(D), 2, "today not done yet: streak still alive");
        p.solve_goal(D, &[], false);
        assert_eq!(p.streak(D), 3);
        assert_eq!(p.streak(D.plus(2)), 0, "a missed day breaks it");
    }
}
