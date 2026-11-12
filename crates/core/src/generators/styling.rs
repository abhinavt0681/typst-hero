//! Goals for the Styling module: set rules, show-set rules, show rules and
//! style-guide checklists.
//!
//! Every goal is "here is your document; make it look like the target".
//! Documents have several headings and emphasized words, so a single rule is
//! always shorter than styling each one by hand, and `require` makes sure a
//! rule is actually used.

use crate::exercise::Goal;
use crate::rng::Rng;

const HEADINGS: &[&str] = &[
    "Introduction",
    "Background",
    "Method",
    "Results",
    "Discussion",
    "Conclusion",
    "Outlook",
];

const SENTENCES: &[&str] = &[
    "Glaciers move slowly but they shape entire valleys over time.",
    "We measured the flow at twelve sites during two summers.",
    "The data show a clear seasonal pattern in every location.",
    "Warmer years produced noticeably faster movement downhill.",
    "Our model explains most of the variation we observed.",
    "Some sites behaved differently and need a closer look.",
    "Future work should cover the winter months as well.",
    "These findings matter for planning water supplies downstream.",
];

/// A paragraph of 2–3 sentences, optionally with one *strong* and one
/// _emph_ word so that show rules have something to act on.
fn paragraph(rng: &mut Rng, marked: bool) -> String {
    let count = 2 + rng.below(2);
    let picks = rng.distinct(SENTENCES.len(), count);
    let mut text = picks
        .into_iter()
        .map(|i| SENTENCES[i])
        .collect::<Vec<_>>()
        .join(" ");
    if marked {
        text = text
            .replacen("clear", "*clear*", 1)
            .replacen("slowly", "_slowly_", 1);
        for (plain, strong) in [
            ("twelve", "*twelve*"),
            ("faster", "*faster*"),
            ("most", "*most*"),
        ] {
            text = text.replacen(plain, strong, 1);
        }
        for (plain, emph) in [
            ("closer", "_closer_"),
            ("winter", "_winter_"),
            ("water", "_water_"),
        ] {
            text = text.replacen(plain, emph, 1);
        }
        // Guarantee something for `strong` and `emph` show rules to act on.
        if !text.contains('*') {
            text.push_str(" The effect is *large*.");
        }
        if !text.contains('_') {
            text.push_str(" It is _not_ uniform.");
        }
    }
    text
}

