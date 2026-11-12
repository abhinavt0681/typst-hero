//! Lessons, goals and how answers are judged.

use crate::render::Rendered;
use crate::rng::Rng;

/// A syntax "chip" row in a lesson, like VimHero's `[d][w] - delete a word`.
pub struct Chip {
    /// The syntax shown on the chip, e.g. `*text*`.
    pub syntax: &'static str,
    /// What it does, e.g. "strong emphasis (bold)".
    pub meaning: &'static str,
    /// A short Typst example, rendered next to the source.
    pub example: &'static str,
}

pub struct Lesson {
    pub slug: &'static str,
    pub title: &'static str,
    pub module: &'static str,
    /// Introductory paragraphs. Text between backticks is shown as code.
    pub intro: &'static [&'static str],
    pub chips: &'static [Chip],
    /// Instructions for the challenge ("Now it's your turn!").
    pub task: &'static [&'static str],
    /// How many goals complete the challenge.
    pub goals: usize,
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The output doesn't match the target yet.
    NotYet,
    /// Matches the target. `efficient` = at least as concise as the ideal.
    Solved { efficient: bool },
}

/// Length used for the efficiency comparison: whitespace runs count as one
/// character and surrounding whitespace is ignored, so formatting choices
/// never cost XP. Only the markup you actually chose matters.
pub fn effective_len(code: &str) -> usize {
    let mut len = 0;
    let mut in_space = false;
    for c in code.trim().chars() {
        if c.is_whitespace() {
            if !in_space {
                len += 1;
            }
            in_space = true;
        } else {
            len += 1;
            in_space = false;
        }
    }
    len
}

pub fn judge(target: &Rendered, attempt: &Rendered, attempt_code: &str, ideal_code: &str) -> Verdict {
    if !attempt.looks_like(target) {
        return Verdict::NotYet;
    }
    Verdict::Solved { efficient: effective_len(attempt_code) <= effective_len(ideal_code) }
}
