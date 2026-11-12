# Third-party notices

typst-hero itself is licensed under the GNU Affero General Public License
v3.0 or later (see `LICENSE`). It includes or links the following components,
which remain under their own licenses.

## Typst (Apache-2.0)

The Typst compiler crates (`typst`, `typst-library`, `typst-layout`,
`typst-svg`, `typst-syntax`, and related crates) are compiled into the server
and the browser bundle. Copyright the Typst authors. Licensed under the Apache
License, Version 2.0: https://www.apache.org/licenses/LICENSE-2.0
Source: https://github.com/typst/typst

## Fonts (via typst-assets)

The fonts served to browsers (Libertinus Serif, DejaVu Sans Mono, New Computer
Modern and others) come from the `typst-assets` crate and are licensed under
the SIL Open Font License 1.1 and other free font licenses. Their full notices
are in the `NOTICE` file of `typst-assets`:
https://github.com/typst/typst-assets/blob/main/NOTICE

## CodeMirror and codemirror-vim (MIT)

`crates/web/js/editor.js` bundles these packages:

| Package | Copyright |
|---|---|
| @codemirror/state, @codemirror/view, @codemirror/commands, @codemirror/language | Copyright (C) 2018-2021 by Marijn Haverbeke and others |
| @replit/codemirror-vim | Copyright (C) 2018-2021 by Marijn Haverbeke and others |
| @lezer/common, @lezer/highlight, style-mod | Copyright (C) 2018 by Marijn Haverbeke and others |
| w3c-keyname | Copyright (C) 2016 by Marijn Haverbeke and others |
| crelt | Copyright (C) 2020 by Marijn Haverbeke |
| @marijn/find-cluster-break | Copyright (C) 2024 by Marijn Haverbeke |

All of the above are released under the MIT License:

> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:
>
> The above copyright notice and this permission notice shall be included in
> all copies or substantial portions of the Software.
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.

## Other Rust crates

The server and browser bundle also compile in many smaller Rust crates
(Leptos, Axum, Tokio, serde, wasm-bindgen, ...), almost all under MIT and/or
Apache-2.0. Their exact versions are listed in `Cargo.lock`; each crate's
license is on crates.io.
