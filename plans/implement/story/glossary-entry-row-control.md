# Implement: glossary-entry-row-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 용어집(Glossary) 탭이 활성일 때, 패널의 각 용어 행을 클릭하면 해당 행이 선택되고 상세 편집기가 열린다.
- design: glossary 행 클릭 → 선택.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | `hit_test_right_panel_row` 결과가 `"glossary.{idx}"`일 때 선택 처리 | `grep -n 'hit_test_right_panel_row'` 또는 `fn on_pointer_event` |
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` | `StoryTab::Glossary` 분기에서 `doc.glossary.len()`만큼 행 히트 테스트 | `fn hit_test_right_panel_row` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` | `StoryTab::Glossary` 분기에서 `story.glossary.{idx}` 노드 emit | `grep -n 'story.glossary'` |
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` | `StoryTab::Glossary` 분기에서 용어/정의 행 렌더링 | `grep -n 'StoryTab::Glossary'` 또는 `fn paint_tab_content` |

## 필요한 변경 (의도 단위)

### 1. 포인터 이벤트에서 용어집 행 클릭 처리
- **입력**: `PointerEvent::Down`, `state.active_tab == StoryTab::Glossary`, `hit_test_right_panel_row`가 `Some("glossary.{idx}")` 반환
- **처리**: 이미 구현된 분기에서 `hit_test_right_panel_row` 결과로 `saved_at` 갱신. 실제 선택 상태(예: `selected_glossary_idx` 필드)가 필요하면 `StoryState`에 필드 추가 및 갱신 로직 구현.
- **출력/사이드 이펙트**: 선택된 용어 인덱스 갱신, 상세 편집기 영역 repaint, `ctx.request_paint()`
- **순서/우선순위**: 탭 선택 히트 테스트 이후, 패널 행 히트 테스트에서 처리.

### 2. 자동화 노드 확인
- **입력**: `push_panel_row_nodes`에서 `StoryTab::Glossary` 분기
- **처리**: `doc.glossary`의 각 항목에 대해 `story.glossary.{idx}` 노드가 `role = "button"`, value = `entry.term`으로 emit되는지 확인.
- **출력/사이드 이펙트**: 노드 emit.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.glossary.{idx}` | Button | `entry.term` | `state.active_tab == StoryTab::Glossary` && `!state.focus_mode` && `idx < doc.glossary.len()` |

## 의존
- 선행 implement: `glossary-right-panel-tab-button` (탭 전환)

## 작업 절차
1. spec/design/background 읽기
2. `grep -n 'hit_test_right_panel_row' apps/story/src-tauri/src/ui/mod.rs`로 클릭 처리 위치 확정
3. `grep -n 'StoryTab::Glossary' apps/story/src-tauri/src/ui/mod.rs`로 히트 테스트 및 노드 emit 확인
4. `grep -n 'StoryTab::Glossary' apps/story/src-tauri/src/ui/panels.rs`로 페인트 확인
5. 의도대로 코드 변경 (필요 시)
6. `cargo check --workspace --locked` 통과 확인
