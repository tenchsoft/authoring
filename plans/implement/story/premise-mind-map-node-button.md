# Implement: premise-mind-map-node-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click the "Premise" mind-map node in the MindMap right-panel tab opens a detail editor for the story premise.
- design: mind-map grid with 2x2 node layout; Premise is the first node (index 0).

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` (right panel row hit-test) | MindMap branch hit-tests premise node rect | `grep -n 'MindMap'` within `hit_test_right_panel_row` |
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` (MindMap rendering) | Paints "Premise" node at grid position (0,0) | `grep -n 'Premise'` within `paint_tab_content` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` (automation nodes) | Emits `story.mind_map.premise` node | `grep -n 'story.mind_map.premise'` |
| `apps/story/src-tauri/src/ui/state.rs::StoryState` | May need `selected_mind_map_node` field for detail editor | `struct StoryState` |

## 필요한 변경 (의도 단위)

### 1. Mind-map node hit-test
- **입력**: PointerEvent::Down when `active_tab == StoryTab::MindMap`, point inside the premise node rect (panel_x, first_row_y, 104x38)
- **처리**: `hit_test_right_panel_row` matches the premise rect in the MindMap branch (index 0 in the iteration). Returns `Some("premise")`. Currently sets `saved_at = "selected premise"`. Future: should open a detail editor panel or inline editor for premise content.
- **출력/사이드 이펙트**: Selection state updated, `request_paint()` triggered.
- **순서/우선순위**: Checked after tab selection hit-test, before general panel row hit-test.

### 2. Automation node emission
- **입력**: `push_panel_row_nodes` iterates mind-map debug_ids when `active_tab == StoryTab::MindMap`
- **처리**: Emit node with `debug_id = "story.mind_map.premise"`, `role = "button"`, `label = "story.mind_map.premise"`, bounds at grid position (panel_x, first_row_y, 104x38).
- **출력/사이드 이펙트**: Only visible when MindMap tab is active and not in focus mode.
- **순서/우선순위**: First node in the mind-map grid.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.mind_map.premise` | Button | `"story.mind_map.premise"` | `active_tab == MindMap` and `focus_mode == false` |

## 의존
- 선행 implement: none
- 영향 받는 implement: `setting-mind-map-node-button` (shared mind-map grid layout)

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `MindMap` branch in `hit_test_right_panel_row` 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
