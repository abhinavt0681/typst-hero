//! Lessons, goals and how answers are judged.

use crate::render::Rendered;
use crate::rng::Rng;

/// A syntax "chip" row in a lesson, like VimHero's `[d][w] - delete a word`.
pub struct Chip {
    /// The syntax shown on the chip, e.g. `*text*`.
    pub syntax: String,
    /// What it does, e.g. "strong emphasis (bold)".
    pub meaning: String,
    /// A short Typst example, typed out live next to its rendering.
    pub example: String,
    /// The skill this chip teaches (links to its docs page).
    pub skill: Option<String>,
}

/// A lesson, loaded from `content/lessons/<slug>.toml`.
pub struct Lesson {
    pub slug: String,
    pub title: String,
    /// Module id from `content/course.toml`.
    pub module: String,
    /// Introductory paragraphs. Text between backticks is shown as code.
    pub intro: Vec<String>,
    pub chips: Vec<Chip>,
    /// Instructions for the challenge ("Now it's your turn!").
    pub task: Vec<String>,
    /// How many goals complete the challenge.
    pub goals: usize,
    /// Goals contain math, so the math font must be loaded.
    pub math: bool,
    /// Builds a fresh, randomized goal.
    pub generate: fn(&mut Rng) -> Goal,
}

/// One thing to achieve: turn `start` into code that renders like `ideal`.
#[derive(Clone, Debug, PartialEq)]
pub struct Goal {
    /// The code the editor starts with.
    pub start: String,
    /// The reference solution. Its rendering is the target.
    pub ideal: String,
    /// Features the ideal solution uses; they earn XP when solved efficiently.
    pub features: Vec<&'static str>,
    /// Text that must still appear in the learner's code. Used where the
    /// output alone can't tell the right technique from a shortcut, e.g. a
    /// commented-out line (vs. deleting it) or `--` (vs. pasting "–").
    pub require: Vec<String>,
    /// Extra instructions shown with this goal, e.g. LaTeX to translate.
    pub prompt: Option<String>,
}

impl Goal {
    pub fn new(start: impl Into<String>, ideal: impl Into<String>, features: Vec<&'static str>) -> Self {
        Self { start: start.into(), ideal: ideal.into(), features, require: Vec::new(), prompt: None }
    }

    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn requiring(mut self, require: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.require = require.into_iter().map(Into::into).collect();
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The output doesn't match the target yet.
    NotYet,
    /// Matches the target. `efficient` = at least as concise as the ideal.
    Solved { efficient: bool },
}

/// Length used for the efficiency comparison: non-whitespace characters
/// only. Whitespace that changes the output is already enforced by the
/// output having to match, so spacing like `(a + b)/2` vs `(a+b)/2` never
/// costs XP. Only the markup you actually chose matters.
pub fn effective_len(code: &str) -> usize {
    code.chars().filter(|c| !c.is_whitespace()).count()
}

pub fn judge(goal: &Goal, target: &Rendered, attempt: &Rendered, attempt_code: &str) -> Verdict {
    if !attempt.looks_like(target) || !goal.require.iter().all(|r| attempt_code.contains(r.as_str())) {
        return Verdict::NotYet;
    }
    Verdict::Solved { efficient: effective_len(attempt_code) <= effective_len(&goal.ideal) }
}
