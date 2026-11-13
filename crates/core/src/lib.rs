//! typst-hero-core: everything that doesn't depend on the UI.
//!
//! Used by the server (to pre-render pages) and by the browser (compiled to
//! WebAssembly, to compile the learner's code on every keystroke).

pub mod exercise;
pub mod fonts;
pub mod generators;
pub mod highlight;
pub mod lessons;
pub mod projects;
pub mod render;
pub mod rng;
pub mod skills;
pub mod world;

pub use exercise::{Goal, Lesson, Verdict, judge};
pub use render::{Diagnostic, Rendered, compile, compile_project, compile_str};
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

    /// The Comments lesson says `//` inside a URL is not a comment.
    #[test]
    fn urls_are_not_comments() {
        let url = compile_str("Visit https://typst.app today").unwrap();
        let link = compile_str("Visit #link(\"https://typst.app\") today").unwrap();
        let cut = compile_str("Visit https:").unwrap();
        assert!(url.looks_like(&link));
        assert!(!url.looks_like(&cut));
    }

    #[test]
    fn judge_rewards_concise_solutions() {
        let goal = Goal::new("a b c", "a *b* c", vec!["strong"]);
        let target = compile_str(&goal.ideal).unwrap();
        let sugar = compile_str("a *b* c").unwrap();
        let call = compile_str("a #strong[b] c").unwrap();
        assert_eq!(
            judge(&goal, &target, &sugar, "a *b* c"),
            Verdict::Solved { efficient: true }
        );
        assert_eq!(
            judge(&goal, &target, &call, "a #strong[b] c"),
            Verdict::Solved { efficient: false }
        );
        let plain = compile_str("a b c").unwrap();
        assert_eq!(judge(&goal, &target, &plain, "a b c"), Verdict::NotYet);
    }

    /// For three or more lines in a row, `//` on every line solves the goal
    /// but `/* … */` is the efficient answer (as the Comments lesson says).
    #[test]
    fn block_comment_beats_line_comments() {
        let goal = (0..200)
            .map(|seed| generators::by_name("comments").unwrap()(&mut Rng::new(seed)))
            .find(|g| g.ideal.contains("/*") && g.ideal.contains("\n\n") && g.require.len() >= 3)
            .expect("a multi-line block comment goal");
        let target = compile_str(&goal.ideal).unwrap();
        let line_comments: String = goal
            .start
            .split("\n\n")
            .map(|l| if goal.require.iter().any(|r| r == l) { format!("// {l}") } else { l.to_string() })
            .collect::<Vec<_>>()
            .join("\n\n");
        let out = compile_str(&line_comments).unwrap();
        assert_eq!(judge(&goal, &target, &out, &line_comments), Verdict::Solved { efficient: false });
        assert_eq!(judge(&goal, &target, &target, &goal.ideal), Verdict::Solved { efficient: true });
    }

    /// Typing the values by hand looks right but fails the hidden variants.
    #[test]
    fn hardcoded_answers_are_caught() {
        let goal = Goal::new("#let n = 3\n\n", "#let n = 3\n\n#for i in range(1, n + 1) [#i ]", vec!["for"])
            .with_variants([("n = 3", "n = 5")]);
        let target = compile_str(&goal.ideal).unwrap();
        let by_hand = "#let n = 3\n\n1 2 3";
        let out = compile_str(by_hand).unwrap();
        assert!(out.looks_like(&target));
        assert_eq!(judge(&goal, &target, &out, by_hand), Verdict::Hardcoded);
        assert_eq!(judge(&goal, &target, &target, &goal.ideal), Verdict::Solved { efficient: true });
    }

    #[test]
    fn judge_enforces_required_text() {
        // Commenting out vs deleting: same output, only one is the lesson.
        let goal =
            Goal::new("keep\n\ndrop", "keep\n\n// drop", vec!["comment"]).requiring(["drop"]);
        let target = compile_str(&goal.ideal).unwrap();
        let deleted = compile_str("keep").unwrap();
        assert_eq!(judge(&goal, &target, &deleted, "keep"), Verdict::NotYet);
        let commented = compile_str("keep\n\n// drop").unwrap();
        assert_eq!(
            judge(&goal, &target, &commented, "keep\n\n// drop"),
            Verdict::Solved { efficient: true }
        );
    }

    /// Every generated goal must compile, and the start must not already
    /// look like the target (otherwise the goal is solved before you begin).
    #[test]
    fn generated_goals_are_valid() {
        for lesson in LESSONS.iter() {
            for seed in 0..40 {
                let goal = (lesson.generate)(&mut Rng::new(seed));
                let target = compile_str(&goal.ideal).unwrap_or_else(|e| {
                    panic!("{}: ideal fails: {e:?}\n{}", lesson.slug, goal.ideal)
                });
                let start = compile_str(&goal.start).unwrap_or_else(|e| {
                    panic!("{}: start fails: {e:?}\n{}", lesson.slug, goal.start)
                });
                assert!(
                    !start.looks_like(&target),
                    "{}: start already solved:\n{}",
                    lesson.slug,
                    goal.start
                );
                assert!(
                    matches!(judge(&goal, &target, &target, &goal.ideal), Verdict::Solved { efficient: true }),
                    "{}: ideal fails its own checks:\n{}",
                    lesson.slug,
                    goal.ideal
                );
                for (from, to) in &goal.variants {
                    let swapped = compile_str(&goal.ideal.replacen(from.as_str(), to, 1))
                        .unwrap_or_else(|e| panic!("{}: variant fails: {e:?}", lesson.slug));
                    assert!(!swapped.looks_like(&target), "{}: variant `{to}` changes nothing", lesson.slug);
                }
                for r in &goal.require {
                    assert!(
                        goal.ideal.contains(r.as_str()),
                        "{}: ideal lacks required `{r}`",
                        lesson.slug
                    );
                }
                assert!(!goal.features.is_empty());
            }
        }
    }

    /// Goals must not repeat too often, or learners memorize answers.
    #[test]
    fn generators_have_variety() {
        for lesson in LESSONS.iter() {
            let mut rng = Rng::new(7);
            let mut seen = std::collections::HashSet::new();
            for _ in 0..50 {
                seen.insert((lesson.generate)(&mut rng).ideal);
            }
            assert!(
                seen.len() >= 25,
                "{}: only {} distinct goals in 50",
                lesson.slug,
                seen.len()
            );
        }
    }

    /// Loads every content file; fails with the file and problem if any
    /// lesson, chip or skill reference is wrong.
    #[test]
    fn course_loads() {
        assert!(!lessons::COURSE.modules.is_empty());
        assert_eq!(
            lessons::LESSONS.len(),
            lessons::COURSE
                .modules
                .iter()
                .map(|m| m.lessons.len())
                .sum::<usize>()
        );
    }

    /// Generators may only award XP to skills that exist.
    #[test]
    fn goal_features_are_known_skills() {
        for lesson in LESSONS.iter() {
            for seed in 0..20 {
                for f in (lesson.generate)(&mut Rng::new(seed)).features {
                    assert!(
                        skills::skill(f).is_some(),
                        "{}: unknown skill `{f}`",
                        lesson.slug
                    );
                }
            }
        }
    }

    #[test]
    fn base_fonts_exist_on_the_server() {
        for name in fonts::BASE_FONTS.iter().chain(fonts::MATH_FONTS) {
            assert!(
                fonts::embedded_font_file(name).is_some(),
                "missing font {name}"
            );
        }
    }

    #[test]
    fn chip_examples_compile() {
        for lesson in LESSONS.iter() {
            for chip in &lesson.chips {
                compile_str(&chip.example).unwrap_or_else(|e| panic!("{}: {e:?}", chip.syntax));
            }
        }
    }
}
