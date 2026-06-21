# Implement: search-case-sensitive-toggle-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Toggle the case-sensitive checkbox in the search bar switches `search_case_sensitive` between true/false, affecting subsequent search results.
- design: search bar overlay with query input and case-sensitive toggle at the right end.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` (search bar hit-test) | Click on case_rect toggles `search_case_sensitive` | `grep -n 'search_case_rect'` within `on_pointer_event` |
| `apps/story/src-tauri/src/ui/mod.rs::search_case_rect` | Computes checkbox bounds | `fn search_case_rect` |
| `apps/story/src-tauri/src/ui/editor.rs::paint_search_bar` | Paints "Search (case-sensitive)" label when active | `fn paint_search_bar` |
| `apps/story/src-tauri/src/ui/state.rs::StoryState::toggle_search_case_sensitive` | Flips `search_case_sensitive` bool | `fn toggle_search_case_sensitive` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | Emits `story.search.case_sensitive` node | `grep -n 'story.search.case_sensitive'` |
| `crates/story-core/src/engine.rs::StoryEngine::search` | Uses `case_sensitive` param for search matching | `fn search` |

## 필요한 변경 (의도 단위)

### 1. Case-sensitive toggle hit-test
- **입력**: PointerEvent::Down while `show_search == true`, point inside `search_case_rect(size)` (right portion of search bar)
- **처리**: Calls `self.state.toggle_search_case_sensitive()`, which flips `search_case_sensitive` bool. If search query is non-empty, future search results should use the new flag.
- **출력/사이드 이펙트**: `search_case_sensitive` toggled, search bar label changes between "Search" and "Search (case-sensitive)". `request_paint()` triggered.
- **순서/우선순위**: Search bar hit-test runs after export/command palette checks, before header buttons.

### 2. Search bar rendering
- **입력**: `paint_search_bar` receives `case_sensitive` bool
- **처리**: When `case_sensitive == true`, paints label "Search (case-sensitive)"; otherwise "Search".
- **출력/사이드 이펙트**: Visual feedback of toggle state.

### 3. Automation node emission
- **입력**: `story_automation_nodes` when `show_search == true`
- **처리**: Emit node with `debug_id = "story.search.case_sensitive"`, `role = "checkbox"`, `label = "Case sensitive"`, bounds from `search_case_rect(size)`.
- **출력/사이드 이펙트**: Only visible when search bar is open.
- **순서/우선순위**: After `story.search.query` node.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.search.case_sensitive` | Checkbox | `"Case sensitive"` | `show_search == true` |

## 의존
- 선행 implement: `search-query-input-control` (search bar must exist)
- 영향 받는 implement: none

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `search_case_rect` 및 `toggle_search_case_sensitive` 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
