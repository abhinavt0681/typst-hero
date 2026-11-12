//! The skill tree: every piece of Typst syntax, from the three syntax tables
//! in the official reference (markup, math, code). Like VimHero's commands,
//! each skill levels up on its own from 0 to 10 as you use it efficiently.

/// Skills are grouped on the Stats page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Group {
    Markup,
    Math,
    Values,
    Expressions,
    Styling,
    ControlFlow,
    Modules,
}

impl Group {
    pub const ALL: [Group; 7] = [
        Group::Markup,
        Group::Math,
        Group::Values,
        Group::Expressions,
        Group::Styling,
        Group::ControlFlow,
        Group::Modules,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Group::Markup => "markup",
            Group::Math => "math",
            Group::Values => "values",
            Group::Expressions => "expressions",
            Group::Styling => "styling",
            Group::ControlFlow => "control flow",
            Group::Modules => "modules",
        }
    }
}

pub struct Skill {
    /// Stable id, used in lesson files and saved progress. Never rename.
    pub id: &'static str,
    /// What you type, as shown on the chip.
    pub syntax: &'static str,
    pub name: &'static str,
    pub group: Group,
    /// Path under https://typst.app/docs/ for the official documentation.
    pub docs: &'static str,
}

const fn s(id: &'static str, syntax: &'static str, name: &'static str, group: Group, docs: &'static str) -> Skill {
    Skill { id, syntax, name, group, docs }
}

use Group::*;

pub static SKILLS: &[Skill] = &[
    // Markup (reference §3.1.2)
    s("parbreak", "blank line", "paragraph break", Markup, "reference/model/parbreak/"),
    s("strong", "*strong*", "strong emphasis", Markup, "reference/model/strong/"),
    s("emph", "_emphasis_", "emphasis", Markup, "reference/model/emph/"),
    s("raw", "`code`", "raw text", Markup, "reference/text/raw/"),
    s("link", "https://…", "link", Markup, "reference/model/link/"),
    s("label", "<intro>", "label", Markup, "reference/foundations/label/"),
    s("ref", "@intro", "reference", Markup, "reference/model/ref/"),
    s("heading", "= Heading", "heading", Markup, "reference/model/heading/"),
    s("list", "- item", "bullet list", Markup, "reference/model/list/"),
    s("enum", "+ item", "numbered list", Markup, "reference/model/enum/"),
    s("terms", "/ Term: text", "term list", Markup, "reference/model/terms/"),
    s("linebreak", "\\", "line break", Markup, "reference/text/linebreak/"),
    s("smartquote", "'…' \"…\"", "smart quotes", Markup, "reference/text/smartquote/"),
    s("shorthand", "~ -- ---", "symbol shorthands", Markup, "reference/symbols/"),
    s("hash", "#expr", "code expression", Markup, "reference/scripting/#expressions"),
    s("escape", "\\#", "character escape", Markup, "reference/syntax/#escapes"),
    s("comment", "// /* */", "comment", Markup, "reference/syntax/#comments"),
    // Math (reference §3.1.3)
    s("math-inline", "$x^2$", "inline math", Math, "reference/math/"),
    s("math-block", "$ x^2 $", "block math", Math, "reference/math/equation/"),
    s("math-sub", "x_1", "subscript", Math, "reference/math/attach/"),
    s("math-sup", "x^2", "superscript", Math, "reference/math/attach/"),
    s("math-frac", "a/b", "fraction", Math, "reference/math/frac/"),
    s("math-linebreak", "\\", "line break in math", Math, "reference/math/"),
    s("math-align", "&=", "alignment point", Math, "reference/math/#alignment"),
    s("math-symbol", "pi alpha", "symbols by name", Math, "reference/symbols/sym/"),
    s("math-variant", "arrow.r.long", "symbol variants", Math, "reference/symbols/"),
    s("math-mul", "x y", "implied multiplication", Math, "reference/math/"),
    s("math-shorthand", "-> !=", "math shorthands", Math, "reference/symbols/#shorthands"),
    s("math-text", "\"text\"", "text in math", Math, "reference/math/"),
    s("math-call", "sqrt(x)", "math function call", Math, "reference/math/#functions"),
    s("math-code", "#x", "code in math", Math, "reference/math/"),
    s("math-escape", "x\\^2", "escape in math", Math, "reference/syntax/#escapes"),
    // Code: values (reference §3.1.4)
    s("none", "none", "none", Values, "reference/foundations/none/"),
    s("auto", "auto", "auto", Values, "reference/foundations/auto/"),
    s("bool", "true false", "boolean", Values, "reference/foundations/bool/"),
    s("int", "10 0xff", "integer", Values, "reference/foundations/int/"),
    s("float", "3.14 1e5", "float", Values, "reference/foundations/float/"),
    s("length", "2pt 1em", "length", Values, "reference/layout/length/"),
    s("angle", "90deg", "angle", Values, "reference/layout/angle/"),
    s("fraction", "2fr", "fraction", Values, "reference/layout/fraction/"),
    s("ratio", "50%", "ratio", Values, "reference/layout/ratio/"),
    s("str", "\"hello\"", "string", Values, "reference/foundations/str/"),
    s("content-block", "[*Hello*]", "content block", Values, "reference/foundations/content/"),
    s("array", "(1, 2, 3)", "array", Values, "reference/foundations/array/"),
    s("dict", "(a: 1, b: 2)", "dictionary", Values, "reference/foundations/dictionary/"),
    // Code: expressions
    s("var", "x", "variable access", Expressions, "reference/scripting/#bindings"),
    s("code-block", "{ let x = 1; x }", "code block", Expressions, "reference/scripting/#blocks"),
    s("paren", "(1 + 2)", "parenthesized expression", Expressions, "reference/scripting/#expressions"),
    s("unary", "-x not x", "unary operator", Expressions, "reference/scripting/#operators"),
    s("binary", "x + y", "binary operator", Expressions, "reference/scripting/#operators"),
    s("assign", "x = 1", "assignment", Expressions, "reference/scripting/#operators"),
    s("field", "x.y", "field access", Expressions, "reference/scripting/#fields"),
    s("method", "x.len()", "method call", Expressions, "reference/scripting/#methods"),
    s("call", "f(x, y: 1)", "function call", Expressions, "reference/foundations/function/"),
    s("spread", "f(..args)", "argument spreading", Expressions, "reference/foundations/arguments/"),
    s("closure", "(x) => x + 1", "unnamed function", Expressions, "reference/foundations/function/"),
    s("let", "let x = 1", "let binding", Expressions, "reference/scripting/#bindings"),
    s("let-fn", "let f(x) = …", "named function", Expressions, "reference/foundations/function/"),
    // Code: styling
    s("set", "set text(14pt)", "set rule", Styling, "reference/styling/#set-rules"),
    s("set-if", "set … if cond", "set-if rule", Styling, "reference/styling/#set-rules"),
    s("show-set", "show heading: set …", "show-set rule", Styling, "reference/styling/#show-rules"),
    s("show-fn", "show raw: it => …", "show rule with function", Styling, "reference/styling/#show-rules"),
    s("show-all", "show: template", "show-everything rule", Styling, "reference/styling/#show-rules"),
    s("context", "context text.lang", "context expression", Styling, "reference/context/"),
    // Code: control flow
    s("if", "if … { } else { }", "conditional", ControlFlow, "reference/scripting/#conditionals"),
    s("for", "for x in xs { }", "for loop", ControlFlow, "reference/scripting/#loops"),
    s("while", "while cond { }", "while loop", ControlFlow, "reference/scripting/#loops"),
    s("loop-control", "break continue", "loop control", ControlFlow, "reference/scripting/#loops"),
    s("return", "return x", "return", ControlFlow, "reference/foundations/function/"),
    // Code: modules
    s("include", "include \"a.typ\"", "include file", Modules, "reference/scripting/#modules"),
    s("import", "import \"a.typ\"", "import module", Modules, "reference/scripting/#modules"),
    s("import-items", "import \"a.typ\": a, b", "import items", Modules, "reference/scripting/#modules"),
];

