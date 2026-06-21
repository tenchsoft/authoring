# Implement: save-header-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click the "Save" header button persists the story project to disk, clears the dirty marker, and updates `saved_at` timestamp.
- design: header button row with "Save" label at fixed position (third button).

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::StoryApp::on_pointer_event` (header button dispatch) | "Save" branch calls `self.state.save()` | `grep -n '"Save"'` within `on_pointer_event` |
| `apps/story/src-tauri/src/ui/state.rs::StoryState::save` | Calls `engine.mark_saved()`, sets `saved_at` | `fn save` |
| `crates/story-core/src/engine.rs::StoryEngine::mark_saved` | Sets `dirty = false` | `fn mark_saved` |
| `crates/story-core/src/project_io.rs::save_project_bundle` | Writes ZIP bundle to disk | `fn save_project_bundle` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | Emits `story.header.save` node | `grep -n 'story.header.save'` |

## 필요한 변경 (의도 단위)

### 1. Header button hit-test and dispatch
- **입력**: PointerEvent::Down with y < 48.0, point inside the "Save" button rect (third in actions array, x = 358.0)
- **처리**: Match `"Save"` action → call `self.state.save()`. `save()` calls `self.engine.mark_saved()` (sets `dirty = false`) and sets `saved_at = "now"`. Future: should also call `save_project_bundle()` to persist to disk.
- **출력/사이드 이펙트**: `is_dirty()` returns false, status bar shows updated `saved_at`, title no longer shows " *" dirty marker. `request_paint()` triggered.
- **순서/우선순위**: Header guard (y < 48.0) ensures this runs before panel hit-tests.

### 2. Automation node emission
- **입력**: `story_automation_nodes` iterates header actions
- **처리**: For the "Save" entry (index 2), emit node with `debug_id = "story.header.save"`, `role = "button"`, `label = "Save"`.
- **출력/사이드 이펙트**: Node always visible in header.
- **순서/우선순위**: After `story.header.open`, before `story.header.export`.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.header.save` | Button | `"Save"` | Always visible |

## 의존
- 선행 implement: none
- 영향 받는 implement: `new-header-button`, `open-header-button` (shared header layout)

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `"Save"` dispatch 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
