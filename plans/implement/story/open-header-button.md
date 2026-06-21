# Implement: open-header-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click the "Open" header button triggers native file open dialog and loads a story project from the selected `.tench-story` bundle.
- design: header button row with "Open" label at fixed position.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::StoryApp::on_pointer_event` (header button dispatch) | "Open" branch calls `self.state.open_project()` | `grep -n '"Open"'` within `on_pointer_event` |
| `apps/story/src-tauri/src/ui/state.rs::StoryState::open_project` | Marks engine saved, sets `saved_at` | `fn open_project` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | Emits `story.header.open` node | `grep -n 'story.header.open'` |
| `crates/story-core/src/project_io.rs::load_project_bundle` | Deserializes `StoryDocument` from ZIP | `fn load_project_bundle` |

## 필요한 변경 (의도 단위)

### 1. Header button hit-test and dispatch
- **입력**: PointerEvent::Down with y < 48.0, point inside the "Open" button rect (second in actions array, x = 294.0)
- **처리**: Match `"Open"` action → call `self.state.open_project()`. Currently `open_project()` calls `engine.mark_saved()` and sets `saved_at = "opened"`. Future: should trigger native file dialog, call `load_project_bundle()`, and replace engine state.
- **출력/사이드 이펙트**: Document replaced with loaded content, dirty flag cleared, `saved_at` updated. `request_paint()` triggered.
- **순서/우선순위**: Header guard (y < 48.0) ensures this runs before panel hit-tests.

### 2. Automation node emission
- **입력**: `story_automation_nodes` iterates header actions
- **처리**: For the "Open" entry (index 1), emit node with `debug_id = "story.header.open"`, `role = "button"`, `label = "Open"`.
- **출력/사이드 이펙트**: Node always visible in header.
- **순서/우선순위**: After `story.header.new` node.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.header.open` | Button | `"Open"` | Always visible |

## 의존
- 선행 implement: none
- 영향 받는 implement: `new-header-button`, `save-header-button` (shared header layout)

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `"Open"` dispatch 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
