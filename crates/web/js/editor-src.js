// The code editor: CodeMirror 6 with optional Vim keybindings.
//
// This is the only hand-written JavaScript in typst-hero. It is bundled into
// `editor.js` by `scripts/vendor-editor.sh` (the bundle is committed, so
// building the app never needs Node). Rust talks to it through the `Editor`
// class; see `crates/web/src/editor.rs`.

import { Annotation, Compartment, EditorState, StateEffect, StateField } from "@codemirror/state";
import {
  Decoration,
  EditorView,
  GutterMarker,
  drawSelection,
  gutter,
  highlightActiveLine,
  keymap,
  lineNumbers,
} from "@codemirror/view";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { Vim, getCM, vim } from "@replit/codemirror-vim";

// Marks changes made by Rust (new goal), so they aren't reported back.
const programmatic = Annotation.define();

// Syntax highlighting: Rust computes ranges with Typst's parser and sends them.
const setHighlights = StateEffect.define();
const highlights = StateField.define({
  create: () => Decoration.none,
  update(deco, tr) {
    deco = deco.map(tr.changes);
    for (const effect of tr.effects) if (effect.is(setHighlights)) deco = effect.value;
    return deco;
  },
  provide: (field) => EditorView.decorations.from(field),
});
const marks = new Map();
const mark = (cls) => {
  if (!marks.has(cls)) marks.set(cls, Decoration.mark({ class: cls }));
  return marks.get(cls);
};

// Relative line numbers, like Neovim's `number` + `relativenumber`: the
// cursor's line shows its real number, others show their distance from it.
class NumberMarker extends GutterMarker {
  constructor(text) {
    super();
    this.text = text;
  }
  eq(other) {
    return other.text === this.text;
  }
  toDOM() {
    return document.createTextNode(this.text);
  }
}

function relativeLineNumbers() {
  return gutter({
    class: "cm-lineNumbers",
    lineMarker(view, line) {
      const doc = view.state.doc;
      const n = doc.lineAt(line.from).number;
      const current = doc.lineAt(view.state.selection.main.head).number;
      return new NumberMarker(String(n === current ? n : Math.abs(n - current)));
    },
    lineMarkerChange: (update) => update.selectionSet || update.docChanged,
    initialSpacer: (view) => new NumberMarker(String(view.state.doc.lines)),
    updateSpacer: (_spacer, update) => new NumberMarker(String(update.view.state.doc.lines)),
  });
}

const numbersFor = (relative) => (relative ? relativeLineNumbers() : lineNumbers());

// `:set rnu`, `:set nornu`, `:set relativenumber`, `:set rnu?` in Vim mode.
// Vim options are global, so the option talks to the editor that owns `cm`.
const editors = new WeakMap();
let relativeDefault = false;
Vim.defineOption("relativenumber", false, "boolean", ["rnu"], (value, cm) => {
  const editor = cm && editors.get(cm.cm6);
  if (value === undefined) return editor ? editor.relative : relativeDefault;
  if (editor) editor.setRelative(value, true);
  else relativeDefault = value;
});

const theme = EditorView.theme(
  {
    "&": { backgroundColor: "var(--bg)", color: "var(--text)", fontSize: "16px", borderRadius: "6px" },
    "&.cm-focused": { outline: "1px solid var(--accent)" },
    ".cm-content": { fontFamily: "var(--mono)", padding: "12px 0", caretColor: "var(--accent)" },
    ".cm-scroller": { fontFamily: "var(--mono)", lineHeight: "1.6" },
    ".cm-gutters": { backgroundColor: "var(--bg)", color: "var(--muted)", border: "none" },
    ".cm-activeLine": { backgroundColor: "rgba(255,255,255,0.03)" },
    ".cm-activeLineGutter": { backgroundColor: "transparent", color: "var(--text)" },
    ".cm-cursor": { borderLeftColor: "var(--accent)" },
    ".cm-fat-cursor": { background: "var(--accent) !important", color: "var(--bg) !important" },
    "&:not(.cm-focused) .cm-fat-cursor": { background: "none !important", outline: "1px solid var(--accent)" },
    ".cm-selectionBackground, &.cm-focused .cm-selectionBackground": { backgroundColor: "rgba(79,209,165,0.25) !important" },
  },
  { dark: true },
);

export class Editor {
  constructor(parent, doc, vimEnabled, relative, onChange, onMode, onRelative) {
    this.onMode = onMode;
    this.onRelative = onRelative;
    this.vimEnabled = vimEnabled;
    this.relative = relative;
    this.keys = new Compartment();
    this.numbers = new Compartment();
    this.view = new EditorView({
      parent,
      state: EditorState.create({
        doc,
        extensions: [
          // Vim must come first so it sees keys before the default keymap.
          this.keys.of(vimEnabled ? vim() : []),
          this.numbers.of(numbersFor(relative)),
          history(),
          drawSelection(),
          highlightActiveLine(),
          keymap.of([...defaultKeymap, ...historyKeymap, indentWithTab]),
          highlights,
          theme,
          EditorView.lineWrapping,
          EditorView.updateListener.of((update) => {
            const fromRust = update.transactions.some((tr) => tr.annotation(programmatic));
            if (update.docChanged && !fromRust) onChange(update.state.doc.toString());
          }),
        ],
      }),
    });
    editors.set(this.view, this);
    this.watchMode();
  }

  // Switch relative line numbers. `fromVim` = changed with `:set`, so tell
  // Rust to remember it.
  setRelative(relative, fromVim = false) {
    if (relative === this.relative) return;
    this.relative = relative;
    this.view.dispatch({ effects: this.numbers.reconfigure(numbersFor(relative)) });
    if (fromVim) this.onRelative(relative);
  }

  // Reports the Vim mode ("normal", "insert", "visual", ...) to Rust.
  watchMode() {
    if (!this.vimEnabled) return this.onMode("");
    const cm = getCM(this.view);
    cm.on("vim-mode-change", (e) => {
      this.mode = e.mode;
      this.onMode(e.subMode ? `${e.mode} ${e.subMode}` : e.mode);
    });
    this.mode = "normal";
    this.onMode("normal");
  }

  // Replace the whole document (a new goal). Vim goes back to normal mode.
  setDoc(doc) {
    this.view.dispatch({
      changes: { from: 0, to: this.view.state.doc.length, insert: doc },
      selection: { anchor: 0 },
      annotations: programmatic.of(true),
    });
    if (this.vimEnabled && this.mode !== "normal") Vim.handleKey(getCM(this.view), "<Esc>", "mapping");
  }

  setVim(enabled) {
    if (enabled === this.vimEnabled) return;
    this.vimEnabled = enabled;
    this.view.dispatch({ effects: this.keys.reconfigure(enabled ? vim() : []) });
    this.watchMode();
    this.view.focus();
  }

  // `ranges` is a flat [from, to, classIndex, ...] array; `classes` names them.
  setHighlights(ranges, classes) {
    const decos = [];
    for (let i = 0; i + 2 < ranges.length; i += 3) {
      decos.push(mark(classes[ranges[i + 2]]).range(ranges[i], ranges[i + 1]));
    }
    this.view.dispatch({ effects: setHighlights.of(Decoration.set(decos, true)) });
  }

  focus() {
    this.view.focus();
  }

  destroy() {
    this.view.destroy();
  }
}
