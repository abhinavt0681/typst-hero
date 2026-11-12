//! The course, loaded from `content/` (embedded at build time by build.rs).
//!
//! Lesson prose lives in TOML files so it can be edited without touching
//! Rust. Each lesson names a goal generator from `generators.rs`.

use std::collections::BTreeSet;
use std::sync::LazyLock;

use serde::Deserialize;

use crate::exercise::{Chip, Lesson};
use crate::generators;
use crate::skills;

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/content.rs"));
}

pub struct Module {
    pub id: String,
    pub title: String,
    /// Indices into `LESSONS`.
    pub lessons: Vec<usize>,
}

pub struct Course {
    pub modules: Vec<Module>,
    /// All lessons in course order.
    pub lessons: Vec<Lesson>,
}

/// The whole course. Panics with a clear message if a content file is
/// invalid; the `course_loads` test catches that before anything ships.
pub static COURSE: LazyLock<Course> = LazyLock::new(|| load().unwrap_or_else(|e| panic!("invalid course content: {e}")));

/// All lessons in course order.
pub static LESSONS: LazyLock<&'static [Lesson]> = LazyLock::new(|| &COURSE.lessons);

/// Skills that at least one lesson teaches (chips or generated goals).
/// Others show as "coming soon" on the Stats page.
pub static TAUGHT: LazyLock<BTreeSet<&'static str>> = LazyLock::new(|| {
    let mut taught = BTreeSet::new();
    for lesson in LESSONS.iter() {
        taught.extend(lesson.chips.iter().filter_map(|c| c.skill.as_deref()));
        for seed in 0..32 {
            taught.extend((lesson.generate)(&mut crate::rng::Rng::new(seed)).features);
        }
    }
    taught
});

pub fn lesson(slug: &str) -> Option<(usize, &'static Lesson)> {
    LESSONS.iter().enumerate().find(|(_, l)| l.slug == slug)
}

pub fn module_of(lesson: &Lesson) -> Option<&'static Module> {
    COURSE.modules.iter().find(|m| m.id == lesson.module)
}

// ------------------------------------------------------------------ files

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CourseFile {
    module: Vec<ModuleFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModuleFile {
    id: String,
    title: String,
    lessons: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LessonFile {
    title: String,
    generator: String,
    goals: usize,
    intro: String,
    task: Vec<String>,
    #[serde(default)]
    chip: Vec<ChipFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChipFile {
    syntax: String,
    meaning: String,
    example: String,
    skill: Option<String>,
}

fn load() -> Result<Course, String> {
    let course: CourseFile = toml::from_str(embedded::COURSE).map_err(|e| format!("course.toml: {e}"))?;
    let mut modules = Vec::new();
    let mut lessons = Vec::new();

    for m in course.module {
        let mut indices = Vec::new();
        for slug in &m.lessons {
            let (_, text) = embedded::LESSON_FILES
                .iter()
                .find(|(s, _)| s == slug)
                .ok_or_else(|| format!("course.toml lists `{slug}` but content/lessons/{slug}.toml is missing"))?;
            let file: LessonFile = toml::from_str(text).map_err(|e| format!("lessons/{slug}.toml: {e}"))?;
            indices.push(lessons.len());
            lessons.push(build_lesson(slug, &m.id, file)?);
        }
        modules.push(Module { id: m.id, title: m.title, lessons: indices });
    }

    for (slug, _) in embedded::LESSON_FILES {
        if !lessons.iter().any(|l| l.slug == *slug) {
            return Err(format!("lessons/{slug}.toml is not listed in course.toml"));
        }
    }
    Ok(Course { modules, lessons })
}

fn build_lesson(slug: &str, module: &str, file: LessonFile) -> Result<Lesson, String> {
    let generate = generators::by_name(&file.generator)
        .ok_or_else(|| format!("lessons/{slug}.toml: unknown generator `{}`", file.generator))?;
    let chips = file
        .chip
        .into_iter()
        .map(|c| {
            if let Some(id) = &c.skill
                && skills::skill(id).is_none()
            {
                return Err(format!("lessons/{slug}.toml: unknown skill `{id}`"));
            }
            Ok(Chip { syntax: c.syntax, meaning: c.meaning, example: c.example, skill: c.skill })
        })
        .collect::<Result<_, _>>()?;
    Ok(Lesson {
        slug: slug.to_string(),
        title: file.title,
        module: module.to_string(),
        intro: file.intro.split("\n\n").map(|p| p.trim().replace('\n', " ")).filter(|p| !p.is_empty()).collect(),
        chips,
        task: file.task,
        goals: file.goals,
        generate,
    })
}
