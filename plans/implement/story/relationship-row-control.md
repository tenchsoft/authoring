# Implement: relationship-row-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click a relationship row in the Relationships right-panel tab selects it and opens a detail editor showing character names and relationship kind.
- design: panel rows with title/detail text; relationship rows show "A -> B" as title and kind as detail.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` (right panel row hit-test) | Relationships branch hit-tests rows against `doc.relationships` | `grep -n 'Relationships'` within `hit_test_right_panel_row` |
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` (Relationships rendering) | Paints relationship rows with character name lookup | `grep -n 'Relationships'` within `paint_tab_content` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` (automation nodes) | Emits `story.relationship.{idx}` nodes | `grep -n 'story.relationship'` |
| `crates/story-core/src/models.rs::CharacterRelationship` | Relationship data model with character_a_id, character_b_id, kind | `struct CharacterRelationship` |

## 필요한 변경 (의도 단위)

### 1. Relationship row hit-test
- **입력**: PointerEvent::Down when `active_tab == StoryTab::Relationships`, point inside a relationship row rect (panel_x, first_row_y + idx * 44.0, width-16, 34)
- **처리**: `hit_test_right_panel_row` iterates `doc.relationships`, matches point against `panel_row_rect(panel_x, first_row_y, width, idx, 44.0)`. Returns `Some("relationship.{idx}")`. Currently sets `saved_at = "selected relationship.{idx}"`. Future: should open detail editor for the selected relationship.
- **출력/사이드 이펙트**: Selection state updated, `request_paint()` triggered.
- **순서/우선순위**: Checked after tab selection hit-test.

### 2. Relationship row rendering
- **입력**: `paint_tab_content` with `active_tab == StoryTab::Relationships`
- **처리**: Iterates `doc.relationships`, looks up character names from `doc.characters` by matching `character_a_id` and `character_b_id`. Paints row with title `"{a_name} -> {b_name}"` and detail `rel.kind`.
- **출력/사이드 이펙트**: Rows painted in right panel.
- **순서/우선순위**: After panel title, row by row.

### 3. Automation node emission
- **입력**: `push_panel_row_nodes` with `active_tab == StoryTab::Relationships`
- **처리**: Iterates `doc.relationships`, emits node per relationship with `debug_id = "story.relationship.{idx}"`, `role = "button"`, `label = "Relationship"`, bounds from `panel_row_rect(panel_x, first_row_y, width, idx, 44.0)`.
- **출력/사이드 이펙트**: Only visible when Relationships tab is active and not in focus mode.
- **순서/우선순위**: After tab nodes.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.relationship.{idx}` | Button | `"Relationship"` | `active_tab == Relationships` and `focus_mode == false` |

## 의존
- 선행 implement: `relationships-right-panel-tab-button` (tab must be selectable first)
- 영향 받는 implement: none

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `Relationships` branch in `hit_test_right_panel_row` 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
