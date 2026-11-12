//! A sandboxed Typst "world": everything the compiler can see.
//!
//! The Typst compiler never touches the disk or network by itself. It asks a
//! `World` for files, fonts and the date. Ours is fully in-memory, so the same
//! code runs natively (server, tests) and in the browser (WebAssembly).

use std::collections::HashMap;
use std::sync::LazyLock;

use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

/// Name of the file holding the learner's code.
pub const USER_FILE: &str = "/user.typ";
/// Name of the entry file: a prelude followed by `#include "user.typ"`.
const MAIN_FILE: &str = "/main.typ";

/// Styles applied to every exercise. Keeping them in `main.typ` (not in the
/// learner's file) means error line numbers match what the learner sees.
/// The learner's own `#set page(...)` still wins, because it comes later.
pub const PRELUDE: &str = "\
#set page(width: 14cm, height: auto, margin: 12pt)
#set text(size: 13pt)
";

/// Fonts and the standard library are expensive to build, so we build them
/// once per process (or per browser tab) and share them.
struct Assets {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
}

static ASSETS: LazyLock<Assets> = LazyLock::new(|| {
    let fonts: Vec<Font> = typst_assets::fonts()
        .flat_map(|data| Font::iter(Bytes::new(data)))
        .collect();
    Assets {
        library: LazyHash::new(Library::default()),
        book: LazyHash::new(FontBook::from_fonts(&fonts)),
        fonts,
    }
});

fn file_id(path: &str) -> FileId {
    let vpath = VirtualPath::new(path).expect("static path is valid");
    FileId::new(RootedPath::new(VirtualRoot::Project, vpath))
}

/// The id of the learner's file, used to map errors back to their code.
pub fn user_file_id() -> FileId {
    file_id(USER_FILE)
}

pub struct SandboxWorld {
    main: Source,
    user: Source,
    /// Extra files (images, data) available to `#image`, `#yaml`, ...
    files: HashMap<FileId, Bytes>,
    today: Option<(i32, u8, u8)>,
}

impl SandboxWorld {
    pub fn new(user_code: &str) -> Self {
        let main = format!("{PRELUDE}#include \"{}\"\n", &USER_FILE[1..]);
        Self {
            main: Source::new(file_id(MAIN_FILE), main),
            user: Source::new(user_file_id(), user_code.to_string()),
            files: HashMap::new(),
            today: None,
        }
    }

    /// Make a file available at `path` (e.g. "/photo.png").
    pub fn with_file(mut self, path: &str, data: impl Into<Vec<u8>>) -> Self {
        self.files.insert(file_id(path), Bytes::new(data.into()));
        self
    }

    /// Fix the date returned by `datetime.today()` (year, month, day).
    pub fn with_today(mut self, ymd: (i32, u8, u8)) -> Self {
        self.today = Some(ymd);
        self
    }

    pub fn user_source(&self) -> &Source {
        &self.user
    }
}

impl World for SandboxWorld {
    fn library(&self) -> &LazyHash<Library> {
        &ASSETS.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &ASSETS.book
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            Ok(self.main.clone())
        } else if id == self.user.id() {
            Ok(self.user.clone())
        } else {
            Err(FileError::NotFound(id.vpath().get_without_slash().into()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(bytes) = self.files.get(&id) {
            return Ok(bytes.clone());
        }
        // Allow `#read("user.typ")` and friends for completeness.
        self.source(id).map(|s| Bytes::from_string(s.text().to_string()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        ASSETS.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        let (y, m, d) = self.today?;
        Datetime::from_ymd(y, m, d)
    }
}
