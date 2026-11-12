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
| `crates/web/style/main.css` | All styles |

The `core` crate is used twice: by the server to pre-render pages, and in the
browser (compiled to WebAssembly) to compile your code on every keystroke.

## License

MIT OR Apache-2.0
