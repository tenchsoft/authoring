<div align="center">

# Tench Authoring

**Long-form writing tools, built in 100% Rust.**

Story · Universe — from manuscript to interactive character dialogue, all on your machine.

[![Language: Rust](https://img.shields.io/badge/Language-Rust-dea584.svg)](https://www.rust-lang.org/)
[![Framework: Tauri 2](https://img.shields.io/badge/Framework-Tauri_2-FFC140.svg)](https://v2.tauri.app/)
[![License: UNLICENSED](https://img.shields.io/badge/License-UNLICENSED-red.svg)](#license)
[![Status: Preview](https://img.shields.io/badge/Status-Preview-orange.svg)](#roadmap)
[![Pricing: $1/mo](https://img.shields.io/badge/Pricing-%241%2Fmo-1ca096.svg)](https://tenchsoft.com/pricing)

</div>

---

## Overview

Tench Authoring bundles two long-form writing tools built entirely in Rust. Both apps run locally and use Tench Engine for AI assistance — your manuscript and character lore never touch a server unless you ask them to.

## Products

| | Product | Description |
|---|---|---|
| 📖 | **Story** | Long-form manuscript editor for novelists — chapters, characters, world building, mind maps, AI-assisted drafting that respects your tone |
| 🎭 | **Universe** | Interactive character-driven storytelling — four modes (Chat, Novel, Interactive, Script), persistent character memory, scenario world builder |

## Features

### Story

- Chapter-based manuscript organization
- Character profiles with relationship maps
- World building (locations, lore, glossary)
- Mind map view for plot and character arcs
- Timeline of events
- Export to **ePub**, **PDF**, **Docx**, **Markdown**, **HTML**, plain text, Tench Story Bundle
- AI-assisted drafting (right panel) with tone preservation
- Statistics & word count tracking

### Universe

- Four story modes: **Chat**, **Novel**, **Interactive**, **Script**
- Persistent character memory across sessions
- Scenario world builder (locations, rules, characters)
- Persona editor with backstory & personality
- Branching narrative with save/load
- Pinned memory rows
- Community character sharing (local-first)

## Architecture

```
apps/<product>/src-tauri/        Product shells (Tauri 2)
crates/document-core/            Shared document model
crates/office-io/                Format readers/writers (ePub, PDF, Docx, Markdown, ...)
crates/storage-core/             Local persistence + AES-GCM encryption
crates/fs-core/                  File-system access policy
crates/engine-core/              Tench Engine client
crates/tench-ui/                 Self-built widget framework
crates/tench-ui-test/            Headless E2E harness
tools/architecture-guard/        Repo structure enforcement
```

## Build

```bash
cargo check --workspace --locked
cargo build --workspace --locked
cargo test --workspace --locked
cargo run --locked -p story    # or: universe
```

## Roadmap

- [x] Story — manuscript editor, characters, world building, AI panel
- [x] Universe — four modes, memory, persona editor
- [ ] Story — collaborative editing (local network)
- [ ] Universe — voice mode (text-to-speech)
- [ ] Mobile companion (review-only)

## Pricing

- **$1 / month per device** — every Tench app.

→ https://tenchsoft.com/pricing

## License

UNLICENSED — source available for review, binary distribution requires a subscription.

## Sister Projects

- **[Tench Office](https://github.com/tenchsoft/office)** — Docs / Sheets / Slides / Kodocs
- **[Tench Media](https://github.com/tenchsoft/media)** — View / Pixel Design / Player / Composer
- **[Tench Knowledge](https://github.com/tenchsoft/knowledge)** — Research / Study
- **[Tench Code](https://github.com/tenchsoft/code)** — AI-augmented code editor
- **[tenchsoft.com](https://tenchsoft.com)** — account, license, downloads
