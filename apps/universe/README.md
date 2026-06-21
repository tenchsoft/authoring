# Tench Universe

Product app slot for character chat, scenario worlds, memory, and interactive
story modes.

Primary plan source: `~/docs/plans/universe`.

Expected shared foundations:

- `packages/app-shell`
- `packages/engine-client`
- `crates/document-core`
- `crates/search-core`
- `crates/storage-core`
- `crates/job-core`

Universe-specific code should focus on characters, scenarios, and conversation
state. Prompt routing and model calls go through Tench Engine.
