# Tench Story

Product app slot for long-form writing, story planning, character management,
and AI-assisted drafting.

Primary plan source: `~/docs/plans/story`.

Expected shared foundations:

- `packages/app-shell`
- `packages/engine-client`
- `crates/document-core`
- `crates/search-core`
- `crates/storage-core`

Story should own story-domain models. Editor, annotation, autosave, export, and
Engine boundaries should be shared where possible.
