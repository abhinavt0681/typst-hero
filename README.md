# typst-hero

Learn Typst by doing: short lessons, a real editor, and drills that are
different every time. Inspired by [VimHero](https://www.vim-hero.com).
See [PLAN.md](PLAN.md) for the full design.

## Run it

You need Rust (stable) plus two one-time installs:

```sh
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
```

Then:

```sh
cargo leptos watch      # dev server with live reload → http://127.0.0.1:3000
cargo leptos serve --release   # optimized build
cargo test -p typst-hero-core  # core tests
```

## How the code is organized

| Path | What it is |
|---|---|
| `crates/core/src/world.rs` | The sandbox the Typst compiler runs in: in-memory files, embedded fonts |
| `crates/core/src/render.rs` | Compile learner code → SVG pages, or errors with line numbers |
| `crates/core/src/exercise.rs` | Lessons, goals, and judging (matches target? efficient?) |
| `crates/core/src/lessons.rs` | Lesson content and the random goal generators |
| `crates/web/src/main.rs` | The Axum server (server-side rendering) |
| `crates/web/src/app.rs` | Page shell, routes, header, sidebar, home page |
| `crates/web/src/lesson.rs` | Lesson page: intro, syntax chips with examples, pager |
| `crates/web/src/challenge.rs` | The live challenge: editor, target vs output, summary |
| `crates/web/src/editor.rs` | The code editor component (CodeMirror + Vim), driven from Rust |
| `crates/web/js/editor-src.js` | The only hand-written JavaScript: a thin CodeMirror wrapper |
| `crates/web/js/editor.js` | Its prebuilt bundle (committed; regenerate with `scripts/vendor-editor.sh`, which needs Node) |
| `crates/core/src/highlight.rs` | Syntax highlighting with Typst's own parser |
| `crates/web/style/main.css` | All styles |

Building and running never needs Node. Only `scripts/vendor-editor.sh`, used
when upgrading CodeMirror, does.

The `core` crate is used twice: by the server to pre-render pages, and in the
browser (compiled to WebAssembly) to compile your code on every keystroke.

## License

typst-hero is free software under the **GNU Affero General Public License
v3.0 or later** (see [LICENSE](LICENSE)). You may use, study, change and share
it. If you distribute it, or run a modified version as a public website, you
must share your source code under the same license.

Bundled third-party components keep their own licenses; see
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
