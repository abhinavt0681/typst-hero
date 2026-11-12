//! Goals for the Math module, including LaTeX → Typst translation.
//!
//! Most math goals start from an empty equation (`$ $`): you see the
//! rendered formula and write it. Whitespace never costs XP, so `(a+b)/2`
//! and `(a + b) / 2` are equally efficient.

use crate::exercise::Goal;
use crate::rng::Rng;

const VARS: &[&str] = &["x", "y", "z", "a", "b", "n", "t"];

/// Two different single-letter variables.
fn two_vars(rng: &mut Rng) -> (&'static str, &'static str) {
    let i = rng.distinct(VARS.len(), 2);
    (VARS[i[0]], VARS[i[1]])
}

fn empty() -> &'static str {
    "$ $"
}

/// Put a formula written as plain text into math mode, inline or as a block.
pub fn basics(rng: &mut Rng) -> Goal {
    const FORMULAS: &[(&str, &[&str])] = &[
        ("pi r^2", &["math-symbol", "math-sup"]),
        ("E = m c^2", &["math-sup", "math-mul"]),
        ("a^2 + b^2 = c^2", &["math-sup"]),
        ("2 pi r", &["math-symbol", "math-mul"]),
        ("x^2 - 1", &["math-sup"]),
        ("alpha + beta", &["math-symbol"]),
        ("n^3 + n", &["math-sup"]),
    ];
    const LEADS: &[&str] = &[
        "the area is",
        "we know that",
        "recall",
        "note that",
        "this gives",
    ];
    let (formula, feats) = *rng.pick(FORMULAS);
    let lead = *rng.pick(LEADS);
    let mut features = feats.to_vec();
    if rng.below(2) == 0 {
        features.push("math-inline");
        Goal::new(
            format!("{lead} {formula} here"),
            format!("{lead} ${formula}$ here"),
            features,
        )
    } else {
        features.push("math-block");
        Goal::new(
            format!("{lead}\n\n{formula}"),
            format!("{lead}\n\n$ {formula} $"),
            features,
        )
    }
}

/// Subscripts, superscripts and fractions, with parentheses for grouping.
pub fn scripts(rng: &mut Rng) -> Goal {
    let (v, w) = two_vars(rng);
    let k = 2 + rng.below(8);
    let (math, features): (String, &[&str]) = match rng.below(8) {
        0 => (format!("{v}^{k}"), &["math-sup"]),
        1 => (format!("{v}_{k}"), &["math-sub"]),
        2 => (format!("{v}^({w}+1)"), &["math-sup"]),
        3 => (format!("{v}_({w} {v})"), &["math-sub"]),
        4 => (format!("{v}/{w}"), &["math-frac"]),
        5 => (format!("({v}+{w})/{k}"), &["math-frac"]),
        6 => (format!("1/(1+{v}^2)"), &["math-frac", "math-sup"]),
        _ => (format!("{v}_1^{k} + {v}_2^{k}"), &["math-sub", "math-sup"]),
    };
    let mut features = features.to_vec();
    features.push("math-block");
    Goal::new(empty(), format!("$ {math} $"), features)
}

/// Symbols by name and shorthands for arrows and relations.
pub fn symbols(rng: &mut Rng) -> Goal {
    let (v, w) = two_vars(rng);
    let greek = *rng.pick(&[
        "alpha", "beta", "gamma", "theta", "lambda", "sigma", "omega", "phi",
    ]);
    let (math, features): (String, &[&str]) = match rng.below(8) {
        0 => (format!("{v} -> {w}"), &["math-shorthand"]),
        1 => (format!("{v} != {w}"), &["math-shorthand"]),
        2 => (format!("{v} <= {w}"), &["math-shorthand"]),
        3 => (format!("{greek} + {v}"), &["math-symbol"]),
        4 => (format!("{v} in RR"), &["math-symbol"]),
        5 => (
            format!("sum_(i=1)^n {v}_i"),
            &["math-symbol", "math-sub", "math-sup"],
        ),
        6 => (format!("{v} => {w}"), &["math-shorthand"]),
        _ => (
            format!("lim_({v} -> oo) 1/{v}"),
            &["math-symbol", "math-shorthand", "math-frac"],
        ),
    };
    let mut features = features.to_vec();
    features.push("math-block");
    Goal::new(empty(), format!("$ {math} $"), features)
}

