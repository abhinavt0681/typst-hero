//! Goals for the Scripting module.
//!
//! Every goal starts with data at the top (`#let …`) and asks for output
//! computed from it. Hidden variants swap the data and check the output
//! still matches, so typing the answer by hand doesn't pass.

use crate::exercise::Goal;
use crate::rng::Rng;

const PEOPLE: &[(&str, &str, u32)] = &[
    ("Ada Lovelace", "mathematics", 1815),
    ("Grace Hopper", "computing", 1906),
    ("Alan Turing", "logic", 1912),
    ("Katherine Johnson", "orbital mechanics", 1918),
    ("Edsger Dijkstra", "algorithms", 1930),
    ("Hedy Lamarr", "radio", 1914),
    ("Margaret Hamilton", "software engineering", 1936),
    ("Tim Berners-Lee", "the web", 1955),
];

/// Two different people: the visible one and the hidden variant.
fn two_people(rng: &mut Rng) -> ((&'static str, &'static str, u32), (&'static str, &'static str, u32)) {
    let i = rng.distinct(PEOPLE.len(), 2);
    let (a, b) = if rng.below(2) == 0 { (i[0], i[1]) } else { (i[1], i[0]) };
    (PEOPLE[a], PEOPLE[b])
}

/// `#let` bindings: use the variables instead of repeating the values.
pub fn variables(rng: &mut Rng) -> Goal {
    let ((name, field, born), (name2, _, born2)) = two_people(rng);
    let data = format!("#let name = \"{name}\"\n#let born = {born}");
    // The starting text has placeholders: replace them with the variables.
    let template = match rng.below(6) {
        0 => "NAME was born in BORN. Today, NAME is famous.".to_string(),
        1 => "In BORN, NAME was born. We still read NAME today.".to_string(),
        2 => format!("NAME (BORN) worked in {field}. NAME changed it."),
        3 => format!("Born in BORN, NAME shaped {field}."),
        4 => "Our hero: NAME. Year of birth: BORN. Thank you, NAME!".to_string(),
        _ => format!("NAME, BORN. Field: {field}. Remember NAME."),
    };
    let start = template.replace("NAME", "[name]").replace("BORN", "[born]");
    let ideal = template.replace("NAME", "#name").replace("BORN", "#born");
    Goal::new(format!("{data}\n\n{start}"), format!("{data}\n\n{ideal}"), vec!["let", "var", "hash"]).with_variants([
        (format!("name = \"{name}\""), format!("name = \"{name2}\"")),
        (format!("born = {born}"), format!("born = {born2}")),
    ])
}

/// Dictionaries and field access: `person.name`.
pub fn dictionaries(rng: &mut Rng) -> Goal {
    let ((name, field, born), (name2, field2, born2)) = two_people(rng);
    let data = format!("#let person = (name: \"{name}\", field: \"{field}\", born: {born})");
    let template = *rng.pick(&[
        "NAME worked in FIELD.",
        "Born BORN: NAME",
        "NAME (BORN) is known for FIELD.",
        "FIELD owes a lot to NAME.",
        "Name: NAME. Born: BORN. Field: FIELD.",
        "In BORN, a future star of FIELD was born.",
    ]);
    let fill = |name: &str, field: &str, born: &str| template.replace("NAME", name).replace("FIELD", field).replace("BORN", born);
    let start = fill("[person.name]", "[person.field]", "[person.born]");
    let ideal = fill("#person.name", "#person.field", "#person.born");
    Goal::new(format!("{data}\n\n{start}"), format!("{data}\n\n{ideal}"), vec!["dict", "field", "hash"]).with_variants([(
        format!("(name: \"{name}\", field: \"{field}\", born: {born})"),
        format!("(name: \"{name2}\", field: \"{field2}\", born: {born2})"),
    )])
}

/// `if` / `else`: choose content based on the data.
pub fn conditionals(rng: &mut Rng) -> Goal {
    let threshold = *rng.pick(&[50, 60, 70]);
    let pass = rng.below(2) == 0;
    let score = if pass { threshold + 1 + rng.below(30) } else { threshold - 1 - rng.below(30) };
    let other = if pass { threshold - 5 } else { threshold + 5 };
    let (yes, no) = *rng.pick(&[("passed", "failed"), ("approved", "rejected"), ("on track", "behind")]);
    let data = format!("#let score = {score}");
    let shown = if pass { yes } else { no };
    let start = format!("{data}\n\nScore #score: ");
    let ideal = format!("{data}\n\nScore #score: #if score >= {threshold} [{yes}] else [{no}]");
    Goal::new(start, ideal, vec!["if", "binary", "content-block"])
        .with_variants([(format!("score = {score}"), format!("score = {other}"))])
        .with_prompt(format!("Write \"{yes}\" if the score is at least {threshold}, otherwise \"{no}\". (Now it says: {shown}.)"))
}

