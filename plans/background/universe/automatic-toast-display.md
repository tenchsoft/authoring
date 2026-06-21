# Background: automatic-toast-display

## 한 줄 정의
시스템 알림(저장 완료, 오류 등)이 자동으로 토스트로 표시되고 일정 시간 후 사라진다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 알림 이벤트 | 시스템 이벤트 발생 | 이벤트 시마다 |

## Lifecycle & State
```
hidden ──[event]──→ showing ──[3s]──→ fading ──[300ms]──→ hidden
```

## Concurrency
- 인스턴스 정책: 다중 (최대 3개 동시).
- 동시성 모델: 메인 스레드 타이머.
- 재진입성: 새 토스트가 큐에 추가.
- 취소: 해당 없음 (자동 만료).

## Resource budget
- 무시 가능.

## Data flow
- Read: 알림 메시지, 타입.
- Write: 토스트 표시 상태.
- Persistence: 없음.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 해당 없음 | — | — | — |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.toast.<idx>` | `Label` | 메시지 텍스트 | 토스트 알림 |

## UI 인터페이스
design(`plans/design/universe/automatic-toast-display.md`)이 토스트의 시각 정의.

## Out of scope
- 개별 알림 내용 (각 spec에서 정의).
