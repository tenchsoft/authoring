# Implement: send-message

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 전송 버튼을 클릭하면 메시지가 전송된다.
- design: 작성기 우측 ACCENT_UNIVERSE 둥근 버튼.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::send_rect` | 버튼 rect | `fn send_rect` |
| `apps/universe/src-tauri/src/ui/state.rs::send_input` | 메시지 전송 | `fn send_input` |

## 필요한 변경
### 1. 전송 버튼 클릭
- **입력**: send_rect 내 클릭 → `UniverseHit::Send` 또는 Enter 키
- **처리**: `state.send_input()` → input_text.trim()이 비어있지 않으면 user 메시지 추가, 캐릭터 응답 추가, input_text 클리어
- **출력**: messages 리스트에 2개 메시지 추가, input_text 클리어, repaint

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.composer.send` | `button` | `"Send"` | 항상 |

## 의존
- 선행 implement: `composer-text-input`
- 영향 받는 implement: `automatic-chat-message-bubble-render`
