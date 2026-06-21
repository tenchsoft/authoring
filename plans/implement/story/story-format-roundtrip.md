# Implement: story-format-roundtrip

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Save and load a story project preserves all data: chapters (title, content, notes, status, section, order), characters (name, description, role, personality, appearance, background, custom_attributes, tags, appearance_chapters), world entries, timeline events, glossary, relationships, bookmarks, comments, version history, writing goals, and metadata.
- Uses `project_io::save_project_bundle` / `load_project_bundle` for `.tench-story` ZIP format with `story.json` inside.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `crates/story-core/src/project_io.rs::save_project_bundle` | Serializes StoryDocument to JSON, writes ZIP | `fn save_project_bundle` |
| `crates/story-core/src/project_io.rs::load_project_bundle` | Reads ZIP, deserializes story.json to StoryDocument | `fn load_project_bundle` |
| `crates/story-core/src/models.rs::StoryDocument::to_json` | Serde serialization | `fn to_json` |
| `crates/story-core/src/models.rs::StoryDocument::from_json` | Serde deserialization | `fn from_json` |
| `crates/story-core/src/models.rs` (all model structs) | Serde `Serialize`/`Deserialize` derives | `grep -n 'Serialize, Deserialize'` in models.rs |
| `crates/story-core/src/engine.rs::StoryEngine` | `dirty` flag, undo/redo stacks (transient, not serialized) | `struct StoryEngine` |

## 필요한 변경 (의도 단위)

### 1. Serialization coverage
- **입력**: `StoryDocument` with all fields populated
- **처리**: `to_json()` uses `serde_json::to_string_pretty`. Every field in `StoryDocument` and its nested structs must have `#[serde(default)]` on optional/vector fields for forward compatibility. All structs derive `Serialize, Deserialize`.
- **출력/사이드 이펙트**: JSON string containing all data. Missing fields in older formats default gracefully.
- **순서/우선순위**: N/A — pure data transformation.

### 2. ZIP bundle save
- **입력**: `StoryDocument` and file path
- **처리**: `save_project_bundle` serializes to JSON, creates ZIP with `story.json` entry using Deflated compression, writes to disk. Creates parent directories if needed.
- **출력/사이드 이펙트**: `.tench-story` file on disk.
- **순서/우선순위**: N/A.

### 3. ZIP bundle load
- **입력**: File path to `.tench-story` bundle
- **처리**: `load_project_bundle` opens file, reads ZIP archive, extracts `story.json`, deserializes via `from_json`.
- **출력/사이드 이펙트**: `StoryDocument` with all fields restored. Error if `story.json` missing or malformed.
- **순서/우선순위**: N/A.

### 4. Roundtrip verification
- **입력**: A fully populated `StoryDocument`
- **처리**: Save → load → compare. All fields must be equal: `metadata`, `chapters` (including content, notes, status, section, order), `characters`, `relationships`, `world_entries`, `timeline_events`, `glossary`, `bookmarks`, `comments`, `version_history`, `writing_goals`.
- **출력/사이드 이펙트**: Test passes if all fields match.
- **순서/우선순위**: Existing test `save_load_roundtrip` in `project_io.rs` covers basic case; should be extended to cover all field types.

## 새 자동화 노드
No new automation nodes — this is a backend data integrity feature.

## 의존
- 선행 implement: none
- 영향 받는 implement: `save-header-button`, `open-header-button` (depend on save/load working correctly)

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `save_project_bundle`, `load_project_bundle`, `to_json`, `from_json` 위치 확정
3. Verify all model structs have proper Serde derives and `#[serde(default)]` on optional fields
4. Extend `save_load_roundtrip` test to cover all field types
5. `cargo test -p tench-story-core --locked` 통과 확인
6. `cargo check --workspace --locked` 통과 확인
