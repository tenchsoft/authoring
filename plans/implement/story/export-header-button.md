# Implement: export-header-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 헤더의 Export 버튼을 클릭하면 익스포트 모달이 열리고, 커맨드 팔레트/검색은 닫힌다.
- design: header Export 버튼 → export modal 열기.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 헤더 "Export" 버튼 클릭 시 `state.open_export()` 호출 | `grep -n '"Export"'` 또는 `fn on_pointer_event` |
| `apps/story/src-tauri/src/ui/state.rs::open_export` | `show_export = true`, `show_command_palette = false`, `show_search = false` 설정 | `fn open_export` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.header.export` 노드 | `grep -n 'story.header.export'` |

## 필요한 변경 (의도 단위)

### 1. 포인터 이벤트에서 Export 버튼 클릭 처리
- **입력**: `PointerEvent::Down`, `y < 48.0` (헤더 영역), Export 버튼 rect 내 클릭
- **처리**: 헤더 버튼 루프에서 `"Export"` 매치 시 `state.open_export()` 호출. 이미 구현된 분기가 있으므로 올바르게 동작하는지 확인만 한다.
- **출력/사이드 이펙트**: `show_export = true`, `show_command_palette = false`, `show_search = false`, `ctx.request_paint()`
- **순서/우선순위**: 오버레이 히트 테스트 이후, 헤더 영역 `y < 48.0` 체크 내에서 처리.

### 2. 자동화 노드 확인
- **입력**: `story_automation_nodes`에서 헤더 버튼 루프
- **처리**: `story.header.export` 노드가 `Rect::new(230.0 + 3*64.0, 10.0, ...)` 위치에 `role = "button"`, value = `"Export"`로 emit되는지 확인.
- **출력/사이드 이펙트**: 노드 emit.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.header.export` | Button | `"Export"` | 항상 (헤더에 표시) |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design/background 읽기
2. `grep -n '"Export"' apps/story/src-tauri/src/ui/mod.rs`로 클릭 처리 위치 확정
3. `grep -n 'fn open_export' apps/story/src-tauri/src/ui/state.rs`로 상태 메서드 확인
4. `grep -n 'story.header.export' apps/story/src-tauri/src/ui/mod.rs`로 자동화 노드 확인
5. 의도대로 코드 변경 (필요 시)
6. `cargo check --workspace --locked` 통과 확인
