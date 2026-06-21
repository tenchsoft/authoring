# Repository Architecture

This repo contains the Tench Authoring product family: **Story** and **Universe**.
Shared foundations first, product shells second.

## Layers

```text
apps/story           Story — book authoring tool
apps/universe        Universe — character-driven novel and dialogue tool
crates/tench-ui      Self-built retained-mode UI framework (GPU rendering via Vello)
crates/ui-automation-core  Shared UI automation protocol
crates/tench-ui-test       Headless UI test harness
crates/storage-core  Local-first storage, cache, and data-boundary contracts
crates/fs-core       Local file, folder, permission, and workspace primitives
crates/document-core Document, annotation, note, and content primitives
crates/office-io     I/O utilities for office formats (docx, epub, html, etc.)
crates/story-core    Core logic for Story (book authoring)
crates/workspace-core Workspace, file, and project runtime boundaries
tools/architecture-guard  Enforces architectural rules and line budgets
```

## Crate Dependency Graph

```
tench-ui ──────── ui-automation-core
tench-ui-test ─── tench-ui, ui-automation-core

story-core ────── document-core, office-io
office-io ─────── document-core, fs-core, storage-core
workspace-core ── fs-core, storage-core
```

## Shared Feature Ownership

| Shared area | Rust crate | Reused by |
| --- | --- | --- |
| UI framework | `tench-ui` | story, universe |
| UI automation protocol | `ui-automation-core` | tench-ui, tench-ui-test |
| UI test harness | `tench-ui-test` | story (dev), universe (dev) |
| Local storage policy | `storage-core` | story, universe |
| Local files/permissions | `fs-core` | office-io, workspace-core |
| Documents/annotations/office content | `document-core` | story, office-io, story-core |
| Office format I/O | `office-io` | story, story-core |
| Story domain logic | `story-core` | story |
| Workspace management | `workspace-core` | story |

## Product Shell Rule

Product apps should only own product-specific composition and domain glue. If a
feature appears in multiple plan directories, it starts in a shared crate.

## Plan Mapping

| Plans | App slot | Primary shared crates |
| --- | --- | --- |
| `story` | `apps/story` | `document-core`, `office-io`, `storage-core`, `story-core`, `workspace-core` |
| `universe` | `apps/universe` | `storage-core` |
