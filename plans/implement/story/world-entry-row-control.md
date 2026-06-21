# Implement: world-entry-row-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click a world entry row in the World right-panel tab selects it and opens a detail editor showing name, category, description, related entries, and related characters.
- design: panel rows with title/detail text; world entry rows show name as title and category as detail.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` (right panel row hit-test) | World branch hit-tests rows against `doc.world_entries` | `grep -n 'World'` within `hit_test_right_panel_row` |
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` (World rendering) | Paints world entry rows with name/category | `grep -n 'World'` within `paint_tab_content` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` (automation nodes) | Emits `story.world.{idx}` nodes | `grep -n 'story.world'` |
| `crates/story-core/src/models.rs::WorldEntry` | Entry data model with name, category, description, related_entries, related_characters, tags | `struct WorldEntry` |

## 필요한 변경 (의도 단위)

### 1. World entry row hit-test
- **입력**: PointerEvent::Down when `active_tab == StoryTab::World`, point inside a world entry row rect (panel_x, first_row_y + idx * 46.0, width-16, 34)
- **처리**: `hit_test_right_panel_row` iterates `doc.world_entries`, matches point against `panel_row_rect(panel_x, first_row_y, width, idx, 46.0)`. Returns `Some("world.{idx}")`. Currently sets `saved_at = "selected world.{idx}"`. Future: should open detail editor for the selected world entry.
- **출력/사이드 이펙트**: Selection state updated, `request_paint()` triggered.
- **순서/우선순위**: Checked after tab selection hit-test.

### 2. World entry row rendering
- **입력**: `paint_tab_content` with `active_tab == StoryTab::World`
- **처리**: Iterates `doc.world_entries`, paints row with title = `entry.name` and detail = `format!("{:?}", entry.category)`.
- **출력/사이드 이펙트**: Rows painted in right panel.
- **순서/우선순위**: After panel title, row by row.

### 3. Automation node emission
- **입력**: `push_panel_row_nodes` with `active_tab == StoryTab::World`
- **처리**: Iterates `doc.world_entries`, emits node per entry with `debug_id = "story.world.{idx}"`, `role = "button"`, `label = entry.name`, bounds from `panel_row_rect(panel_x, first_row_y, width, idx, 46.0)`.
- **출력/사이드 이펙트**: Only visible when World tab is active and not in focus mode.
- **순서/우선순위**: After tab nodes.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.world.{idx}` | Button | entry name | `active_tab == World` and `focus_mode == false` |

## 의존
- 선행 implement: `world-right-panel-tab-button` (tab must be selectable first)
- 영향 받는 implement: none

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `World` branch in `hit_test_right_panel_row` 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
