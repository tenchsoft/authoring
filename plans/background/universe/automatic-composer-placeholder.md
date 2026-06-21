# Background: automatic-composer-placeholder

## 한 줄 정의
작성창의 플레이스홀더 텍스트가 현재 모드와 컨텍스트에 따라 자동으로 변경된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 모드 전환 | 활성 모드 변경 | 전환 시마다 |
| 입력창 포커스 | 빈 입력창에 포커스 | 포커스 시 |
| 메시지 전송 완료 | 입력창 비워짐 | 전송 후 |

## Lifecycle & State
```
idle ──[context_change]──→ updating ──[ok]──→ idle
```

## Concurrency
- 인스턴스 정책: 단일.
- 동시성 모델: 메인 스레드 동기.
- 재진입성: 항상 최신 컨텍스트 기준.
- 취소: 해당 없음.

## Resource budget
- 무시 가능.

## Data flow
- Read: 현재 모드, 캐릭터 선택 상태.
- Write: 플레이스홀더 텍스트.
- Persistence: 없음.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 해당 없음 | — | — | — |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.composer.placeholder` | `Label` | 플레이스홀더 텍스트 | 입력 안내 |

## UI 인터페이스
design(`plans/design/universe/composer-text-input.md`)이 입력창의 시각 정의.

## Out of scope
- 사용자 입력 처리 (별도 spec).
