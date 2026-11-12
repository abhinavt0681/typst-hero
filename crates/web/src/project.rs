//! A project page: build a real document across guided steps, then make it
//! yours and download the PDF.
//!
//! Projects use real A4 pages. Your document is saved in the browser as you
//! type, so you can leave and come back.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use typst_hero_core::projects::{Project, project};
use typst_hero_core::{Diagnostic, Rendered, compile_project};

use crate::drill::{EditorToggles, ModeBadge};
use crate::editor::CodeEditor;
use crate::fonts::use_fonts_ready;
use crate::lesson::rich;
use crate::progress::{ProjectState, use_progress};
use crate::settings::use_settings;

#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();
    move || match project(&slug()) {
        Some(p) => view! { <ProjectView project=p /> }.into_any(),
        None => view! { <p>"No such project."</p> }.into_any(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Preview {
    Yours,
    Target,
}

#[component]
fn ProjectView(project: &'static Project) -> impl IntoView {
    let store = use_progress();
    let settings = use_settings();
    let ready = use_fonts_ready();
    let slug = project.slug.as_str();
    let total = project.steps.len();

    let step = RwSignal::new(0usize);
    let code = RwSignal::new(String::new());
    let reset = RwSignal::new(0u64);
    let started = RwSignal::new(false);
    let preview = RwSignal::new(Preview::Yours);
    let solution = RwSignal::new(false);
    let vim_mode = RwSignal::new(String::new());

    // Resume saved work once progress has loaded (browser only).
    Effect::new(move |_| {
        if store.loaded.get() && !started.get_untracked() {
            if let Some(saved) = store.project(slug) {
                step.set(saved.step.min(total));
                code.set(saved.code);
                started.set(true);
            }
        }
    });

    let output = Memo::new(move |_| {
        ready.track();
        code.with(|c| compile_project(c))
    });
    let target = Memo::new(move |_| {
        ready.track();
        project.steps.get(step.get()).and_then(|s| compile_project(&s.target).ok())
    });
    let finished = move || step.get() >= total;
    let matches = Memo::new(move |_| {
        let Some(s) = project.steps.get(step.get()) else { return false };
        match (output.get(), target.get()) {
            (Ok(out), Some(t)) => code.with(|c| s.accepts(c, &out, &t)),
            _ => false,
        }
    });

    let persist = move || store.save_project(slug, ProjectState { step: step.get_untracked(), code: code.get_untracked() });
    let replace_code = move |text: String| {
        code.set(text);
        reset.update(|r| *r += 1);
    };
    // Entering a step appends its new content, if any.
    let enter_step = move |i: usize| {
        step.set(i);
        solution.set(false);
        preview.set(Preview::Yours);
        if let Some(insert) = project.steps.get(i).and_then(|s| s.insert.as_ref()) {
            replace_code(format!("{}{insert}", code.get_untracked().trim_end()));
        }
        store.set_project(slug, ProjectState { step: i, code: code.get_untracked() });
    };
    let begin = move || {
        started.set(true);
        replace_code(String::new());
        enter_step(0);
    };

    view! {
        <Title text=project.title.clone() />
        <article class="project">
            <p class="eyebrow">"Project"</p>
            <h1>{project.title.as_str()}</h1>
            {move || (!started.get()).then(|| view! {
                {project.intro.iter().map(|p| view! { <p class="lead">{rich(p.as_str())}</p> }).collect_view()}
                <ol class="step-overview">
                    {project.steps.iter().map(|s| view! { <li>{s.title.as_str()}</li> }).collect_view()}
                    <li>"Make it yours, download the PDF"</li>
                </ol>
                <button class="button" disabled=move || !ready.get() on:click=move |_| begin()>
                    {move || if ready.get() { "▶ Start the project" } else { "Loading the Typst compiler…" }}
                </button>
            })}

            <Show when=move || started.get()>
                {view! {
                <ol class="stepper">
                    {project.steps.iter().enumerate().map(|(i, s)| view! {
                        <li class:done=move || i < step.get() class:current=move || i == step.get() title=s.title.clone()>
                            {i + 1}
                        </li>
                    }).collect_view()}
                    <li class:current=finished title="Make it yours">"★"</li>
                </ol>

                <div class="project-grid">
                    <div class="project-left">
                        {move || match project.steps.get(step.get()) {
                            Some(s) => view! {
                                <h2>"Step " {step.get() + 1} ": " {s.title.as_str()}</h2>
                                {s.text.iter().map(|p| view! { <p>{rich(p.as_str())}</p> }).collect_view()}
                            }.into_any(),
                            None => view! {
                                <h2>"Make it yours"</h2>
                                {project.finale.iter().map(|p| view! { <p>{rich(p.as_str())}</p> }).collect_view()}
                            }.into_any(),
                        }}

                        <CodeEditor
                            code
                            reset=reset.into()
                            on_edit=Callback::new(move |_| persist())
                            vim=settings.vim()
                            mode=vim_mode
                            relative=settings.relative_numbers()
                            on_relative=Callback::new(move |on| settings.update(|s| s.relative_numbers = on))
                        />
                        <div class="statusline">
                            <ModeBadge mode=vim_mode />
                            {move || if finished() {
                                view! { <span class="accent">"free editing"</span> }.into_any()
                            } else if matches.get() {
                                view! { <span class="accent">"✓ matches the target"</span> }.into_any()
                            } else {
                                view! { <span>"step " {step.get() + 1} " / " {total}</span> }.into_any()
                            }}
                            <EditorToggles />
                        </div>

                        <div class="actions">
                            {move || if finished() {
                                view! {
                                    <button class="button" on:click=move |_| download_pdf(&code.get_untracked(), slug)>"⤓ Download PDF"</button>
                                    <button class="link" on:click=move |_| {
                                        if let Some(last) = project.steps.last() { replace_code(last.target.clone()); persist(); }
                                    }>"↺ reset to the example"</button>
                                }.into_any()
                            } else {
                                view! {
                                    <button class="button" disabled=move || !matches.get() on:click=move |_| enter_step(step.get_untracked() + 1)>
                                        {move || if step.get() + 1 == total { "Finish ★" } else { "Next step →" }}
                                    </button>
                                    <button class="link" on:click=move |_| solution.update(|s| *s = !*s)>
                                        {move || if solution.get() { "hide solution" } else { "show solution" }}
                                    </button>
                                }.into_any()
                            }}
                        </div>
                        {move || solution.get().then(|| project.steps.get(step.get()).map(|s| {
                            let target_code = s.target.clone();
                            view! {
                                <div class="solution">
                                    <pre>{target_code.clone()}</pre>
                                    <button class="link" on:click=move |_| { replace_code(target_code.clone()); persist(); }>
                                        "use this code"
                                    </button>
                                </div>
                            }
                        }))}
                    </div>

                    <div class="project-right">
                        <Show when=move || !finished()>
                            <div class="seg">
                                <button class="seg-btn" class:on=move || preview.get() == Preview::Yours on:click=move |_| preview.set(Preview::Yours)>"yours"</button>
                                <button class="seg-btn" class:on=move || preview.get() == Preview::Target on:click=move |_| preview.set(Preview::Target)>"target"</button>
                            </div>
                        </Show>
                        {move || {
                            let show_target = preview.get() == Preview::Target && !finished();
                            let rendered: Result<Rendered, Vec<Diagnostic>> =
                                if show_target { target.get().ok_or_else(Vec::new) } else { output.get() };
                            view! { <ProjectPreview rendered /> }
                        }}
                    </div>
                </div>
                }.into_any()}
            </Show>
        </article>
    }
}

#[component]
fn ProjectPreview(rendered: Result<Rendered, Vec<Diagnostic>>) -> impl IntoView {
    match rendered {
        Ok(r) => r.pages.into_iter().map(|svg| view! { <div class="page sheet" inner_html=svg></div> }).collect_view().into_any(),
        Err(errs) => view! {
            <ul class="errors">
                {errs.into_iter().filter(|d| d.is_error).map(|d| view! {
                    <li>{d.position.map(|(l, c)| format!("line {l}:{c} · "))} {d.message}</li>
                }).collect_view()}
            </ul>
        }
        .into_any(),
    }
}

/// Compiles the document to PDF in the browser and downloads it. Nothing is
/// sent to the server: your resume stays on your machine.
fn download_pdf(code: &str, name: &str) {
    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::JsCast;
        let world = typst_hero_core::SandboxWorld::project(code);
        let Ok(bytes) = typst_hero_core::render::export_pdf(&world) else {
            leptos::logging::error!("PDF export failed: fix the errors first");
            return;
        };
        let result = (|| -> Result<(), wasm_bindgen::JsValue> {
            let parts = js_sys::Array::of1(&js_sys::Uint8Array::from(bytes.as_slice()));
            let options = web_sys::BlobPropertyBag::new();
            options.set_type("application/pdf");
            let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options)?;
            let url = web_sys::Url::create_object_url_with_blob(&blob)?;
            let document = web_sys::window().ok_or("no window")?.document().ok_or("no document")?;
            let link: web_sys::HtmlAnchorElement = document.create_element("a")?.dyn_into()?;
            link.set_href(&url);
            link.set_download(&format!("{name}.pdf"));
            link.click();
            web_sys::Url::revoke_object_url(&url)?;
            Ok(())
        })();
        if let Err(err) = result {
            leptos::logging::error!("PDF download failed: {err:?}");
        }
    }
    #[cfg(not(feature = "hydrate"))]
    let _ = (code, name);
}
