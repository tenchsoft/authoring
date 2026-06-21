# Background: automatic-interactive-choice-selection-render

## 한 줄 정의
대화형 모드에서 선택지가 표시될 때 현재 선택된 선택지가 자동으로 하이라이트된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 선택지 클릭 | 사용자가 선택지 버튼 클릭 | 클릭 시마다 |
| 선택지 표시 | 새 선택지 세트 도착 | 표시 시마다 |

## Lifecycle & State
```
idle ──[choice_change]──→ rendering ──[ok]──→ idle
```

## Concurrency
- 인스턴스 정책: 단일.
- 동시성 모델: 메인 스레드 동기.
- 재진입성: 새 선택이 이전을 대체.
- 취소: 해당 없음.

## Resource budget
- 무시 가능.

## Data flow
- Read: 선택지 목록, 현재 선택 상태.
- Write: 하이라이트 상태.
- Persistence: 없음.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 해당 없음 | — | — | — |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.interactive.choice.<idx>` | `Button` | 선택지 텍스트 | 선택지 버튼 |

## UI 인터페이스
design(`plans/design/universe/interactive-choice-open-gate.md` 등)이 선택지 버튼의 시각 정의.

## Out of scope
- 선택지 액션 실행 (별도 spec).
