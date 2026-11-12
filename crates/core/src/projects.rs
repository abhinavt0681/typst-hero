//! Projects: one real document built across guided steps (e.g. a resume),
//! loaded from `content/projects/*.toml`.
//!
//! Each step shows a target (the whole document after the step). Your code
//! carries over between steps; a step is done when your output matches.

use std::sync::LazyLock;

use serde::Deserialize;

use crate::lessons::embedded;

pub struct Project {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub intro: Vec<String>,
    pub steps: Vec<Step>,
    /// Shown after the last step, in free "make it yours" mode.
    pub finale: Vec<String>,
}

pub struct Step {
    pub title: String,
    pub text: Vec<String>,
    /// The complete document after this step.
    pub target: String,
    /// Plain text appended to the learner's code when the step starts.
    pub insert: Option<String>,
    /// Text the learner's code must contain.
    pub require: Vec<String>,
}

impl Step {
    /// Whether `code` completes this step, given its compiled output.
    pub fn accepts(&self, code: &str, output: &crate::Rendered, target: &crate::Rendered) -> bool {
        output.looks_like(target) && self.require.iter().all(|r| code.contains(r.as_str()))
    }
}

pub static PROJECTS: LazyLock<Vec<Project>> =
    LazyLock::new(|| load().unwrap_or_else(|e| panic!("invalid project content: {e}")));

pub fn project(slug: &str) -> Option<&'static Project> {
    PROJECTS.iter().find(|p| p.slug == slug)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectFile {
    title: String,
    summary: String,
    intro: String,
    step: Vec<StepFile>,
    finale: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StepFile {
    title: String,
    text: String,
    target: String,
    insert: Option<String>,
    #[serde(default)]
    require: Vec<String>,
}

fn paragraphs(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(|p| p.trim().replace('\n', " "))
        .filter(|p| !p.is_empty())
        .collect()
}

fn load() -> Result<Vec<Project>, String> {
    embedded::PROJECT_FILES
        .iter()
        .map(|(slug, text)| {
            let file: ProjectFile =
                toml::from_str(text).map_err(|e| format!("projects/{slug}.toml: {e}"))?;
            Ok(Project {
                slug: slug.to_string(),
                title: file.title,
                summary: file.summary,
                intro: paragraphs(&file.intro),
                steps: file
                    .step
                    .into_iter()
                    .map(|s| Step {
                        title: s.title,
                        text: paragraphs(&s.text),
                        target: s.target.trim_start().to_string(),
                        insert: s.insert,
                        require: s.require,
                    })
                    .collect(),
                finale: paragraphs(&file.finale),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile_project;

    /// The finished document exports to a real PDF (what "Download PDF" does).
    #[cfg(feature = "pdf")]
    #[test]
    fn finished_project_exports_pdf() {
        for p in PROJECTS.iter() {
            let last = &p.steps.last().unwrap().target;
            let pdf = crate::render::export_pdf(&crate::SandboxWorld::project(last)).unwrap();
            assert!(pdf.starts_with(b"%PDF-"), "{}: not a PDF", p.slug);
            assert!(pdf.len() > 5_000, "{}: suspiciously small PDF", p.slug);
        }
    }

    /// Every step must compile, change the document, and contain the text
    /// it requires; otherwise the step would be impossible or pointless.
    #[test]
    fn project_steps_are_valid() {
        assert!(!PROJECTS.is_empty());
        for p in PROJECTS.iter() {
            let mut previous = compile_project("").unwrap();
            for (i, step) in p.steps.iter().enumerate() {
                let out = compile_project(&step.target)
                    .unwrap_or_else(|e| panic!("{} step {}: target fails: {e:?}", p.slug, i + 1));
                assert!(!out.looks_like(&previous), "{} step {}: changes nothing", p.slug, i + 1);
                assert!(
                    step.accepts(&step.target, &out, &out),
                    "{} step {}: target lacks required text",
                    p.slug,
                    i + 1
                );
                previous = out;
            }
        }
    }
}
