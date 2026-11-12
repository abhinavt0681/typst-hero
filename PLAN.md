# typst-hero — Plan

A free, open-source, in-browser course that makes you fluent in Typst the way
VimHero makes you fluent in Vim: short lessons, a real editor, drills that are
generated fresh every time, and per-skill levels that only rise when you solve
things the idiomatic way. Unlike VimHero, you also leave with real documents
(resume, letter, thesis, invoice), not just muscle memory.

## 1. What we copy from VimHero (and why it works)

| VimHero mechanic | Why it works | typst-hero equivalent |
|---|---|---|
| Tiny lessons: a key-chip row per command + auto-playing example | One idea at a time, shown not told | Syntax chip (`*strong*`) + example that types itself while the preview renders |
| "Now it's your turn" challenge, random targets, instant next goal | Repetition without memorizing answers | Target rendering + your source + live preview; new randomized goal on match |
| Per-challenge stats: best time, recent average, history chart | Visible improvement | Same |
| Practice: category filters, goals / time / endless, 5/15/30/50 | Short sessions, user picks focus | Same; categories = tracks |
| XP only when as efficient as the ideal solution | Pushes toward idioms, not just correctness | XP only when the solution is as concise/idiomatic as the ideal |
| "Review inefficient goals": ideal vs yours, replayed | Teaches the better way at the exact moment it matters | Ideal vs yours diff, ideal replayed in the editor |
| Per-command levels 0–10 (learning / practiced / fluent), heatmap, streak | Long-term motivation | Per-*feature* levels, heatmap, streak, project milestones |

## 2. Audience

People who **write documents**, not programmers learning a language:
LaTeX switchers, students (theses, problem sets), professionals (resumes,
reports, invoices, legal documents). The promise: *productive in Typst in a
weekend, power user in a month.*

## 3. Learning loops (the "multi-faceted" approach)

| Facet | Purpose | Format |
|---|---|---|
| **Lessons** | Introduce one concept | Text + chips + auto-played examples + challenge |
| **Drills** | Fluency | Generated goals, timed, efficiency-scored, levelled |
| **Projects** | Real documents you keep | Build-along to a target design, then "make it yours", export PDF |
| **Templates** | Use Typst Universe | Import, configure, override, `typst init`, fork |
| **Local workflow** | Leave the browser | `typst watch`, Tinymist LSP in Neovim/VS Code, `typst init` |
| **Reference** | Look things up | Searchable cheat sheet + LaTeX → Typst table |

## 4. Curriculum

Typst has three modes — **markup** (default), **math** (`$…$`), **code** (`#…`).
The key insight, taught mid-course: *markup is sugar for function calls*
(`*x*` ≡ `#strong[x]`), which is what makes set/show rules click.

### Agreed after reviewing the official tutorial and the 0.15.1 docs (2026-11-13)

- **Skill tree = Typst's syntax tables** (reference §3.1: markup, math, code):
  ~72 skills, each levelled 0–10 like VimHero's 76 commands. Lessons and
  drills are tagged with skills; efficient solutions earn XP for them.
- **One growing document per track** (the tutorial's glacier-report idea:
  write → format → style to a guide → template), *plus* the standalone
  projects (resume, letter, invoice, legal, thesis).
