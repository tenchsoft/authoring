# Implement: character-arc-mind-map-node-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Mind Map 탭에서 Character Arc 노드 클릭 시 상세 에디터가 열린다 (또는 해당 액션이 트리거된다).

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` | `StoryTab::MindMap` 분기에서 4개 그리드 노드 렌더 | `grep -n 'MindMap' panels.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` | `StoryTab::MindMap` 분기에서 4개 노드 hit-test | `grep -n 'MindMap' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` | `StoryTab::MindMap` 분기에서 `story.mind_map.character_arc` 노드 emit | `grep -n 'story.mind_map' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | right panel row hit-test 결과 처리 | `grep -n 'hit_test_right_panel_row' mod.rs` |

## 필요한 변경 (의도 단위)

### 1. Mind Map 그리드 렌더
- **입력**: `active_tab == MindMap`, `!focus_mode`
- **처리**: `paint_tab_content`의 MindMap 분기에서 4개 노드(Premise, Conflict, Setting, Character arc)를 2x2 그리드로 렌더. 각 노드: 104x38 rect, `theme.background` 배경, 6.0 라운딩. 텍스트: 11.0 BOLD, `theme.on_surface`, 중앙 정렬.
- **출력/사이드 이펙트**: 시각적 마인드 맵.
- **순서/우선순위**: 패널 타이틀 렌더 이후.

### 2. Character Arc 노드 hit-test
- **입력**: pointer down, `active_tab == MindMap`, `!focus_mode`
- **처리**: `hit_test_right_panel_row`의 MindMap 분기에서 4개 노드 rect를 순회하며 hit-test. Character arc는 idx=3 (2행 2열). rect: `(panel_x + 120.0, first_row_y + 54.0)` ~ `(panel_x + 224.0, first_row_y + 92.0)`. hit 시 `"character_arc"` 반환.
- **출력/사이드 이펙트**: `"character_arc"` label 반환.
- **순서/우선순위**: 다른 MindMap 노드와 함께 순회.

### 3. 클릭 액션
- **입력**: `hit_test_right_panel_row`가 `"character_arc"` 반환
- **처리**: 현재는 `saved_at = "selected character_arc"`. 향후 character arc 상세 에디터 열기로 교체 예정.
- **출력/사이드 이펙트**: 상태 업데이트, repaint.
- **순서/우선순위**: 다른 right panel row 액션과 동일.

### 4. 자동화 노드
- **입력**: `active_tab == MindMap && !focus_mode`
- **처리**: `story.mind_map.character_arc` Button 노드를 그리드 위치에 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 다른 MindMap 노드와 함께.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.mind_map.character_arc` | Button | `"story.mind_map.character_arc"` | `active_tab == MindMap && !focus_mode` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-right-panel-content-render-behavior` (동일 패널).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'MindMap\|mind_map' panels.rs mod.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
