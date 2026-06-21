# Background: automatic-chat-message-bubble-render

## 한 줄 정의
채팅 모드에서 메시지가 전송되거나 수신되면 자동으로 말풍선이 렌더링된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 메시지 전송 | 사용자가 전송 버튼 클릭 | 전송 시마다 |
| 메시지 수신 | AI 응답 도착 | 응답 시마다 |
| 세션 로드 | 기존 대화 열기 | 1회 |

## Lifecycle & State
```
idle ──[new_message]──→ rendering ──[ok]──→ idle
```

## Concurrency
- 인스턴스 정책: 단일 (순차 렌더).
- 동시성 모델: 메인 스레드 동기.
- 재진입성: 메시지 큐잉 후 순차 처리.
- 취소: 해당 없음.

## Resource budget
- 메모리: 대화 길이에 비례. 가상 스크롤로 제한.
- CPU: 무시 가능.

## Data flow
- Read: 대화 메시지 목록.
- Write: 말풍선 렌더 상태.
- Persistence: 대화 내용은 디스크에 저장.
- IPC: Engine을 통한 AI 응답 수신.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| AI 응답 실패 | IPC 타임아웃 | 에러 말풍선 표시 | 말풍선 내 에러 메시지 |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.chat.message.<idx>` | `Label` | 메시지 텍스트 | 대화 말풍선 |

## UI 인터페이스
design(`plans/design/universe/automatic-chat-message-bubble-render.md`)이 말풍선의 시각 정의.

## Out of scope
- AI 응답 생성 (Engine 담당).