/// Math functions: roots, vectors, binomials, absolute values, text.
pub fn functions(rng: &mut Rng) -> Goal {
    let (v, w) = two_vars(rng);
    let k = 2 + rng.below(4);
    let math = match rng.below(7) {
        0 => format!("sqrt({v})"),
        1 => format!("root({k}, {v})"),
        2 => format!("vec({v}, {w})"),
        3 => format!("binom(n, {k})"),
        4 => format!("abs({v} - {w})"),
        5 => format!("{v} = 1 \"if\" {w} > 0"),
        _ => format!("sqrt({v}^2 + {w}^2)"),
    };
    let mut features = vec!["math-call", "math-block"];
    if math.contains('"') {
        features = vec!["math-text", "math-block"];
    }
    Goal::new(empty(), format!("$ {math} $"), features)
}

/// Multi-line equations aligned at `&`.
pub fn alignment(rng: &mut Rng) -> Goal {
    let (v, w) = two_vars(rng);
    let k = 2 + rng.below(7);
    let math = match rng.below(4) {
        0 => format!("{v} &= {k}{w} + {k} \\\n  &= {k}({w} + 1)"),
        1 => format!("({v}+{w})^2 &= ({v}+{w})({v}+{w}) \\\n  &= {v}^2 + 2 {v} {w} + {w}^2"),
        2 => format!("{v} &= {k} \\\n{w} &= {}", k + 1),
        _ => format!("f({v}) &= {v}^2 \\\n  f'({v}) &= 2{v}"),
    };
    Goal::new(
        empty(),
        format!("$ {math} $"),
        vec!["math-block", "math-align", "math-linebreak"],
    )
}

/// (LaTeX, Typst, skills)
const LATEX: &[(&str, &str, &[&str])] = &[
    (r"\frac{a}{b}", "a/b", &["math-frac"]),
    (r"\sqrt{x}", "sqrt(x)", &["math-call"]),
    (r"x^{n+1}", "x^(n+1)", &["math-sup"]),
    (r"\alpha + \beta", "alpha + beta", &["math-symbol"]),
    (
        r"\sum_{i=1}^{n} i",
        "sum_(i=1)^n i",
        &["math-symbol", "math-sub", "math-sup"],
    ),
    (r"x \in \mathbb{R}", "x in RR", &["math-symbol"]),
    (r"a \leq b", "a <= b", &["math-shorthand"]),
    (r"a \cdot b", "a dot b", &["math-symbol"]),
    (r"\infty", "oo", &["math-symbol"]),
    (r"\left( \frac{1}{2} \right)", "(1/2)", &["math-frac"]),
    (r"\vec{v}", "arrow(v)", &["math-call"]),
    (r"\hat{x}", "hat(x)", &["math-call"]),
    (
        r"\int_0^1 x \, dx",
        "integral_0^1 x dif x",
        &["math-symbol", "math-sub", "math-sup"],
    ),
    (r"x \neq 0", "x != 0", &["math-shorthand"]),
    (r"\binom{n}{k}", "binom(n, k)", &["math-call"]),
    (r"a \to b", "a -> b", &["math-shorthand"]),
];

/// See LaTeX, write the Typst equivalent.
pub fn latex(rng: &mut Rng) -> Goal {
    let count = 1 + rng.below(2);
    let picks = rng.distinct(LATEX.len(), count);
    let (mut tex, mut typ, mut features) = (Vec::new(), Vec::new(), vec!["math-block"]);
    for i in picks {
        let (t, y, f) = LATEX[i];
        tex.push(t);
        typ.push(y);
        features.extend_from_slice(f);
    }
    features.sort_unstable();
    features.dedup();
    let latex = tex.join(r" \quad ");
    Goal::new(empty(), format!("$ {} $", typ.join(" quad ")), features)
        .with_prompt(format!("LaTeX: {latex}"))
}

/// A mix of everything in the Math module.
pub fn review(rng: &mut Rng) -> Goal {
    const ALL: &[fn(&mut Rng) -> Goal] = &[basics, scripts, symbols, functions, alignment, latex];
    (rng.pick(ALL))(rng)
}
