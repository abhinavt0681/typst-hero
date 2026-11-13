//! Goals for the Functions module: calling functions, arguments, images,
//! figures, tables, alignment and spacing.

use crate::exercise::Goal;
use crate::rng::Rng;

const SENTENCES: &[&str] = &[
    "the deadline moved to friday",
    "please read the attached draft",
    "our results look promising",
    "this section needs more sources",
    "the budget was approved today",
    "never skip the final check",
    "the team shipped version two",
];

/// Picks a word (not the first) to style and returns (before, word, after).
fn split_word<'a>(rng: &mut Rng, sentence: &'a str) -> (String, &'a str, String) {
    let words: Vec<&str> = sentence.split(' ').collect();
    let i = 1 + rng.below(words.len() - 1);
    (words[..i].join(" "), words[i], words[i + 1..].join(" "))
}

fn join3(a: &str, b: &str, c: &str) -> String {
    [a, b, c]
        .iter()
        .filter(|s| !s.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Functions that take content: `#underline[word]`.
pub fn text_functions(rng: &mut Rng) -> Goal {
    let sentence = *rng.pick(SENTENCES);
    let (before, word, after) = split_word(rng, sentence);
    let f = *rng.pick(&["underline", "highlight", "strike", "smallcaps", "overline"]);
    Goal::new(
        join3(&before, word, &after),
        join3(&before, &format!("#{f}[{word}]"), &after),
        vec!["hash", "call", "content-block"],
    )
}

/// Arguments: `#text(blue)[word]`, `#text(16pt)[word]`, or both.
pub fn arguments(rng: &mut Rng) -> Goal {
    let sentence = *rng.pick(SENTENCES);
    let (before, word, after) = split_word(rng, sentence);
    let color = *rng.pick(&["red", "blue", "green", "orange", "purple", "eastern"]);
    let size = format!("{}pt", rng.pick(&[8, 9, 15, 16, 18, 20]));
    let mut features = vec!["hash", "call", "content-block"];
    let args = match rng.below(3) {
        0 => color.to_string(),
        1 => {
            features.push("length");
            size
        }
        _ => {
            features.push("length");
            format!("{color}, {size}")
        }
    };
    Goal::new(
        join3(&before, word, &after),
        join3(&before, &format!("#text({args})[{word}]"), &after),
        features,
    )
}

const IMAGES: &[&str] = &["mountain.svg", "chart.svg", "logo.svg", "wave.svg"];
const WIDTHS: &[&str] = &[
    "30%", "40%", "50%", "60%", "70%", "80%", "3cm", "4cm", "5cm", "6cm", "7cm",
];

/// Resize an image with the named `width` argument.
pub fn images(rng: &mut Rng) -> Goal {
    let img = *rng.pick(IMAGES);
    let width = *rng.pick(WIDTHS);
    let unit = if width.ends_with('%') {
        "ratio"
    } else {
        "length"
    };
    Goal::new(
        format!("#image(\"{img}\")"),
        format!("#image(\"{img}\", width: {width})"),
        vec!["hash", "call", "str", unit],
    )
}

const CAPTIONS: &[&str] = &[
    "Sales by month.",
    "The study area.",
    "Our project logo.",
    "Wave height over time.",
    "Results of the survey.",
    "A view from the summit.",
];

/// Wrap an image in a figure with a caption.
pub fn figures(rng: &mut Rng) -> Goal {
    let img = *rng.pick(IMAGES);
    let width = *rng.pick(&["40%", "50%", "60%", "4cm", "5cm"]);
    let caption = *rng.pick(CAPTIONS);
    let image = format!("image(\"{img}\", width: {width})");
    Goal::new(
        format!("#{image}"),
        format!("#figure({image}, caption: [{caption}])"),
        vec!["hash", "call", "content-block"],
    )
}

const TABLES: &[&[&[&str]]] = &[
    &[
        &["Name", "Role"],
        &["Ada", "Engineer"],
        &["Grace", "Admiral"],
        &["Alan", "Researcher"],
    ],
    &[
        &["Fruit", "Price", "Stock"],
        &["Apple", "1.20", "40"],
        &["Pear", "0.90", "25"],
        &["Plum", "2.10", "8"],
    ],
    &[
        &["Year", "Revenue"],
        &["2023", "1.4M"],
        &["2024", "2.1M"],
        &["2025", "3.0M"],
    ],
    &[
        &["Day", "Task"],
        &["Mon", "Draft"],
        &["Tue", "Review"],
        &["Wed", "Submit"],
        &["Thu", "Celebrate"],
    ],
];

/// Turn rows of words into a table.
pub fn tables(rng: &mut Rng) -> Goal {
    let data = *rng.pick(TABLES);
    let rows = 2 + rng.below(data.len() - 1);
    // Keep the header, then a random subset of the data rows (in order).
    let mut picked = vec![data[0]];
    picked.extend(
        rng.distinct(data.len() - 1, rows - 1)
            .into_iter()
            .map(|i| data[i + 1]),
    );
    let cols = data[0].len();
    let start = picked
        .iter()
        .map(|r| r.join(" "))
        .collect::<Vec<_>>()
        .join(" \\\n");
    let cells = picked
        .iter()
        .flat_map(|r| r.iter().map(|c| format!("[{c}]")))
        .collect::<Vec<_>>()
        .join(", ");
    Goal::new(
        start,
        format!("#table(columns: {cols}, {cells})"),
        vec!["hash", "call", "content-block", "int"],
    )
}

const PLACES: &[&str] = &[
    "Acme Corp",
    "Globex",
    "Initech",
    "North University",
    "Hooli",
    "City Library",
];
const PERIODS: &[&str] = &[
    "2021 to 2024",
    "since 2024",
    "2019 to 2021",
    "summer 2020",
    "2015 to 2019",
];

/// Center or right-align a line, or push part of a line to the right edge
/// with `#h(1fr)` (the classic resume layout).
pub fn spacing(rng: &mut Rng) -> Goal {
    match rng.below(3) {
        0 => {
            let (place, period) = (*rng.pick(PLACES), *rng.pick(PERIODS));
            Goal::new(
                format!("{place} {period}"),
                format!("{place} #h(1fr) {period}"),
                vec!["hash", "call", "fraction"],
            )
        }
        n => {
            let text = *rng.pick(SENTENCES);
            let side = if n == 1 { "center" } else { "right" };
            Goal::new(
                text,
                format!("#align({side})[{text}]"),
                vec!["hash", "call", "content-block"],
            )
        }
    }
}

const SECTIONS: &[&str] = &["Methods", "Results", "Background", "Data", "Analysis"];

/// Labels and references: `<label>` marks something, `@label` refers to it.
/// The output of `@methods` ("Section 1") could also be typed by hand, so
/// the goal requires a label and a reference in the code.
pub fn references(rng: &mut Rng) -> Goal {
    if rng.below(2) == 0 {
        let picks = rng.distinct(SECTIONS.len(), 2);
        let (a, b) = (SECTIONS[picks[0]], SECTIONS[picks[1]]);
        let label = a.to_lowercase();
        let pre = "#set heading(numbering: \"1.\")";
        let start = format!("{pre}\n\n= {a}\nSome text.\n\n= {b}\nAs described in the {label} section, it works.");
        let ideal = format!("{pre}\n\n= {a} <{label}>\nSome text.\n\n= {b}\nAs described in @{label}, it works.");
        Goal::new(start, ideal, vec!["label", "ref"]).requiring(["<", "@"])
    } else {
        let img = *rng.pick(IMAGES);
        let caption = *rng.pick(CAPTIONS);
        let fig = format!("#figure(image(\"{img}\", width: 40%), caption: [{caption}])");
        let lead = *rng.pick(&["The results are clear", "Look closely", "The trend is obvious", "Note the shape"]);
        let start = format!("{fig}\n\n{lead}, see the figure above.");
        let ideal = format!("{fig} <fig>\n\n{lead}, see @fig.");
        Goal::new(start, ideal, vec!["label", "ref"]).requiring(["<", "@"])
    }
}

/// A mix of everything in the Functions module.
pub fn review(rng: &mut Rng) -> Goal {
    const ALL: &[fn(&mut Rng) -> Goal] =
        &[text_functions, arguments, images, figures, tables, spacing];
    (rng.pick(ALL))(rng)
}