const ITEM_SETS: &[&[&str]] = &[
    &["milk", "eggs", "bread", "coffee"],
    &["Rust", "Typst", "Git"],
    &["draft", "review", "submit", "celebrate"],
    &["north", "east", "south", "west"],
    &["apples", "pears", "plums"],
    &["red", "green", "blue"],
    &["Paris", "Lagos", "Lima", "Oslo"],
    &["alpha", "beta", "gamma", "delta", "epsilon"],
];

/// Two different item lists: visible and hidden variant.
fn two_sets(rng: &mut Rng) -> (&'static [&'static str], &'static [&'static str]) {
    let i = rng.distinct(ITEM_SETS.len(), 2);
    (ITEM_SETS[i[0]], ITEM_SETS[i[1]])
}

fn array_literal(items: &[&str]) -> String {
    format!("({})", items.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", "))
}

/// `for` loops over numbers and arrays.
pub fn loops(rng: &mut Rng) -> Goal {
    if rng.below(2) == 0 {
        let n = 4 + rng.below(8);
        let other = if n > 7 { n - 3 } else { n + 3 };
        let (body, shape) = *rng.pick(&[("#i ", "1 2 3 …"), ("(#i) ", "(1) (2) (3) …"), ("#i. ", "1. 2. 3. …")]);
        let data = format!("#let n = {n}");
        Goal::new(format!("{data}\n\nCount: "), format!("{data}\n\nCount: #for i in range(1, n + 1) [{body}]"), vec![
            "for", "call", "binary",
        ])
        .with_variants([(format!("n = {n}"), format!("n = {other}"))])
        .with_prompt(format!("Count from 1 to n, like {shape} (n = {n})."))
    } else {
        let (items, other) = two_sets(rng);
        let (body, how) = *rng.pick(&[
            ("*#item* \\ ", "in bold, each on its own line"),
            // Not `_#item_`: `_` can be part of a name, so that reads `#item_`.
            ("#emph(item) \\ ", "in italics, each on its own line"),
            ("[#item] ", "in square brackets, side by side"),
        ]);
        let data = format!("#let items = {}", array_literal(items));
        Goal::new(format!("{data}\n\n"), format!("{data}\n\n#for item in items [{body}]"), vec![
            "for", "var", "content-block",
        ])
        .with_variants([(array_literal(items), array_literal(other))])
        .with_prompt(format!("Every item {how}."))
    }
}

/// Array methods: `len`, `join`, `sorted`, `rev`, `first`, `last`.
pub fn arrays(rng: &mut Rng) -> Goal {
    let (items, other) = two_sets(rng);
    let data = format!("#let items = {}", array_literal(items));
    let (prompt, code) = match rng.below(6) {
        0 => ("How many items, and which?", "#items.len() items: #items.join(\", \")"),
        1 => ("The items in alphabetical order.", "#items.sorted().join(\", \")"),
        2 => ("The first and the last item.", "First #items.first(), last #items.last()."),
        3 => ("The items in reverse order.", "#items.rev().join(\", \")"),
        4 => ("The second item (arrays count from 0).", "Second: #items.at(1)"),
        _ => ("The items, with \"and\" before the last one.", "#items.join(\", \", last: \" and \")"),
    };
    Goal::new(format!("{data}\n\n"), format!("{data}\n\n{code}"), vec!["method", "array", "str"])
        .with_variants([(array_literal(items), array_literal(other))])
        .with_prompt(prompt)
}

const TAGS: &[&str] = &["Rust", "Typst", "Git", "Linux", "Neovim", "SQL", "Docker"];

/// Functions: define once, call many times.
pub fn functions(rng: &mut Rng) -> Goal {
    let count = 3 + rng.below(2);
    let tags: Vec<&str> = rng.distinct(TAGS.len(), count).into_iter().map(|i| TAGS[i]).collect();
    let args = *rng.pick(&["fill: luma(225), inset: 3pt", "stroke: blue, inset: 3pt", "fill: yellow, inset: 3pt"]);
    let def = format!("#let tag(t) = box({args}, t)");
    let calls = tags.iter().map(|t| format!("#tag[{t}]")).collect::<Vec<_>>().join(" ");
    Goal::new(tags.join(" "), format!("{def}\n\n{calls}"), vec!["let-fn", "call", "content-block"])
        .with_prompt(format!("Put each word in box({args}). Define a function once instead of repeating the box."))
}

/// A mix of everything in the Scripting module.
pub fn review(rng: &mut Rng) -> Goal {
    const ALL: &[fn(&mut Rng) -> Goal] = &[variables, dictionaries, conditionals, loops, arrays, functions];
    (rng.pick(ALL))(rng)
}
