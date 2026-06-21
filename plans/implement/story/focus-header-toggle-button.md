# Implement: focus-header-toggle-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 헤더의 Focus 버튼을 클릭하면 포커스 모드가 토글된다. 포커스 모드에서는 좌우 패널이 숨겨지고 원고 편집 영역이 전체 너비로 확장된다.
- design: header Focus 버튼 → focus mode 토글.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 헤더 "Focus" 버튼 클릭 시 `state.toggle_focus_mode()` 호출 | `grep -n '"Focus"'` 또는 `fn on_pointer_event` |
| `apps/story/src-tauri/src/ui/state.rs::toggle_focus_mode` | `focus_mode` 필드 토글 | `fn toggle_focus_mode` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` | `focus_mode`에 따라 `left_w`, `right_w`를 0으로 설정 | `grep -n 'focus_mode'` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.header.focus` 노드, `story.focus_layout` 노드 | `grep -n 'story.header.focus'` |

## 필요한 변경 (의도 단위)

### 1. 포인터 이벤트에서 Focus 버튼 클릭 처리
- **입력**: `PointerEvent::Down`, `y < 48.0` (헤더 영역), Focus 버튼 rect 내 클릭
- **처리**: 헤더 버튼 루프에서 `"Focus"` 매치 시 `state.toggle_focus_mode()` 호출. 이미 구현된 분기가 있으므로 올바르게 동작하는지 확인만 한다.
- **출력/사이드 이펙트**: `focus_mode` 토글, 패널 너비 0/원복, `ctx.request_paint()`
- **순서/우선순위**: 오버레이 히트 테스트 이후, 헤더 영역 내에서 처리.

### 2. 페인트에서 포커스 모드 레이아웃
- **입력**: `state.focus_mode == true`
- **처리**: `left_w = 0.0`, `right_w = 0.0`으로 설정하여 좌우 패널 숨김, 중앙 편집 영역이 전체 너비 차지.
- **출력/사이드 이펙트**: 챕터 트리, 탭 바, 패널 콘텐츠 미렌더링.

### 3. 자동화 노드 확인
- **입력**: `story_automation_nodes`에서
- **처리**: `story.header.focus` 노드가 헤더 버튼 위치에 emit, `focus_mode == true`일 때 `story.focus_layout` 노드 emit.
- **출력/사이드 이펙트**: 노드 emit.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.header.focus` | Button | `"Focus"` | 항상 (헤더에 표시) |
| `story.focus_layout` | Status | `"Focus layout"` | `state.focus_mode == true` |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design/background 읽기
2. `grep -n '"Focus"' apps/story/src-tauri/src/ui/mod.rs`로 클릭 처리 위치 확정
3. `grep -n 'fn toggle_focus_mode' apps/story/src-tauri/src/ui/state.rs`로 상태 메서드 확인
4. `grep -n 'story.header.focus' apps/story/src-tauri/src/ui/mod.rs`로 자동화 노드 확인
5. 의도대로 코드 변경 (필요 시)
6. `cargo check --workspace --locked` 통과 확인
