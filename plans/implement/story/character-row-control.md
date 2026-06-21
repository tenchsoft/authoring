# Implement: character-row-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Characters 탭에서 캐릭터 행 클릭 시 해당 캐릭터가 선택되고, 상세 에디터가 열린다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` | `StoryTab::Characters` 분기에서 캐릭터 행 렌더 | `grep -n 'Characters' panels.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` | `StoryTab::Characters` 분기에서 캐릭터 행 hit-test | `grep -n 'Characters' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` | `StoryTab::Characters` 분기에서 `story.character.{idx}` 노드 emit | `grep -n 'story.character' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | right panel row hit-test 결과 처리 | `grep -n 'hit_test_right_panel_row' mod.rs` |

## 필요한 변경 (의도 단위)

### 1. 캐릭터 행 렌더
- **입력**: `active_tab == Characters`, `doc.characters` 벡터
- **처리**: `paint_tab_content`의 Characters 분기에서 "Characters" 타이틀 후 각 캐릭터를 `paint_panel_row`로 렌더. 행 간격 40.0. 제목: `ch.name`, 상세: `ch.role`.
- **출력/사이드 이펙트**: 시각적 캐릭터 목록.
- **순서/우선순위**: 패널 타이틀 렌더 이후.

### 2. 캐릭터 행 hit-test
- **입력**: pointer down, `active_tab == Characters`, `!focus_mode`
- **처리**: `hit_test_right_panel_row`의 Characters 분기에서 `doc.characters.len()`개 행을 40.0 간격으로 hit-test. hit 시 `"character.{idx}"` 반환.
- **출력/사이드 이펙트**: 캐릭터 인덱스 label.
- **순서/우선순위**: 다른 탭 hit-test와 동일 패턴.

### 3. 클릭 액션
- **입력**: `hit_test_right_panel_row`가 `"character.{idx}"` 반환
- **처리**: 현재는 `saved_at = "selected character.{idx}"`. 향후 캐릭터 상세 에디터 열기로 교체 예정.
- **출력/사이드 이펙트**: 상태 업데이트, repaint.
- **순서/우선순위**: 다른 right panel row 액션과 동일.

### 4. 자동화 노드
- **입력**: `active_tab == Characters && !focus_mode`
- **처리**: 각 캐릭터마다 `story.character.{idx}` Button 노드를 해당 행 위치에 emit. value: `ch.name`.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: right panel content 노드 이후.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.character.{idx}` | Button | `ch.name` | `active_tab == Characters && !focus_mode && idx < characters.len()` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-right-panel-content-render-behavior` (동일 패널).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'Characters\|story.character' panels.rs mod.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
