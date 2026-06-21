# Implement: docx-export-format-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 익스포트 모달에서 DOCX(.docx) 행을 클릭하면 DOCX 포맷으로 익스포트가 시작된다.
- design: export modal 내 포맷 선택 행.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 익스포트 모달 열려 있을 때 `hit_test_export_format`으로 행 인덱스 확인, index 0이 DOCX | `grep -n 'hit_test_export_format'` 또는 `fn on_pointer_event` |
| `apps/story/src-tauri/src/ui/mod.rs::paint_export_modal` | DOCX 행 렌더링 | `fn paint_export_modal` |
| `apps/story/src-tauri/src/ui/commands.rs::export_formats` | DOCX 포맷 라벨 (index 0) | `fn export_formats` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.export.docx` 노드 (index 0) | `grep -n 'story.export.docx'` |

## 필요한 변경 (의도 단위)

### 1. 포인터 이벤트에서 DOCX 행 클릭 처리
- **입력**: `PointerEvent::Down`, 익스포트 모달 열림 상태(`state.show_export == true`), `hit_test_export_format`이 `Some(0)` 반환
- **처리**: 이미 구현된 분기에서 `index == 0`일 때 DOCX 포맷 라벨로 `saved_at` 갱신, 모달 닫기. 실제 파일 쓰기 로직은 `crates/story-core/src/project_io.rs`의 익스포트 함수를 호출하도록 확장 필요.
- **출력/사이드 이펙트**: `state.show_export = false`, `saved_at` 갱신, `ctx.request_paint()`
- **순서/우선순위**: 익스포트 모달 히트 테스트가 다른 오버레이/헤더 분기보다 먼저 실행됨.

### 2. 자동화 노드 확인
- **입력**: `story_automation_nodes`에서 `state.show_export == true`일 때
- **처리**: `story.export.docx` 노드가 `export_format_rect(size, 0)` 위치에 `role = "button"`으로 이미 emit되는지 확인.
- **출력/사이드 이펙트**: 노드 emit.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.export.docx` | Button | `"DOCX (.docx)"` | `state.show_export == true` |

## 의존
- 선행 implement: `export-header-button` (모달 열기)

## 작업 절차
1. spec/design/background 읽기
2. `grep -n 'hit_test_export_format' apps/story/src-tauri/src/ui/mod.rs`로 클릭 처리 위치 확정
3. `grep -n 'story.export.docx' apps/story/src-tauri/src/ui/mod.rs`로 자동화 노드 확인
4. 의도대로 코드 변경 (필요 시)
5. `cargo check --workspace --locked` 통과 확인
