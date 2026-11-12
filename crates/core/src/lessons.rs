//! Lesson content. Each lesson is plain data plus a goal generator.
//! (Later milestones move the prose into files under `content/`.)

use crate::exercise::{Chip, Goal, Lesson};
use crate::rng::Rng;

pub static LESSONS: &[Lesson] = &[
    Lesson {
        slug: "emphasis",
        title: "Bold and Italic",
        module: "Writing",
        intro: &[
            "Typst documents start out as plain text. You only add markup where you need it.",
            "Wrap a word in stars to make it strong (bold), or in underscores to emphasize it (italic).",
        ],
        chips: &[
            Chip { syntax: "*text*", meaning: "strong emphasis (bold)", example: "This is *important*." },
            Chip { syntax: "_text_", meaning: "emphasis (italic)", example: "I _really_ mean it." },
            Chip {
                syntax: "*_text_*",
                meaning: "both at once",
                example: "Read the *_entire_* manual.",
            },
        ],
        task: &[
            "Make your code render exactly like the target.",
            "Add only the markup you need: shorter solutions earn XP.",
        ],
        goals: 6,
        generate: gen_emphasis,
    },
    Lesson {
        slug: "headings",
        title: "Headings",
        module: "Writing",
        intro: &[
            "Start a line with `=` and a space to turn it into a heading.",
            "More equals signs make deeper levels: `==` is a subsection, `===` a sub-subsection.",
        ],
        chips: &[
            Chip { syntax: "= Title", meaning: "level 1 heading", example: "= Introduction\nWe begin here." },
            Chip { syntax: "== Title", meaning: "level 2 heading", example: "== Background\nSome context." },
            Chip { syntax: "=== Title", meaning: "level 3 heading", example: "=== Details\nThe fine print." },
        ],
        task: &[
            "Turn the right lines into headings of the right level.",
            "Compare the size of each heading with the target.",
        ],
        goals: 5,
        generate: gen_headings,
    },
    Lesson {
        slug: "lists",
        title: "Lists",
        module: "Writing",
        intro: &[
            "Start a line with `-` for a bullet list, or `+` for a numbered list.",
            "Typst numbers `+` items for you, so you never renumber by hand.",
        ],
        chips: &[
            Chip { syntax: "- item", meaning: "bullet list item", example: "- milk\n- eggs" },
            Chip { syntax: "+ item", meaning: "numbered list item", example: "+ preheat\n+ bake" },
        ],
        task: &[
            "Turn the lines into the kind of list shown in the target.",
            "Each item goes on its own line.",
        ],
        goals: 5,
        generate: gen_lists,
    },
];

pub fn lesson(slug: &str) -> Option<(usize, &'static Lesson)> {
    LESSONS.iter().enumerate().find(|(_, l)| l.slug == slug)
}

// ---------------------------------------------------------------- generators

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

fn gen_emphasis(rng: &mut Rng) -> Goal {
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

fn gen_headings(rng: &mut Rng) -> Goal {
    let (top, subs) = rng.pick(TOPICS);
    let deep = rng.below(2) == 0;
    let mut start = vec![top.to_string(), rng.pick(BODIES).to_string()];
    let mut ideal = vec![format!("= {top}"), start[1].clone()];
    let mut features = vec!["heading"];
    for (i, sub) in subs.iter().enumerate() {
        let level = if deep && i == 1 { "===" } else { "==" };
        let body = rng.pick(BODIES).to_string();
        start.push(sub.to_string());
        start.push(body.clone());
        ideal.push(format!("{level} {sub}"));
        ideal.push(body);
    }
    if deep {
        features.push("heading-levels");
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

fn gen_lists(rng: &mut Rng) -> Goal {
    let pool = rng.pick(LIST_POOLS);
    let count = 3 + rng.below(2);
    // Random items, kept in their natural order (recipes stay sensible).
    let items: Vec<&str> = rng.distinct(pool.len(), count).into_iter().map(|i| pool[i]).collect();
    let (marker, feature) = if rng.below(2) == 0 { ("-", "list") } else { ("+", "enum") };
    let start = items.join("\n\n");
    let ideal = items.iter().map(|i| format!("{marker} {i}")).collect::<Vec<_>>().join("\n");
    Goal { start, ideal, features: vec![feature] }
}
