//! Compiling learner code into something we can show and compare.

use typst::WorldExt;
use typst::diag::Severity;
use typst_layout::PagedDocument;

use crate::world::{SandboxWorld, user_file_id};

/// A successfully compiled document.
#[derive(Clone, Debug, PartialEq)]
pub struct Rendered {
    /// One SVG string per page.
    pub pages: Vec<String>,
}

impl Rendered {
    /// Two documents look identical if every page renders to the same SVG.
    /// The SVG contains positions, glyphs, sizes and colors but no source
    /// information, so `*hi*` and `#strong[hi]` compare equal.
    pub fn looks_like(&self, other: &Rendered) -> bool {
        self.pages == other.pages
    }
}

/// An error or warning, located in the learner's code when possible.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub is_error: bool,
    pub message: String,
    pub hints: Vec<String>,
    /// 1-based (line, column) in the learner's code.
    pub position: Option<(usize, usize)>,
}

pub fn compile(world: &SandboxWorld) -> Result<Rendered, Vec<Diagnostic>> {
    let doc = compile_document(world)?;
    let opts = typst_svg::SvgOptions::default();
    let pages = doc
        .pages()
        .iter()
        .map(|p| typst_svg::svg(p, &opts))
        .collect();
    Ok(Rendered { pages })
}

fn compile_document(world: &SandboxWorld) -> Result<PagedDocument, Vec<Diagnostic>> {
    typst::compile::<PagedDocument>(world)
        .output
        .map_err(|errors| diagnostics(world, &errors))
}

/// Compile to a PDF file (bytes), e.g. for downloading a finished project.
#[cfg(feature = "pdf")]
pub fn export_pdf(world: &SandboxWorld) -> Result<Vec<u8>, Vec<Diagnostic>> {
    let doc = compile_document(world)?;
    typst_pdf::pdf(&doc, &typst_pdf::PdfOptions::default())
        .map_err(|errors| diagnostics(world, &errors))
}

fn diagnostics(world: &SandboxWorld, errors: &[typst::diag::SourceDiagnostic]) -> Vec<Diagnostic> {
    errors
        .iter()
        .map(|d| {
            let position = (d.span.id() == Some(user_file_id()))
                .then(|| world.range(d.span))
                .flatten()
                .and_then(|range| world.user_source().lines().byte_to_line_column(range.start))
                .map(|(line, col)| (line + 1, col + 1));
            Diagnostic {
                is_error: d.severity == Severity::Error,
                message: d.message.to_string(),
                hints: d.hints.iter().map(|h| h.v.to_string()).collect(),
                position,
            }
        })
        .collect()
}

/// Compile project code (real A4 pages).
pub fn compile_project(code: &str) -> Result<Rendered, Vec<Diagnostic>> {
    compile(&SandboxWorld::project(code))
}

/// Convenience: compile a snippet of learner code with default settings.
pub fn compile_str(code: &str) -> Result<Rendered, Vec<Diagnostic>> {
    compile(&SandboxWorld::new(code))
}
