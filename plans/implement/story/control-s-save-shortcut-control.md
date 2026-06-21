# Implement: control-s-save-shortcut-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Ctrl+S 키보드 단축키로 현재 스토리를 저장한다. 헤더의 Save 버튼과 동일 동작.
- design: 저장 바로가기.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_text_event` | Ctrl+S 분기가 `state.save()` 호출 | `grep -n 'ch == "s"'` 또는 `fn on_text_event` |
| `apps/story/src-tauri/src/ui/state.rs::save` | `engine.mark_saved()` 호출, `saved_at` 갱신 | `fn save` |

## 필요한 변경 (의도 단위)

### 1. 키보드 이벤트 핸들러에 Ctrl+S 분기
- **입력**: `TextEvent::Keyboard` 이벤트, `logical_key == Character("s")` + `modifiers.control == true`, `is_pressed == true`, `is_repeat == false`
- **처리**: `state.save()` 호출. 이미 구현된 분기(`grep -n 'ch == "s"'`)가 있으므로 해당 분기가 올바르게 동작하는지 확인만 한다.
- **출력/사이드 이펙트**: `engine.mark_saved()` 호출, `saved_at` 갱신, dirty 상태 해제, `ctx.request_paint()`
- **순서/우선순위**: `Escape` 분기 이후, 일반 `Character` 분기 이전.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| (없음 — 키보드 단축키는 자동화 노드 불필요) | | | |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design/background 읽기
2. `grep -n 'ch == "s"' apps/story/src-tauri/src/ui/mod.rs`로 Ctrl+S 분기 위치 확정
3. `grep -n 'fn save' apps/story/src-tauri/src/ui/state.rs`로 상태 메서드 확인
4. 의도대로 코드 변경 (필요 시)
5. `cargo check --workspace --locked` 통과 확인
