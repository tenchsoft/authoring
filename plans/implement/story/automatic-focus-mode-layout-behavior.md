# Implement: automatic-focus-mode-layout-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- `focus_mode == true`일 때 좌우 패널이 숨겨지고 에디터가 전체 너비로 확장된다.
- Focus 버튼이 활성 상태로 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::paint` | `left_w`, `right_w` 계산에서 `focus_mode` 분기 | `grep -n 'focus_mode' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` | left panel, right panel 렌더 `!focus_mode` 가드 | `grep -n 'if !self.state.focus_mode' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` | Focus 버튼 활성 상태 | `grep -n 'Focus.*focus_mode' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::toggle_focus_mode` | `focus_mode` 토글 | `fn toggle_focus_mode` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 챕터 트리, 탭 선택 hit-test `!focus_mode` 가드 | `grep -n 'focus_mode' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.focus_layout` 노드 emit | `grep -n 'story.focus_layout'` |

## 필요한 변경 (의도 단위)

### 1. 패널 geometry 계산
- **입력**: `focus_mode` bool
- **처리**: `left_w = if focus_mode { 0.0 } else { 220.0 }`, `right_w = if focus_mode { 0.0 } else { 300.0 }`. `center_w = width - left_w - right_w`. 에디터 카드가 `center_w` 전체를 차지.
- **출력/사이드 이펙트**: 에디터 영역 확장, 패널 영역 축소.
- **순서/우선순위**: panel geometry 계산이 모든 페인트의 기준.

### 2. Left/right panel 렌더 스킵
- **입력**: `focus_mode == true`
- **처리**: left panel(챕터 트리) 렌더 블록 전체를 `if !focus_mode` 가드로 감쌈. right panel(탭 바 + 콘텐츠)도 동일.
- **출력/사이드 이펙트**: 패널 미렌더.
- **순서/우선순위**: panel geometry 이후.

### 3. Focus 버튼 활성 상태
- **입력**: `focus_mode == true`
- **처리**: 헤더 버튼 렌더에서 `action == "Focus" && focus_mode`일 때 `active = true` → `theme.primary` 배경.
- **출력/사이드 이펙트**: 버튼 시각적 활성.
- **순서/우선순위**: 헤더 렌더 내.

### 4. 이벤트 핸들링 가드
- **입력**: `focus_mode == true`
- **처리**: 챕터 트리 hit-test, 탭 선택 hit-test, right panel row hit-test 모두 `!focus_mode` 가드로 스킵.
- **출력/사이드 이펙트**: focus mode에서 패널 클릭 무시.
- **순서/우선순위**: 오버레이 hit-test 이후.

### 5. 자동화 노드
- **입력**: `focus_mode == true`
- **처리**: `story.focus_layout` Status 노드를 전체 에디터 영역에 emit. rect: `(0.0, panel_y)` ~ `(width, height - status_h)`.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 챕터/탭 노드 대신 조건부 emit.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.focus_layout` | Status | `"Focus layout"` | `focus_mode == true` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-status-bar-sync-behavior` (status bar에 "focus mode" 표시).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'focus_mode' mod.rs state.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
