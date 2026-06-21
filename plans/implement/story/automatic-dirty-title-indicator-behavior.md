# Implement: automatic-dirty-title-indicator-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 저장되지 않은 편집이 있으면 프로젝트 제목 옆에 별표(`*`)가 나타난다.
- 저장 후 별표가 사라진다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::paint` | 헤더 프로젝트명 렌더, `is_dirty()` 분기로 `*` 추가 | `grep -n 'is_dirty' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::is_dirty` | 엔진 dirty 상태 조회 | `fn is_dirty` |
| `apps/story/src-tauri/src/ui/state.rs::save` | `mark_saved()` 호출로 dirty 해제 | `fn save` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.dirty_title` 노드 emit | `grep -n 'story.dirty_title'` |

## 필요한 변경 (의도 단위)

### 1. Dirty 인디케이터 페인트
- **입력**: `state.is_dirty()` 반환값
- **처리**: `paint`에서 프로젝트명 텍스트를 `format!("{}{}", project_name(), if is_dirty() { " *" } else { "" })`로 구성. 제목 위치 (16.0, 32.0)에 렌더.
- **출력/사이드 이펙트**: 제목 옆 `*` 표시 또는 미표시.
- **순서/우선순위**: 헤더 배경 렌더 직후, 액션 버튼 렌더 전.

### 2. Save 시 dirty 해제
- **입력**: Save 버튼 클릭 또는 Ctrl+S
- **처리**: `save()` → `engine.mark_saved()` + `saved_at = "now"`. 이후 `is_dirty()`가 `false` 반환.
- **출력/사이드 이펙트**: 다음 repaint에서 `*` 사라짐.
- **순서/우선순위**: 엔진 mark_saved 이후.

### 3. 자동화 노드
- **입력**: `state.is_dirty() == true`
- **처리**: `story.dirty_title` Status 노드를 제목 영역 rect에 emit. rect: `(12.0, 8.0)` ~ `(210.0, 40.0)`.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: word_count 노드 이후.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.dirty_title` | Status | `"Dirty title"` | `is_dirty() == true` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-status-bar-sync-behavior` (saved_at 표시와 연동).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'is_dirty\|dirty_title' mod.rs state.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
