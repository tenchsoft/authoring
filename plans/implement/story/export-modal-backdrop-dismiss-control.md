# Implement: export-modal-backdrop-dismiss-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 익스포트 모달 바깥 영역(백드롭)을 클릭하면 모달이 닫힌다.
- design: export modal 백드롭 클릭 → 닫기.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 익스포트 모달 열려 있을 때 모달 rect 외부 클릭 시 `state.close_overlays()` 호출 | `grep -n 'export_modal_rect'` 또는 `grep -n 'show_export'` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.export.backdrop` 노드 | `grep -n 'story.export.backdrop'` |

## 필요한 변경 (의도 단위)

### 1. 포인터 이벤트에서 백드롭 클릭 처리
- **입력**: `PointerEvent::Down`, `state.show_export == true`, 클릭 위치가 `export_modal_rect(size)` 외부
- **처리**: `export_modal_rect(size).contains(e.pos)`가 false이면 `state.close_overlays()` 호출. 이미 구현된 분기(`grep -n 'export_modal_rect'`)에서 `!export_modal_rect(size).contains(e.pos)` 체크 후 `close_overlays()` 호출이 있으므로 올바르게 동작하는지 확인만 한다.
- **출력/사이드 이펙트**: `show_export = false`, `show_command_palette = false`, `show_search = false`, `input_focus = Manuscript`, `ctx.request_paint()`
- **순서/우선순위**: 익스포트 포맷 행 히트 테스트 이후, 모달 rect 외부 체크. 모든 다른 포인터 분기보다 먼저 평가됨(오버레이가 열려 있을 때).

### 2. 자동화 노드 확인
- **입력**: `story_automation_nodes`에서 `state.show_export == true`일 때
- **처리**: `story.export.backdrop` 노드가 `Rect::new(0.0, 0.0, size.width, size.height)` 전체 화면에 `role = "button"`으로 emit되는지 확인.
- **출력/사이드 이펙트**: 노드 emit.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.export.backdrop` | Button | `"Export backdrop"` | `state.show_export == true` |

## 의존
- 선행 implement: `export-header-button` (모달 열기)

## 작업 절차
1. spec/design/background 읽기
2. `grep -n 'export_modal_rect' apps/story/src-tauri/src/ui/mod.rs`로 백드롭 처리 위치 확정
3. `grep -n 'story.export.backdrop' apps/story/src-tauri/src/ui/mod.rs`로 자동화 노드 확인
4. 의도대로 코드 변경 (필요 시)
5. `cargo check --workspace --locked` 통과 확인
