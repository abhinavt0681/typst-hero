//! Goal generators: Rust functions that build a fresh, randomized goal.
//! Lesson files (`content/lessons/*.toml`) refer to them by name.

use crate::exercise::Goal;
use crate::rng::Rng;

/// Finds a generator by the name used in lesson files.
pub fn by_name(name: &str) -> Option<fn(&mut Rng) -> Goal> {
    Some(match name {
        "emphasis" => emphasis,
        "headings" => headings,
        "lists" => lists,
        _ => return None,
    })
}


const SENTENCES: &[&str] = &[
    "the quick brown fox jumps over the lazy dog",
    "always back up your thesis before the deadline",
    "typst compiles documents in milliseconds",
    "never trust a spreadsheet you did not check",
    "good figures make a paper easy to read",
    "the meeting moved to thursday afternoon",
    "please submit the final report by friday",
    "small habits compound into large results",
];

pub fn emphasis(rng: &mut Rng) -> Goal {
    let words: Vec<&str> = rng.pick(SENTENCES).split(' ').collect();
    let count = 1 + rng.below(2);
    let marked = rng.distinct(words.len(), count);
    let mut features = Vec::new();
    let ideal: Vec<String> = words
        .iter()
        .enumerate()
        .map(|(i, w)| {
            if !marked.contains(&i) {
                return w.to_string();
            }
            match rng.below(5) {
                0..=1 => {
                    features.push("strong");
                    format!("*{w}*")
                }
                2..=3 => {
                    features.push("emph");
                    format!("_{w}_")
                }
                _ => {
                    features.extend(["strong", "emph"]);
                    format!("*_{w}_*")
                }
            }
        })
        .collect();
    features.sort_unstable();
    features.dedup();
    Goal { start: words.join(" "), ideal: ideal.join(" "), features }
}

const TOPICS: &[(&str, &[&str])] = &[
    ("Methods", &["Data Collection", "Analysis"]),
    ("Results", &["Accuracy", "Speed"]),
    ("Background", &["Prior Work", "Motivation"]),
    ("Recipe", &["Ingredients", "Steps"]),
    ("Experience", &["Acme Corp", "Globex"]),
];

const BODIES: &[&str] = &[
    "Some text goes here.",
    "We describe this below.",
    "This part is short.",
    "More details follow.",
];

pub fn headings(rng: &mut Rng) -> Goal {
    let (top, subs) = rng.pick(TOPICS);
    let deep = rng.below(2) == 0;
    let mut start = vec![top.to_string(), rng.pick(BODIES).to_string()];
    let mut ideal = vec![format!("= {top}"), start[1].clone()];
    let features = vec!["heading"];
    for (i, sub) in subs.iter().enumerate() {
        let level = if deep && i == 1 { "===" } else { "==" };
        let body = rng.pick(BODIES).to_string();
        start.push(sub.to_string());
        start.push(body.clone());
        ideal.push(format!("{level} {sub}"));
        ideal.push(body);
    }
    // Blank lines keep each line its own paragraph in the starting code.
    Goal { start: start.join("\n\n"), ideal: ideal.join("\n\n"), features }
}

const LIST_POOLS: &[&[&str]] = &[
    &["milk", "eggs", "bread", "coffee", "butter", "apples", "rice", "cheese"],
    &["clone the repo", "install typst", "write the report", "export the pdf", "send it for review"],
    &["preheat the oven", "mix the batter", "pour into the tin", "bake for twenty minutes", "let it cool"],
    &["Rust", "Typst", "Postgres", "Neovim", "Linux", "Git"],
    &["read the brief", "sketch ideas", "draft the layout", "collect feedback", "polish"],
];

pub fn lists(rng: &mut Rng) -> Goal {
    let pool = rng.pick(LIST_POOLS);
    let count = 3 + rng.below(2);
    // Random items, kept in their natural order (recipes stay sensible).
    let items: Vec<&str> = rng.distinct(pool.len(), count).into_iter().map(|i| pool[i]).collect();
    let (marker, feature) = if rng.below(2) == 0 { ("-", "list") } else { ("+", "enum") };
    let start = items.join("\n\n");
    let ideal = items.iter().map(|i| format!("{marker} {i}")).collect::<Vec<_>>().join("\n");
    Goal { start, ideal, features: vec![feature] }
}
