# Implement: control-e-export-shortcut-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/control-e-export-shortcut-control.md`): Ctrl+E 누르면 Export 모달이 열림 (Export header 버튼과 동일).
- design(`plans/design/story/control-e-export-shortcut-control.md`): 키보드 단축키 컨트롤.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_text_event` (Ctrl+E 분기) | `open_export` 호출 | `"e" && e.modifiers.control` 검색 |
| `apps/story/src-tauri/src/ui/state.rs::open_export` | export 모달 열기 | `fn open_export` 검색 |

## 필요한 변경 (의도 단위)
### 1. Ctrl+E 키 이벤트 처리
- **입력**: `TextEvent::Keyboard`, `LogicalKey::Character("e")`, `modifiers.control == true`, `is_pressed && !is_repeat`
- **처리**: `state.open_export()` 호출. 이 함수는 `show_export = true`, `show_command_palette = false`, `show_search = false` 설정.
- **출력/사이드 이펙트**: Export 모달 열림, 다른 오버레이 닫힘, `request_paint()`
- **순서/우선순위**: Escape 처리보다 나중, 다른 Ctrl 조합과 동일 레벨

## 새 자동화 노드
이 단축키는 기존 `story.export.modal` 및 `story.export.backdrop` 노드를 활성화하므로 별도 노드 불필요.

## 의존
- 선행 implement: `export-header-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `on_text_event`, `"e" && e.modifiers.control` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
