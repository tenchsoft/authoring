# Implement: search-query-input-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Type in the search bar updates `search_query` buffer and triggers full-text search across all chapters, displaying match results.
- design: search bar overlay with text input area on the left and case-sensitive toggle on the right.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` (search bar focus) | Click on search_bar_rect sets `input_focus = Search` | `grep -n 'search_bar_rect'` within `on_pointer_event` |
| `apps/story/src-tauri/src/ui/mod.rs::on_text_event` (text input routing) | Routes character input to `append_search_text` when `input_focus == Search` | `grep -n 'StoryInputFocus::Search'` within `on_text_event` |
| `apps/story/src-tauri/src/ui/editor.rs::paint_search_bar` | Paints query text or placeholder "Type to search..." | `fn paint_search_bar` |
| `apps/story/src-tauri/src/ui/state.rs::StoryState` | `search_query` field, `append_search_text`, `backspace_search` | `grep -n 'search_query'` in state.rs |
| `crates/story-core/src/engine.rs::StoryEngine::search` | Full-text search with case sensitivity | `fn search` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | Emits `story.search.query` node | `grep -n 'story.search.query'` |

## 필요한 변경 (의도 단위)

### 1. Search bar focus acquisition
- **입력**: PointerEvent::Down while `show_search == true`, point inside `search_bar_rect(size)`
- **처리**: Sets `self.state.input_focus = StoryInputFocus::Search`. This routes subsequent keyboard input to the search query buffer instead of the manuscript editor.
- **출력/사이드 이펙트**: `input_focus` changed, `request_paint()` triggered.
- **순서/우선순위**: After case-sensitive toggle check, before header buttons.

### 2. Text input routing
- **입력**: TextEvent::Keyboard with character input when `input_focus == StoryInputFocus::Search`
- **처리**: `LogicalKey::Character(ch)` → `self.state.append_search_text(ch)` appends to `search_query`. `NamedKey::Backspace` → `self.state.backspace_search()` pops last character.
- **출력/사이드 이펙트**: `search_query` updated, search bar repaints with new text. Future: should call `engine.search(&query, case_sensitive)` and display match count/highlights.
- **순서/우선순위**: Character routing checked after Escape, Ctrl+shortcuts, Enter, Backspace.

### 3. Search bar rendering
- **입력**: `paint_search_bar` receives `query` string
- **처리**: If query is empty, paints placeholder "Type to search..." in `theme.secondary`. Otherwise paints the query text in `theme.on_background`.
- **출력/사이드 이펙트**: Visual feedback of current query.

### 4. Automation node emission
- **입력**: `story_automation_nodes` when `show_search == true`
- **처리**: Emit node with `debug_id = "story.search.query"`, `role = "textbox"`, `label = "Search query"`, bounds from `search_input_rect(size)`.
- **출력/사이드 이펙트**: Only visible when search bar is open.
- **순서/우선순위**: After `story.search.bar` node, before `story.search.case_sensitive`.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.search.query` | Textbox | `"Search query"` | `show_search == true` |

## 의존
- 선행 implement: none
- 영향 받는 implement: `search-case-sensitive-toggle-control` (uses same search bar)

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `StoryInputFocus::Search` 및 `search_query` 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