- **Style-guide compliance exercises**: a checklist ("level-1 headings 13pt,
  centered, small caps; two columns; page numbers") verified item by item
  against the compiled layout.
- **LaTeX → Typst drills** for the largest audience (`\frac{a}{b}` → `a/b`),
  based on the docs' LaTeX guide.
- **Fill the tutorial's gaps**: a full tables module (Table guide), outline &
  footnotes, loops & data, packages, local workflow. Never use before teaching.
- Teach 0.15 idioms (`title`, `divider`, `path`); short accessibility lessons
  (alt text, title vs heading); every chip links to its official docs page.
- Lesson prose is our own (short, VimHero-style), not copied from the docs.
- The docs PDF stays local and untracked (`*.pdf` in `.gitignore`).

### Track 1 — Writing
1. Paragraphs & emphasis — blank lines, `*strong*`, `_emph_`, `` `raw` ``
2. Escapes & comments — `\*`, `\#`, `//`, `/* */`
3. Headings — `=`, `==`, `===`
4. Lists — `-`, `+`, `/ term:`, nesting by indentation
5. Links, labels, references — URLs, `#link`, `<label>`, `@ref`
6. Typography — `~`, `--`, `---`, smart quotes, line break `\`
- **Review**, then **Project: Resume v1** (hand-formatted)

### Track 2 — Images & functions
7. `#` enters code — `#image("x.png", width: 60%)`, positional vs named args
8. Content vs strings — `[…]` vs `"…"`, trailing content args
9. Markup is sugar — `#strong[]`, `#emph[]`, `#heading[]`
10. Figures — `#figure(image(…), caption: […]) <fig>`, `@fig`
11. Tables — `#table(columns: 3, …)`, headers, alignment
12. Spacing & boxes — `#h(1fr)`, `#v`, `#box`, `#block`, `#align`
- **Review**

### Track 3 — Math
13. Inline vs display — `$x$` vs `$ x $`
14. Scripts & fractions — `^`, `_`, `a/b`, grouping with `()`
15. Symbols by name — `alpha`, `infinity`, `arrow.r.double`, modifiers
16. Math functions — `sqrt`, `vec`, `mat`, `cases`, `lr`, `"text"`
17. Multi-line & alignment — `&`, `\`, equation numbering, `@eq` refs
- **Review**, **Project: Problem set**

### Track 4 — Styling
18. Set rules I — `#set text(…)`, `#set page(…)`, `#set par(justify: true)`
19. Set rules II — `#set heading(numbering: "1.1")`, `#set enum/list`
20. Show-set rules — `#show heading: set text(navy)`
21. Show rules with functions — `#show heading: it => …`, text & regex selectors
22. Scoping — rules inside `[…]` blocks
23. Layout — `grid`, `columns`, `place`, `pad`
- **Review**, **Project: Resume v2** (grid, styled headings), **Project: Cover letter**

### Track 5 — Scripting your documents
24. `#let` — values, content, and the `#` / code-block boundary
25. Data types — strings, arrays, dictionaries; `.at(i, default:)`, `.len()`, `.last()`
26. Functions — `#let f(x, y: 1) = …`, closures `(x) => …`
27. Control flow — `if`/`else`, `for`, `while`; joining content
28. Argument sinks & spreading — `(..args) => args.pos()`, `..arr`
29. Functions as rule arguments — `numbering: (..n) => …`, `#show: f`
30. Counters, state & `context` — why document-wide state needs `context`
31. Loading data — `#yaml`, `#json`, `#csv`, `#read`
- **Review**, **Project: Resume v3** (`#let entry(…)` + `cv.yaml`),
  **Project: Invoice** (CSV → table, totals),
  **Project: Legal document** (continuous affidavit numbering, see Appendix A)

### Track 6 — Real documents & templates
32. Footnotes, `#outline`, bibliography & `@citations`
33. Page furniture — headers, footers, page numbers, `context counter(page)`
34. Packages — `#import "@preview/name:ver": *`
35. Using a Typst Universe template — `#show: template.with(…)`, reading its parameters
36. Overriding a template — your rules after the template's
37. Writing your own template — turn Resume v3 into `#show: resume.with(…)`
38. Local workflow — `typst init @preview/…`, `typst watch`, Tinymist in Neovim
- **Project: Report/thesis**, **Project: Slides** (Touying)

## 5. Exercise model

Every exercise is data (RON/TOML files under `content/`), so lessons are easy
to contribute. Kinds:

| Kind | Goal shown | Check | "Ideal" |
|---|---|---|---|
| **Reproduce** (tracks 1–3) | Rendered target | Compiled output equals target's | Shortest idiomatic source (normalized length) |
| **Transform** (track 4) | Doc A rendered → doc B rendered | Output equals B | Fewest changed characters; e.g. one `#set` rule |
| **Rule** (track 5) | Target rendering of a list/heading scheme | Output equals target on the visible doc **and** on hidden variant docs | Reference solution; feedback = review, not hard XP gate |
| **Project** | Full target design | Structural match + "make it yours" free mode | n/a |

**Generators** fill templates with randomized content (words, names, numbers,
formulas), so goals never repeat exactly.

**Equality checking** compares Typst's laid-out frames (text runs, positions,
fonts, sizes, shapes) rather than pixels, which is robust and gives precise
feedback ("your heading is 14pt, target is 16pt"). The first version compares
normalized SVG output.

**Feature tagging**: each ideal solution is tagged with the features it uses
(`strong`, `heading`, `set-text`, …). XP goes to the features you used when
your solution is efficient, exactly like VimHero's per-command XP.

## 6. Architecture

Full-stack Rust. No Node, no npm, no JavaScript framework.

```
                 ┌──────────── Axum server (Rust) ────────────┐
 browser ──HTTP──▶ Leptos SSR: lesson pages as real HTML (SEO) │
                 │ server functions: auth, progress sync       │
                 │ SQLx ──▶ Postgres                           │
                 └─────────────────────────────────────────────┘
 then, in the browser: Leptos hydrates (WASM) and the Typst compiler
 runs locally — every keystroke compiles with zero network latency.
```

```
typst-hero/
├── crates/
│   ├── core/   typst-hero-core: Typst World (in-memory VFS, embedded fonts),
│   │           compile → SVG, checker, content model, generators, scoring.
│   │           Pure Rust, unit-tested natively; used by server and browser.
│   └── web/    typst-hero-web: Leptos app. `ssr` feature = Axum server binary,
│               `hydrate` feature = WASM for the browser. Built with cargo-leptos.
├── content/    lessons & exercises as data
└── PLAN.md
```

