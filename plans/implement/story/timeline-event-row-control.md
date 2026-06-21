# Implement: timeline-event-row-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click a timeline event row in the Timeline right-panel tab selects it and opens a detail editor showing date, title, description, and related characters.
- design: panel rows with title/detail text; timeline rows show date as title and event title as detail.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` (right panel row hit-test) | Timeline branch hit-tests rows against `doc.timeline_events` | `grep -n 'Timeline'` within `hit_test_right_panel_row` |
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` (Timeline rendering) | Paints timeline event rows with date/title | `grep -n 'Timeline'` within `paint_tab_content` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` (automation nodes) | Emits `story.timeline.{idx}` nodes | `grep -n 'story.timeline'` |
| `crates/story-core/src/models.rs::TimelineEvent` | Event data model with date, title, description, related_characters, tags | `struct TimelineEvent` |

## 필요한 변경 (의도 단위)

### 1. Timeline event row hit-test
- **입력**: PointerEvent::Down when `active_tab == StoryTab::Timeline`, point inside a timeline event row rect (panel_x, first_row_y + idx * 44.0, width-16, 34)
- **처리**: `hit_test_right_panel_row` iterates `doc.timeline_events`, matches point against `panel_row_rect(panel_x, first_row_y, width, idx, 44.0)`. Returns `Some("timeline.{idx}")`. Currently sets `saved_at = "selected timeline.{idx}"`. Future: should open detail editor for the selected event.
- **출력/사이드 이펙트**: Selection state updated, `request_paint()` triggered.
- **순서/우선순위**: Checked after tab selection hit-test.

### 2. Timeline event row rendering
- **입력**: `paint_tab_content` with `active_tab == StoryTab::Timeline`
- **처리**: Iterates `doc.timeline_events`, paints row with title = `event.date` and detail = `event.title`.
- **출력/사이드 이펙트**: Rows painted in right panel.
- **순서/우선순위**: After panel title, row by row.

### 3. Automation node emission
- **입력**: `push_panel_row_nodes` with `active_tab == StoryTab::Timeline`
- **처리**: Iterates `doc.timeline_events`, emits node per event with `debug_id = "story.timeline.{idx}"`, `role = "button"`, `label = event.title`, bounds from `panel_row_rect(panel_x, first_row_y, width, idx, 44.0)`.
- **출력/사이드 이펙트**: Only visible when Timeline tab is active and not in focus mode.
- **순서/우선순위**: After tab nodes.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.timeline.{idx}` | Button | event title | `active_tab == Timeline` and `focus_mode == false` |

## 의존
- 선행 implement: `timeline-right-panel-tab-button` (tab must be selectable first)
- 영향 받는 implement: none

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `Timeline` branch in `hit_test_right_panel_row` 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
