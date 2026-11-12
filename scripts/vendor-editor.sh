#!/usr/bin/env bash
# Rebuilds crates/web/js/editor.js from crates/web/js/editor-src.js.
#
# Only needed when upgrading CodeMirror or changing editor-src.js. The output
# is committed, so building and running typst-hero never requires Node.
# This script does: it installs the packages into a temporary folder and
# bundles them with esbuild.
set -euo pipefail

repo="$(cd "$(dirname "$0")/.." && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

cd "$work"
npm init -y >/dev/null
npm install --no-audit --no-fund \
  @codemirror/state@6 @codemirror/view@6 @codemirror/commands@6 \
  @replit/codemirror-vim@6 esbuild@0
cp "$repo/crates/web/js/editor-src.js" .

./node_modules/.bin/esbuild editor-src.js \
  --bundle --minify --format=esm --target=es2020 \
  --legal-comments=eof \
  --outfile="$repo/crates/web/js/editor.js"

echo "Wrote crates/web/js/editor.js"
