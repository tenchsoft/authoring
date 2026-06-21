# Implement: statistics-row-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click a statistics row in the Stats right-panel tab expands or navigates to the source data (e.g., clicking "Total Words" could scroll to word count details).
- design: panel rows with label/value pairs; statistics rows show metric name as title and value as detail.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` (right panel row hit-test) | Stats branch hit-tests 10 fixed rows | `grep -n 'Stats'` within `hit_test_right_panel_row` |
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` (Stats rendering) | Paints 10 statistics rows from `engine.statistics()` | `grep -n 'Stats'` within `paint_tab_content` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` (automation nodes) | Emits `story.statistics.{idx}` nodes (10 fixed) | `grep -n 'story.statistics'` |
| `crates/story-core/src/engine.rs::StoryEngine::statistics` | Returns `StoryStatistics` struct | `fn statistics` |

## 필요한 변경 (의도 단위)

### 1. Statistics row hit-test
- **입력**: PointerEvent::Down when `active_tab == StoryTab::Stats`, point inside a statistics row rect (panel_x, first_row_y + idx * 38.0, width-16, 34)
- **처리**: `hit_test_right_panel_row` uses fixed count of 10, step 38.0, prefix "statistics". Returns `Some("statistics.{idx}")`. Currently sets `saved_at = "selected statistics.{idx}"`. Future: should expand the row or navigate to the source data.
- **출력/사이드 이펙트**: Selection state updated, `request_paint()` triggered.
- **순서/우선순위**: Checked after tab selection hit-test.

### 2. Statistics row rendering
- **입력**: `paint_tab_content` with `active_tab == StoryTab::Stats`
- **처리**: Calls `state.engine.statistics()` to get `StoryStatistics`. Paints 10 rows: Total Words, Characters, Sentences, Avg Sentence, Reading Time, Chapters, Character Entries, World Entries, Timeline Events, Glossary Terms.
- **출력/사이드 이펙트**: Rows painted in right panel with label/value pairs.

### 3. Automation node emission
- **입력**: `push_panel_row_nodes` with `active_tab == StoryTab::Stats`
- **처리**: Emits a "Statistics refresh" status node at index 0, then 10 button nodes with `debug_id = "story.statistics.{idx}"`, `role = "button"`, `label = "Statistic"`, bounds from `panel_row_rect(panel_x, first_row_y, width, idx, 38.0)`.
- **출력/사이드 이펙트**: Only visible when Stats tab is active and not in focus mode.
- **순서/우선순위**: After tab nodes.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.statistics.{idx}` | Button | `"Statistic"` | `active_tab == Stats` and `focus_mode == false` |

## 의존
- 선행 implement: `stats-right-panel-tab-button` (tab must be selectable first)
- 영향 받는 implement: none

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `Stats` branch in `hit_test_right_panel_row` 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
