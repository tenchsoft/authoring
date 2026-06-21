# Implement: automatic-chapter-selection-highlight-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 선택된 챕터 행이 활성 배경색(`theme.primary`)으로 하이라이트된다.
- 챕터 추가/삭제 시 하이라이트가 올바른 인덱스를 반영한다.
- focus mode에서는 챕터 트리가 숨겨지지만 선택 상태는 유지된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::paint` | Left panel 챕터 행 렌더, `is_selected` 분기 | `grep -n 'is_selected' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::select_chapter` | `selected_chapter_idx` 갱신 | `fn select_chapter` |
| `apps/story/src-tauri/src/ui/state.rs::add_chapter` | 새 챕터 추가 후 마지막 인덱스로 선택 | `fn add_chapter` |
| `apps/story/src-tauri/src/ui/state.rs::delete_current_chapter` | 삭제 후 인덱스 보정 | `fn delete_current_chapter` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.chapter.selected` 노드 emit | `grep -n 'story.chapter.selected'` |

## 필요한 변경 (의도 단위)

### 1. 선택 챕터 하이라이트 페인트
- **입력**: `!focus_mode`, `titles` 벡터 순회, `selected_chapter_idx`
- **처리**: `i == selected_chapter_idx`일 때 `bg = theme.primary`, `fg = theme.on_primary`. 그 외 `bg = theme.surface`, `fg = theme.on_background`. `fill_rounded_rect`로 행 배경 칠함.
- **출력/사이드 이펙트**: 시각적 하이라이트.
- **순서/우선순위**: 챕터 타이틀 텍스트 렌더 전.

### 2. 챕터 추가/삭제 시 하이라이트 추적
- **입력**: `add_chapter` 호출 → 엔진에 챕터 추가
- **처리**: `add_chapter`에서 `selected_chapter_idx = chapters.len() - 1`로 설정. `delete_current_chapter`에서 삭제 후 `selected_chapter_idx`가 범위 초과하면 마지막 인덱스로 보정.
- **출력/사이드 이펙트**: `selected_chapter_idx` 갱신, repaint 시 올바른 행 하이라이트.
- **순서/우선순위**: 엔진 CRUD 이후.

### 3. Focus mode에서 트리 숨김 + 상태 유지
- **입력**: `focus_mode == true`
- **처리**: left panel 페인트 스킵. `selected_chapter_idx` 값은 변경 없음.
- **출력/사이드 이펙트**: 시각적 숨김만, 상태 보존.
- **순서/우선순위**: focus mode 체크가 챕터 트리 렌더 전.

### 4. 자동화 노드
- **입력**: `!focus_mode`
- **처리**: `story.chapter.selected` Status 노드를 `selected_chapter_idx` 위치에 emit. value `"Selected chapter"`.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 개별 `story.chapter.{idx}` 노드 이후.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.chapter.selected` | Status | `"Selected chapter"` | `!focus_mode` (항상, 챕터가 1개 이상일 때) |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `chapter-tree-row-control` (동일 영역).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'is_selected\|selected_chapter_idx' mod.rs state.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