/// A short document: `sections` headings, each followed by a paragraph.
/// `levels` mixes in `==` subheadings.
fn document(rng: &mut Rng, sections: usize, levels: bool, marked: bool) -> String {
    let heads = rng.distinct(HEADINGS.len(), sections);
    heads
        .into_iter()
        .enumerate()
        .map(|(i, h)| {
            let marker = if levels && i % 2 == 1 { "==" } else { "=" };
            format!("{marker} {}\n\n{}", HEADINGS[h], paragraph(rng, marked))
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Plain paragraphs, no headings.
fn paragraphs(rng: &mut Rng, count: usize) -> String {
    (0..count)
        .map(|_| paragraph(rng, false))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn styled(rules: &[&str], doc: &str) -> String {
    format!("{}\n\n{doc}", rules.join("\n"))
}

fn goal(doc: String, rules: &[&str], features: Vec<&'static str>) -> Goal {
    let keyword = if rules.iter().any(|r| r.starts_with("#show")) {
        "#show"
    } else {
        "#set"
    };
    Goal::new(doc.clone(), styled(rules, &doc), features).requiring([keyword])
}

/// `#set text(…)`: color, size or weight for the whole document.
pub fn set_text(rng: &mut Rng) -> Goal {
    let rule = *rng.pick(&[
        "#set text(blue)",
        "#set text(red)",
        "#set text(eastern)",
        "#set text(purple)",
        "#set text(11pt)",
        "#set text(15pt)",
        "#set text(blue, 15pt)",
        "#set text(red, 11pt)",
    ]);
    let doc = document(rng, 2, false, false);
    let mut features = vec!["set"];
    if rule.contains("pt") {
        features.push("length");
    }
    goal(doc, &[rule], features)
}

/// `#set par(…)`: justification, line spacing, indents.
pub fn set_par(rng: &mut Rng) -> Goal {
    let rule = *rng.pick(&[
        "#set par(justify: true)",
        "#set par(leading: 1.2em)",
        "#set par(first-line-indent: 1.5em)",
        "#set par(spacing: 2em)",
        "#set par(justify: true, leading: 1em)",
        "#set par(justify: true, first-line-indent: 1em)",
    ]);
    let doc = paragraphs(rng, 3);
    let mut features = vec!["set"];
    features.push(if rule.contains("true") {
        "bool"
    } else {
        "length"
    });
    goal(doc, &[rule], features)
}

/// Numbering patterns for headings and lists.
pub fn set_numbering(rng: &mut Rng) -> Goal {
    match rng.below(3) {
        0 => {
            let pattern = *rng.pick(&["1.", "1.1", "I.", "A.", "1.a"]);
            let levels = pattern.contains("1.1") || pattern.contains("1.a");
            let doc = document(rng, 3, levels, false);
            goal(
                doc,
                &[&format!("#set heading(numbering: \"{pattern}\")")],
                vec!["set", "str"],
            )
        }
        1 => {
            let pattern = *rng.pick(&["a)", "i.", "(1)", "A."]);
            let items: Vec<String> = rng
                .distinct(SENTENCES.len(), 3)
                .into_iter()
                .map(|i| format!("+ {}", SENTENCES[i]))
                .collect();
            goal(
                items.join("\n"),
                &[&format!("#set enum(numbering: \"{pattern}\")")],
                vec!["set", "str"],
            )
        }
        _ => {
            let marker = *rng.pick(&["[--]", "[>]", "[+]", "[–]"]);
            let items: Vec<String> = rng
                .distinct(SENTENCES.len(), 3)
                .into_iter()
                .map(|i| format!("- {}", SENTENCES[i]))
                .collect();
            goal(
                items.join("\n"),
                &[&format!("#set list(marker: {marker})")],
                vec!["set", "content-block"],
            )
        }
    }
}

/// `#set page(…)`: numbers, margins, headers, background.
pub fn set_page(rng: &mut Rng) -> Goal {
    let rule = *rng.pick(&[
        "#set page(numbering: \"1\")",
        "#set page(numbering: \"1 / 1\")",
        "#set page(numbering: \"- 1 -\")",
        "#set page(margin: 2cm)",
        "#set page(margin: 1.5cm)",
        "#set page(header: [Draft])",
        "#set page(header: [Confidential])",
        "#set page(fill: luma(235))",
    ]);
    let doc = document(rng, 2, false, false);
    let mut features = vec!["set"];
    features.push(if rule.contains("cm") {
        "length"
    } else if rule.contains('[') {
        "content-block"
    } else {
        "str"
    });
    goal(doc, &[rule], features)
}

/// `#show selector: set …`: style one kind of element everywhere.
pub fn show_set(rng: &mut Rng) -> Goal {
    let rule = match rng.below(4) {
        0 => format!(
            "#show heading: set text({})",
            rng.pick(&["blue", "red", "eastern", "purple"])
        ),
        1 => "#show heading: set align(center)".to_string(),
        2 => format!(
            "#show strong: set text({})",
            rng.pick(&["red", "blue", "eastern"])
        ),
        _ => format!(
            "#show emph: set text({})",
            rng.pick(&["green", "purple", "blue"])
        ),
    };
    let doc = document(rng, 3, false, true);
    goal(doc, &[&rule], vec!["show-set"])
}

const BRANDS: &[&str] = &["Typst", "Glacio", "Acme", "Nimbus"];

/// `#show selector: function`: transform matches with a function or
/// replace them outright.
pub fn show_fn(rng: &mut Rng) -> Goal {
    match rng.below(3) {
        0 => {
            let brand = *rng.pick(BRANDS);
            let f = *rng.pick(&["smallcaps", "strong", "emph", "underline"]);
            let doc = brand_document(rng, brand);
            goal(
                doc,
                &[&format!("#show \"{brand}\": {f}")],
                vec!["show-fn", "str"],
            )
        }
        1 => {
            let brand = *rng.pick(BRANDS);
            let full = format!("{brand} Labs Inc.");
            let doc = brand_document(rng, brand);
            goal(
                doc,
                &[&format!("#show \"{brand}\": [{full}]")],
                vec!["show-fn", "str", "content-block"],
            )
        }
        _ => {
            let f = *rng.pick(&["smallcaps", "underline", "emph"]);
            let doc = document(rng, 3, false, false);
            goal(doc, &[&format!("#show heading: {f}")], vec!["show-fn"])
        }
    }
}

/// A few sentences that mention `brand` several times.
fn brand_document(rng: &mut Rng, brand: &str) -> String {
    let lines = [
        format!("{brand} builds tools for writers."),
        format!("Everyone at {brand} reads every word twice."),
        format!("Ask {brand} support if anything breaks."),
        format!("The {brand} team ships on Fridays."),
    ];
    let picks = rng.distinct(lines.len(), 3);
    picks
        .into_iter()
        .map(|i| lines[i].clone())
        .collect::<Vec<_>>()
        .join(" ")
}

/// (rule, checklist item) pairs from different areas, for style guides.
const GUIDE: &[&[(&str, &str)]] = &[
    &[
        ("#set text(11pt)", "body text 11pt"),
        ("#set text(eastern)", "text in the color eastern"),
    ],
    &[
        ("#set par(justify: true)", "justified paragraphs"),
        (
            "#set par(first-line-indent: 1em)",
            "first lines indented 1em",
        ),
    ],
    &[
        (
            "#set heading(numbering: \"1.\")",
            "headings numbered 1., 2., …",
        ),
        (
            "#set heading(numbering: \"I.\")",
            "headings numbered I., II., …",
        ),
    ],
    &[
        ("#show heading: set align(center)", "headings centered"),
        ("#show heading: set text(blue)", "headings in blue"),
        ("#show heading: smallcaps", "headings in small capitals"),
    ],
    &[
        ("#set page(numbering: \"1\")", "page numbers at the bottom"),
        ("#set page(header: [Draft])", "\"Draft\" in the page header"),
    ],
];

/// A checklist of 2–3 requirements from different areas, like a
/// conference's style guide.
pub fn style_guide(rng: &mut Rng) -> Goal {
    let count = 2 + rng.below(2);
    let areas = rng.distinct(GUIDE.len(), count);
    let picked: Vec<(&str, &str)> = areas.into_iter().map(|a| *rng.pick(GUIDE[a])).collect();
    let rules: Vec<&str> = picked.iter().map(|(r, _)| *r).collect();
    let checklist = picked
        .iter()
        .map(|(_, item)| format!("• {item}"))
        .collect::<Vec<_>>()
        .join("\n");
    let doc = document(rng, 3, false, false);
    let mut features = vec![];
    if rules.iter().any(|r| r.starts_with("#set")) {
        features.push("set");
    }
    if rules.iter().any(|r| r.contains(": set ")) {
        features.push("show-set");
    }
    if rules
        .iter()
        .any(|r| r.starts_with("#show") && !r.contains(": set "))
    {
        features.push("show-fn");
    }
    let mut goal = Goal::new(doc.clone(), styled(&rules, &doc), features)
        .with_prompt(format!("Style guide:\n{checklist}"));
    // Each requirement must be met with a rule, not by hand.
    goal.require = rules
        .iter()
        .map(|r| {
            r.split('(')
                .next()
                .unwrap()
                .split(':')
                .next()
                .unwrap()
                .to_string()
        })
        .collect();
    goal.require.dedup();
    goal
}

/// A mix of everything in the Styling module.
pub fn review(rng: &mut Rng) -> Goal {
    const ALL: &[fn(&mut Rng) -> Goal] = &[
        set_text,
        set_par,
        set_numbering,
        set_page,
        show_set,
        show_fn,
        style_guide,
    ];
    (rng.pick(ALL))(rng)
}
