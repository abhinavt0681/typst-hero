//! The learner's progress, saved in the browser (localStorage).
//!
//! Guests need no account: progress lives in their browser. When accounts
//! arrive (milestone 4), this is what gets synced to the server.

use std::collections::BTreeMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "hydrate")]
const STORAGE_KEY: &str = "typst-hero/progress/v1";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    pub lessons: BTreeMap<String, LessonProgress>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

impl Progress {
    pub fn runs(&self, slug: &str) -> &[Run] {
        self.lessons.get(slug).map_or(&[], |l| &l.runs)
    }

    pub fn completed(&self, slug: &str) -> bool {
        !self.runs(slug).is_empty()
    }
}

#[derive(Clone, Copy)]
pub struct ProgressStore(RwSignal<Progress>);

impl ProgressStore {
    pub fn with<T>(&self, f: impl FnOnce(&Progress) -> T) -> T {
        self.0.with(f)
    }

    pub fn record(&self, slug: &str, run: Run) {
        self.0.update(|p| p.lessons.entry(slug.to_string()).or_default().runs.push(run));
        save(&self.0.get_untracked());
    }
}

/// Call once at the top of the app. Starts empty (on the server and during
/// hydration), then loads saved progress in the browser.
pub fn provide_progress() {
    let progress = RwSignal::new(Progress::default());
    provide_context(ProgressStore(progress));

    // Effects only run in the browser, after hydration.
    Effect::new(move |_| {
        if let Some(saved) = load() {
            progress.set(saved);
        }
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

    #[test]
    fn progress_round_trips_through_json() {
        let mut p = Progress::default();
        p.lessons.entry("emphasis".into()).or_default().runs.push(Run {
            ms: 12_345.0,
            efficient: 5,
            goals: 6,
            at: 1.0,
        });
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(serde_json::from_str::<Progress>(&json).unwrap(), p);
        assert!(p.completed("emphasis"));
        assert!(!p.completed("lists"));
    }
}