- **UI**: Leptos 0.8 (reactive components with signals, written in Rust),
  server-side rendered and hydrated. Plain CSS.
- **Server**: Axum. Serves pages, auth (GitHub OAuth), progress sync, analytics events.
- **Database**: Postgres via SQLx (compile-time checked queries, migrations).
  The DB is never on the typing path; progress is saved locally first, then synced.
- **Guests**: no account needed; progress in localStorage, merged on sign-in.
- **Analytics**: self-hosted Umami (privacy-friendly) + own event table.
- **Hosting**: one small VPS (server binary + Postgres) or a managed free tier.

- **Compiler**: the real `typst` crate (0.15) inside the app's WASM module.
- **Fonts**: `typst-assets` embedded fonts (later: lazy-loaded).
- **Editor**: plain textarea in the first slice → CodeMirror 6 via a small JS
  bridge (Vim keybindings available).
- **Packages/templates**: fetch from the Typst package registry into the
  virtual file system. If browser (CORS) access isn't allowed, ship a curated
  bundled set instead.
- **Licensing**: typst-hero is AGPL-3.0-or-later (copyleft, also covers people
  hosting modified versions). Typst is Apache-2.0, which is compatible with
  (A)GPLv3 but not GPLv2. Third-party notices in `THIRD_PARTY_NOTICES.md`.
  Backlog: generate full Rust dependency notices with `cargo about`.

### Why WebAssembly (and the alternatives)

| Option | Pros | Cons |
|---|---|---|
| **SSR server + WASM in browser (chosen)** | SEO-friendly pages, instant feedback, accounts, tiny server load | Initial download of a few MB (cached); steeper learning curve |
| Server compiles Typst (Axum + htmx) | Simplest, no WASM | Compute cost scales with users, latency per keystroke |
| Native desktop (Tauri / egui / iced) | Native speed, no download limits | Install friction kills the "try it now" funnel |
| Terminal app (ratatui, vimtutor-style) | Great for Neovim users | Rendering previews in terminals is limited |

Because all logic lives in `core`, a native or terminal frontend can be added
later without rewriting anything.

## 7. Milestones

1. ✅ **M1 — Vertical slice**: workspace, core compiles Typst natively and in WASM,
   one lesson (Paragraphs & emphasis) with a live reproduce drill and a match check.
2. 🟡 **M2 — Lesson engine**: ✅ chips, self-typing examples, lesson navigation
   (Ctrl+j/k), challenge timer, per-challenge stats (best / average / chart),
   progress in localStorage, sidebar checkmarks, fonts loaded on demand
   (download 14 MB → ~7 MB). ⬜ lesson prose in `content/` files.
3. ✅ **M3a — Engine**: skill registry, lesson prose in `content/*.toml`, per-skill
   XP and levels, Stats page (heatmap, streak, levels by group).
   **M3b — Practice + Writing**: Practice page (categories; goals/time/endless;
   review inefficient goals), remaining Track 1 lessons.
   **M3c — Polish**: keep last good render on error; optional Vim keystroke XP.
4. **M4 — Stats & accounts**: per-feature levels, heatmap, streak; review-inefficient-goals
   screen; Postgres + SQLx, GitHub sign-in, progress sync.
5. **M5 — Tracks 2–3**, CodeMirror editor, Project: Resume v1.
6. **M6 — Track 4** (transform exercises), resume v2, cover letter.
7. **M7 — Track 5** (rule exercises with hidden variants), resume v3, invoice, legal.
8. **M8 — Track 6**, packages/templates, reference pages, deploy.

### Known issues / backlog
- WASM is still ~6 MB (brotli). Remaining weight is Typst's built-in data
  (hyphenation, CSL styles, syntax definitions) and code. Next steps: hashed
  file names + immutable caching, compile in a Web Worker so the page is
  usable while it loads, measure what can be trimmed.
- ✅ CodeMirror editor with Vim mode (toggle, remembered) and Typst syntax
  highlighting from Typst's own parser.
- Idea: in Vim mode, also score keystroke efficiency (VimHero-style) on top of
  Typst idiom efficiency.
- While the learner's code has an error, keep showing the last good render
  next to the error instead of replacing it.

## Appendix A — Capstone: continuous affidavit numbering

```typst
#let ec = counter("affidavit")
#let sub = ("(a)", "(i)", "(A)")   // styles for levels 2, 3, 4

#set enum(full: true, numbering: (..n) => {
  let nums = n.pos()
  if nums.len() == 1 {
    ec.step()
    context numbering("1.", ..ec.get())
  } else {
    numbering(sub.at(nums.len() - 2, default: "(1)"), nums.last())
  }
})
```

Concepts: `#let`, arrays & methods, closures, argument sinks, `if`, functions
as set-rule arguments, numbering patterns, counters & `context`. Track 5
builds exactly these, one lesson each.
