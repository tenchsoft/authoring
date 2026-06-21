# Implement: new-header-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click the "New" header button resets the entire StoryState to a fresh untitled project with one empty "Chapter 1", clearing all dirty state.
- design: header button row with "New" label at fixed position.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::StoryApp::on_pointer_event` (header button dispatch) | "New" branch replaces entire state with `StoryState::default()` | `grep -n '"New"'` within `on_pointer_event` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` (header button rendering) | Paints "New" button in header action row | `grep -n 'actions = \["New"'` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` (automation nodes) | Emits `story.header.new` node | `grep -n 'story.header.new'` |
| `apps/story/src-tauri/src/ui/state.rs::StoryState::default` | Factory for fresh state | `fn default` |

## 필요한 변경 (의도 단위)

### 1. Header button hit-test and dispatch
- **입력**: PointerEvent::Down with y < 48.0, point inside the "New" button rect (first in the actions array, starting at x=230.0)
- **처리**: Match `"New"` action → replace `self.state` with `StoryState::default()`. This creates a new `StoryEngine` with "Untitled Story", seeds one empty "Chapter 1", resets `active_tab`, `selected_chapter_idx`, `focus_mode`, all overlay flags, `search_query`, `saved_at`, and `input_focus`.
- **출력/사이드 이펙트**: Full state reset. `is_dirty()` returns false. `request_paint()` triggered for full repaint.
- **순서/우선순위**: Must be checked before chapter tree or tab hit-tests (header y < 48.0 guard ensures this).

### 2. Automation node emission
- **입력**: `story_automation_nodes` iterates header action labels
- **처리**: For the "New" entry (index 0), emit a node with `debug_id = "story.header.new"`, `role = "button"`, `label = "New"`, bounds matching the painted rect.
- **출력/사이드 이펙트**: Node always present in header region (no conditional visibility).
- **순서/우선순위**: Emitted before other header nodes in the loop.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.header.new` | Button | `"New"` | Always visible |

## 의존
- 선행 implement: none
- 영향 받는 implement: `open-header-button`, `save-header-button` (shared header layout)

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `"New"` dispatch 위치 확정 in `on_pointer_event`
3. 의도대로 코드 변경 (현재 이미 구현됨 — 검증만 수행)
4. `cargo check --workspace --locked` 통과 확인