pub fn skill(id: &str) -> Option<&'static Skill> {
    SKILLS.iter().find(|s| s.id == id)
}

pub fn docs_url(skill: &Skill) -> String {
    format!("https://typst.app/docs/{}", skill.docs)
}

// ------------------------------------------------------------------ levels

/// XP needed to *reach* each level 1..=10. Grows quadratically, so early
/// levels come quickly and fluency takes sustained practice.
pub const LEVEL_XP: [u32; 10] = [3, 9, 18, 30, 45, 63, 84, 108, 135, 165];

pub fn level(xp: u32) -> u8 {
    LEVEL_XP.iter().take_while(|&&need| xp >= need).count() as u8
}

/// Progress within the current level, 0.0..1.0 (1.0 at max level).
pub fn level_progress(xp: u32) -> f64 {
    let lv = level(xp) as usize;
    if lv >= LEVEL_XP.len() {
        return 1.0;
    }
    let floor = if lv == 0 { 0 } else { LEVEL_XP[lv - 1] };
    let ceil = LEVEL_XP[lv];
    (xp - floor) as f64 / (ceil - floor) as f64
}

/// VimHero's bands: started (any XP), learning (< 4), practiced (4–5),
/// fluent (6–10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Band {
    Unstarted,
    Learning,
    Practiced,
    Fluent,
}

pub fn band(xp: u32) -> Band {
    match (xp, level(xp)) {
        (0, _) => Band::Unstarted,
        (_, 0..=3) => Band::Learning,
        (_, 4..=5) => Band::Practiced,
        _ => Band::Fluent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_ids_are_unique() {
        let mut ids: Vec<_> = SKILLS.iter().map(|s| s.id).collect();
        ids.sort_unstable();
        let before = ids.len();
        ids.dedup();
        assert_eq!(before, ids.len(), "duplicate skill id");
    }

    #[test]
    fn levels_and_bands() {
        assert_eq!(level(0), 0);
        assert_eq!(level(2), 0);
        assert_eq!(level(3), 1);
        assert_eq!(level(30), 4);
        assert_eq!(level(10_000), 10);
        assert_eq!(band(0), Band::Unstarted);
        assert_eq!(band(1), Band::Learning);
        assert_eq!(band(30), Band::Practiced);
        assert_eq!(band(63), Band::Fluent);
        assert!((level_progress(6) - 0.5).abs() < 1e-9);
        assert_eq!(level_progress(165), 1.0);
    }
}
