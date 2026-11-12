//! typst-hero-core: everything that doesn't depend on the UI.
//!
//! Used by the server (to pre-render pages) and by the browser (compiled to
//! WebAssembly, to compile the learner's code on every keystroke).

pub mod exercise;
pub mod fonts;
pub mod lessons;
pub mod render;
pub mod rng;
pub mod world;

pub use exercise::{Goal, Lesson, Verdict, judge};
pub use render::{Diagnostic, Rendered, compile, compile_str};
pub use world::SandboxWorld;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lessons::LESSONS;
    use crate::rng::Rng;

    #[test]
    fn compiles_markup_to_svg() {
        let out = compile_str("Hello *world*").expect("compiles");
        assert_eq!(out.pages.len(), 1);
        assert!(out.pages[0].starts_with("<svg"));
    }

    #[test]
    fn sugar_and_function_call_look_identical() {
        let a = compile_str("Hello *world*").unwrap();
        let b = compile_str("Hello #strong[world]").unwrap();
        assert!(a.looks_like(&b));
        let c = compile_str("Hello _world_").unwrap();
        assert!(!a.looks_like(&c));
    }

    #[test]
    fn errors_point_at_the_learners_line() {
        let errs = compile_str("fine\n#let x = (").unwrap_err();
        assert!(errs[0].is_error);
        assert_eq!(errs[0].position.map(|p| p.0), Some(2), "{errs:?}");
    }

    #[test]
    fn judge_rewards_concise_solutions() {
        let target = compile_str("a *b* c").unwrap();
        let sugar = compile_str("a *b* c").unwrap();
        let call = compile_str("a #strong[b] c").unwrap();
        assert_eq!(judge(&target, &sugar, "a *b* c", "a *b* c"), Verdict::Solved { efficient: true });
        assert_eq!(judge(&target, &call, "a #strong[b] c", "a *b* c"), Verdict::Solved { efficient: false });
        let plain = compile_str("a b c").unwrap();
        assert_eq!(judge(&target, &plain, "a b c", "a *b* c"), Verdict::NotYet);
    }

    /// Every generated goal must compile, and the start must not already
    /// look like the target (otherwise the goal is solved before you begin).
    #[test]
    fn generated_goals_are_valid() {
        for lesson in LESSONS {
            for seed in 0..40 {
                let goal = (lesson.generate)(&mut Rng::new(seed));
                let target = compile_str(&goal.ideal)
                    .unwrap_or_else(|e| panic!("{}: ideal fails: {e:?}\n{}", lesson.slug, goal.ideal));
                let start = compile_str(&goal.start).expect("start compiles");
                assert!(!start.looks_like(&target), "{}: start already solved", lesson.slug);
                assert!(!goal.features.is_empty());
            }
        }
    }

    #[test]
    fn base_fonts_exist_on_the_server() {
        for name in fonts::BASE_FONTS.iter().chain(fonts::MATH_FONTS) {
            assert!(fonts::embedded_font_file(name).is_some(), "missing font {name}");
        }
    }

    #[test]
    fn chip_examples_compile() {
        for lesson in LESSONS {
            for chip in lesson.chips {
                compile_str(chip.example).unwrap_or_else(|e| panic!("{}: {e:?}", chip.syntax));
            }
        }
    }
}
