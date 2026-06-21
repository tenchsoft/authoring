# Implement: conflict-mind-map-node-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/conflict-mind-map-node-button.md`): Conflict 마인드맵 노드 클릭 시 상세 편집기 열림, 선택 스타일 적용.
- design(`plans/design/story/conflict-mind-map-node-button.md`): Mind Map panel의 Conflict 노드 버튼.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` (MindMap 분기) | Conflict 노드 hit-test | `"conflict"` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` (MindMap 분기) | `story.mind_map.conflict` 노드 | `story.mind_map.conflict` 검색 |
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` (MindMap 분기) | "Conflict" 노드 렌더링 | `"Conflict"` 검색 |

## 필요한 변경 (의도 단위)
### 1. Conflict 노드 클릭 처리
- **입력**: 포인터 다운 이벤트, `active_tab == StoryTab::MindMap`, Conflict 노드 rect 내부
- **처리**: `hit_test_right_panel_row`의 MindMap 분기에서 idx=1 ("conflict") 매칭. 노드 상세 편집기 열기.
- **출력/사이드 이펙트**: Conflict 노드 선택, 상세 편집기 열림, `request_paint()`
- **순서/우선순위**: Mind Map 노드는 2x2 그리드로 배치됨

### 2. Conflict 노드 렌더링
- **입력**: `paint_tab_content` 호출, `active_tab == StoryTab::MindMap`
- **처리**: "Conflict" 텍스트가 2x2 그리드의 두 번째 위치(idx=1)에 렌더링
- **출력/사이드 이펙트**: 노드 그리기

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.mind_map.conflict` | `Button` | `"Conflict"` | `active_tab == MindMap && !focus_mode` |

## 의존
- 선행 implement: `mind-map-right-panel-tab-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `"conflict"`, `StoryTab::MindMap` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
