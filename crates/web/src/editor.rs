//! The code editor component: CodeMirror (with optional Vim keybindings),
//! driven from Rust. Syntax highlighting comes from Typst's own parser.

use leptos::html;
use leptos::prelude::*;

/// An editor bound to `code`.
///
/// - Typing updates `code` and then calls `on_edit`.
/// - Bumping `reset` replaces the editor's text with the current `code` (used
///   when a new goal starts) and returns Vim to normal mode.
/// - `vim` turns Vim keybindings on and off live; `mode` receives the current
///   Vim mode ("normal", "insert", "visual", ... or "" when Vim is off).
#[component]
pub fn CodeEditor(
    code: RwSignal<String>,
    reset: Signal<u64>,
    on_edit: Callback<()>,
    vim: Signal<bool>,
    mode: RwSignal<String>,
) -> impl IntoView {
    let container = NodeRef::<html::Div>::new();

    #[cfg(feature = "hydrate")]
    browser::attach(container, code, reset, on_edit, vim, mode);
    #[cfg(not(feature = "hydrate"))]
    let _ = (code, reset, on_edit, vim, mode);

    view! { <div class="editor" node_ref=container></div> }
}

#[cfg(feature = "hydrate")]
mod browser {
    use leptos::html;
    use leptos::prelude::*;
    use typst_hero_core::highlight::highlight_code;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/js/editor.js")]
    extern "C" {
        type Editor;

        #[wasm_bindgen(constructor)]
        fn new(
            parent: &web_sys::HtmlElement,
            doc: &str,
            vim: bool,
            on_change: &js_sys::Function,
            on_mode: &js_sys::Function,
        ) -> Editor;

        #[wasm_bindgen(method, js_name = setDoc)]
        fn set_doc(this: &Editor, doc: &str);

        #[wasm_bindgen(method, js_name = setVim)]
        fn set_vim(this: &Editor, enabled: bool);

        #[wasm_bindgen(method, js_name = setHighlights)]
        fn set_highlights(this: &Editor, ranges: Vec<u32>, classes: Vec<String>);

        #[wasm_bindgen(method)]
        fn focus(this: &Editor);

        #[wasm_bindgen(method)]
        fn destroy(this: &Editor);
    }

    /// The JS editor plus the Rust callbacks it holds (kept alive with it).
    struct Live {
        editor: Editor,
        _on_change: Closure<dyn FnMut(String)>,
        _on_mode: Closure<dyn FnMut(String)>,
    }

    fn highlight(editor: &Editor, code: &str) {
        let spans = highlight_code(code);
        let mut classes: Vec<String> = Vec::new();
        let mut ranges = Vec::with_capacity(spans.len() * 3);
        for span in spans {
            let idx = classes.iter().position(|c| c == span.class).unwrap_or_else(|| {
                classes.push(span.class.to_string());
                classes.len() - 1
            });
            ranges.extend([span.from, span.to, idx as u32]);
        }
        editor.set_highlights(ranges, classes);
    }

    pub fn attach(
        container: NodeRef<html::Div>,
        code: RwSignal<String>,
        reset: Signal<u64>,
        on_edit: Callback<()>,
        vim: Signal<bool>,
        mode: RwSignal<String>,
    ) {
        let live = StoredValue::new_local(None::<Live>);
        let with_editor = move |f: &dyn Fn(&Editor)| live.with_value(|l| l.as_ref().map(|l| f(&l.editor)));

        // Create the editor once its <div> is in the page.
        Effect::new(move |_| {
            let Some(el) = container.get() else { return };
            if live.with_value(Option::is_some) {
                return;
            }
            let on_change = Closure::<dyn FnMut(String)>::new(move |text: String| {
                code.set(text);
                on_edit.run(());
            });
            let on_mode = Closure::<dyn FnMut(String)>::new(move |m: String| mode.set(m));
            let editor = Editor::new(
                el.unchecked_ref(),
                &code.get_untracked(),
                vim.get_untracked(),
                on_change.as_ref().unchecked_ref(),
                on_mode.as_ref().unchecked_ref(),
            );
            highlight(&editor, &code.get_untracked());
            editor.focus();
            live.set_value(Some(Live { editor, _on_change: on_change, _on_mode: on_mode }));
        });

        // New goal: replace the text. (Skips the first run, which is creation.)
        Effect::new(move |prev: Option<u64>| {
            let generation = reset.get();
            if prev.is_some() {
                with_editor(&|e| {
                    e.set_doc(&code.get_untracked());
                    e.focus();
                });
            }
            generation
        });

        // Re-highlight whenever the code changes.
        Effect::new(move |_| {
            code.with(|c| with_editor(&|e| highlight(e, c)));
        });

        // Toggle Vim live.
        Effect::new(move |_| {
            let enabled = vim.get();
            with_editor(&|e| e.set_vim(enabled));
        });

        on_cleanup(move || {
            with_editor(&|e| e.destroy());
        });
    }
}
