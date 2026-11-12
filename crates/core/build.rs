//! Embeds the course content (`content/`) into the binary, so the server and
//! the browser bundle both carry it without reading files at runtime.
//! Adding a lesson file needs no Rust changes here.

use std::path::Path;
use std::{env, fs};

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let content = Path::new(&manifest).join("../../content").canonicalize().expect("content/ exists");
    println!("cargo:rerun-if-changed={}", content.display());

    let mut lessons: Vec<_> = fs::read_dir(content.join("lessons"))
        .expect("content/lessons exists")
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .collect();
    lessons.sort();

    let mut out = String::new();
    out += &format!("pub const COURSE: &str = include_str!({:?});\n", content.join("course.toml"));
    out += "pub const LESSON_FILES: &[(&str, &str)] = &[\n";
    for path in &lessons {
        let slug = path.file_stem().unwrap().to_str().unwrap();
        out += &format!("    ({slug:?}, include_str!({path:?})),\n");
    }
    out += "];\n";

    fs::write(Path::new(&env::var("OUT_DIR").unwrap()).join("content.rs"), out).unwrap();
}
