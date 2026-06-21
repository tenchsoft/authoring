# Implement: automatic-chat-message-bubble-render

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 메시지가 전송되면 채팅 버블이 자동으로 렌더링된다.
- background: messages 리스트 변경 시 자동 트리거.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::paint_chat_messages` | 버블 렌더링 | `fn paint_chat_messages` |
| `apps/universe/src-tauri/src/ui/mod.rs::universe_automation_nodes` | message 노드 | `universe.chat.message.{idx}` |

## 필요한 변경
### 1. 메시지 버블 렌더링
- **입력**: `state.messages` 리스트
- **처리**: 각 메시지에 대해 user면 우측 정렬(NEUTRAL_700 + ACCENT_UNIVERSE 테두리), 캐릭터면 좌측 정렬(NEUTRAL_600). 발신자, 텍스트, 시간 표시.
- **출력**: center 패널에 채팅 버블 목록

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.chat.message.{idx}` | `text` | `"Message"` | messages 존재 |

## 의존
- 선행 implement: `send-message`
