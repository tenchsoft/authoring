# Implement: enter-send

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Enter 키를 누르면 메시지가 전송된다.
- design: 키보드 단축키, 표시 요소 없음.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/mod.rs::on_text_event` | Enter 키 처리 | `NamedKey::Enter` |
| `apps/universe/src-tauri/src/ui/state.rs::send_input` | 메시지 전송 | `fn send_input` |

## 필요한 변경
### 1. Enter 키 처리
- **입력**: `LogicalKey::Named(NamedKey::Enter)` + `is_pressed`
- **처리**: `state.send_input()` 호출
- **출력**: 메시지 전송, repaint

## 새 자동화 노드
없음 (키보드 단축키).

## 의존
- 선행 implement: `composer-text-input`
