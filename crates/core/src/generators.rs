//! Goal generators: Rust functions that build a fresh, randomized goal.
//! Lesson files (`content/lessons/*.toml`) refer to them by name.

use crate::exercise::Goal;
use crate::rng::Rng;

mod functions;
mod math;

/// Finds a generator by the name used in lesson files.
pub fn by_name(name: &str) -> Option<fn(&mut Rng) -> Goal> {
    Some(match name {
        "emphasis" => emphasis,
        "headings" => headings,
        "lists" => lists,
        "raw" => raw,
        "escapes" => escapes,
        "comments" => comments,
        "terms" => terms,
        "linebreaks" => linebreaks,
        "dashes" => dashes,
        "writing-review" => writing_review,
        "text-functions" => functions::text_functions,
        "arguments" => functions::arguments,
        "images" => functions::images,
        "figures" => functions::figures,
        "tables" => functions::tables,
        "spacing" => functions::spacing,
        "functions-review" => functions::review,
        "math-basics" => math::basics,
        "math-scripts" => math::scripts,
        "math-symbols" => math::symbols,
        "math-functions" => math::functions,
        "math-alignment" => math::alignment,
        "latex" => math::latex,
        "math-review" => math::review,
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
    Goal::new(words.join(" "), ideal.join(" "), features)
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
    Goal::new(start.join("\n\n"), ideal.join("\n\n"), features)
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
    Goal::new(start, ideal, vec![feature])
}

// ------------------------------------------------------------------ raw text

/// (before, code, after): the code part goes in backticks.
const RAW_SENTENCES: &[(&str, &str, &str)] = &[
    ("run", "cargo build", "before you push"),
    ("the", "main", "function starts the program"),
    ("call", "len()", "to count the items"),
    ("set", "x = 1", "at the top of the file"),
    ("type", "git status", "to see your changes"),
    ("open", "config.toml", "and change the port"),
    ("use", "typst watch", "while you write"),
    ("the", "print", "statement is for debugging"),
    ("install it with", "pip install numpy", "first"),
    ("press", "Ctrl+S", "to save often"),
];

pub fn raw(rng: &mut Rng) -> Goal {
    let (before, code, after) = *rng.pick(RAW_SENTENCES);
    let lead = rng.pick(&["", "first ", "then ", "always ", "please ", "now "]);
    let start = format!("{lead}{before} {code} {after}");
    let ideal = format!("{lead}{before} `{code}` {after}");
    Goal::new(start, ideal, vec!["raw"])
}

// ------------------------------------------------------------------- escapes

const LEADS: &[&str] = &["note:", "remember,", "in the report", "as agreed", "today", "honestly"];
const WORDS: &[&str] = &["draft", "final", "pending", "urgent", "optional", "secret", "beta"];

/// Text that *looks* like markup but should appear literally.
pub fn escapes(rng: &mut Rng) -> Goal {
    let lead = rng.pick(LEADS);
    let word = rng.pick(WORDS);
    let (start, ideal) = match rng.below(4) {
        0 => (format!("{lead} the *{word}* label stays"), format!("{lead} the \\*{word}\\* label stays")),
        1 => (format!("{lead} the file is _{word}_ now"), format!("{lead} the file is \\_{word}\\_ now")),
        2 => {
            let (a, b) = (1 + rng.below(9), 10 + rng.below(90));
            (format!("{lead} it costs ${a} or ${b}"), format!("{lead} it costs \\${a} or \\${b}"))
        }
        _ => (format!("{lead} wrap it in `{word}` ticks"), format!("{lead} wrap it in \\`{word}\\` ticks")),
    };
    Goal::new(start, ideal, vec!["escape"])
}

// ------------------------------------------------------------------ comments

const NOTES: &[&str] = &[
    "The results are in.",
    "TODO check these numbers",
    "Our method beats the baseline.",
    "FIXME this paragraph is too long",
    "We thank the reviewers.",
    "note to self: cite Smith here",
    "The data covers ten years.",
    "remove before submitting",
];

/// Hide lines from the output without deleting them.
pub fn comments(rng: &mut Rng) -> Goal {
    let lines: Vec<&str> = rng.distinct(NOTES.len(), 4).into_iter().map(|i| NOTES[i]).collect();
    let count = 1 + rng.below(2);
    let hidden = rng.distinct(lines.len(), count);
    let ideal: Vec<String> = lines
        .iter()
        .enumerate()
        .map(|(i, l)| if hidden.contains(&i) { format!("// {l}") } else { l.to_string() })
        .collect();
    let required: Vec<&str> = hidden.iter().map(|&i| lines[i]).collect();
    Goal::new(lines.join("\n\n"), ideal.join("\n\n"), vec!["comment"]).requiring(required)
}

// ---------------------------------------------------------------- term lists

const GLOSSARIES: &[&[(&str, &str)]] = &[
    &[
        ("Typst", "a markup-based typesetting system"),
        ("LaTeX", "a typesetting system from 1984"),
        ("Markdown", "a lightweight markup language"),
        ("PDF", "a fixed-layout document format"),
    ],
    &[
        ("Rust", "a systems programming language"),
        ("Cargo", "the Rust build tool"),
        ("crate", "a Rust package"),
        ("WASM", "a binary format browsers run"),
    ],
    &[
        ("Abstract", "a short summary of the paper"),
        ("Thesis", "the claim you defend"),
        ("Citation", "a pointer to a source"),
        ("Appendix", "extra material at the end"),
    ],
    &[
        ("Margin", "space around the text"),
        ("Leading", "space between lines"),
        ("Kerning", "space between letter pairs"),
        ("Serif", "a small stroke on a letter"),
    ],
    &[
        ("Commit", "a saved snapshot"),
        ("Branch", "a line of development"),
        ("Merge", "joining two branches"),
        ("Remote", "a copy on a server"),
    ],
];

pub fn terms(rng: &mut Rng) -> Goal {
    let glossary = rng.pick(GLOSSARIES);
    let count = 2 + rng.below(2);
    let picked: Vec<_> = rng.distinct(glossary.len(), count).into_iter().map(|i| glossary[i]).collect();
    let start = picked.iter().map(|(t, d)| format!("{t}: {d}")).collect::<Vec<_>>().join("\n\n");
    let ideal = picked.iter().map(|(t, d)| format!("/ {t}: {d}")).collect::<Vec<_>>().join("\n");
    Goal::new(start, ideal, vec!["terms"])
}

// --------------------------------------------------------------- line breaks

const BLOCKS: &[&[&str]] = &[
    &["Jane Doe", "12 Garden Lane", "Springfield 4021"],
    &["Dr. Amal Rao", "Dept. of Physics", "North University"],
    &["Kind regards,", "Sam Lee", "Product Team"],
    &["Acme Corp", "Invoice 2026-114", "Due in 30 days"],
    &["Mon: write", "Tue: review", "Wed: submit"],
    &["Room 4.12", "Building C", "Main Campus"],
    &["Yours truly,", "Alex Kim"],
    &["Order 5531", "Shipped today", "Arrives Friday"],
];

/// Short lines need `\`, not a blank line: a blank line would start a new
/// paragraph, with extra space between.
pub fn linebreaks(rng: &mut Rng) -> Goal {
    let first = rng.pick(BLOCKS);
    let two = rng.below(2) == 0;
    let mut start = vec![first.join(" ")];
    let mut ideal = vec![first.join(" \\\n")];
    let mut features = vec!["linebreak"];
    if two {
        let second = loop {
            let b = rng.pick(BLOCKS);
            if b != first {
                break b;
            }
        };
        start.push(second.join(" "));
        ideal.push(second.join(" \\\n"));
        features.push("parbreak");
    }
    Goal::new(start.join("\n\n"), ideal.join("\n\n"), features)
}

// -------------------------------------------------------------------- dashes

const SUBJECTS: &[&str] = &["the plan", "the draft", "our budget", "the survey", "this method"];
const ASIDES: &[&str] = &["surprisingly", "for now", "in theory", "mostly", "as expected"];

/// `--` is an en dash (ranges), `---` an em dash (asides).
pub fn dashes(rng: &mut Rng) -> Goal {
    if rng.below(2) == 0 {
        let what = rng.pick(&["read pages", "see chapters", "the years", "open from"]);
        let (a, len) = if *what == "the years" { (1990 + rng.below(30), 1 + rng.below(9)) } else { (1 + rng.below(40), 2 + rng.below(20)) };
        let b = a + len;
        Goal::new(format!("{what} {a} - {b}"), format!("{what} {a}--{b}"), vec!["shorthand"]).requiring(["--"])
    } else {
        let (topic, aside) = (rng.pick(SUBJECTS), rng.pick(ASIDES));
        let start = format!("{topic} - {aside} - worked");
        let ideal = format!("{topic}---{aside}---worked");
        Goal::new(start, ideal, vec!["shorthand"]).requiring(["---"])
    }
}

// -------------------------------------------------------------------- review

/// A mix of everything in the Writing module.
pub fn writing_review(rng: &mut Rng) -> Goal {
    const ALL: &[fn(&mut Rng) -> Goal] = &[emphasis, raw, escapes, comments, headings, lists, terms, linebreaks, dashes];
    (rng.pick(ALL))(rng)
}
