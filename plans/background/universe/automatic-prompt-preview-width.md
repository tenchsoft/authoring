# Background: automatic-prompt-preview-width

## 한 줄 정의
작성창의 프롬프트 미리보기 너비가 패널 크기에 맞게 자동으로 조정된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 패널 리사이즈 | 좌/우 패널 크기 변경 | 리사이즈 시마다 |
| 모드 전환 | 활성 모드 변경 | 전환 시 |

## Lifecycle & State
```
idle ──[resize]──→ recalculating ──[ok]──→ idle
```

## Concurrency
- 인스턴스 정책: 단일.
- 동시성 모델: 메인 스레드 동기.
- 재진입성: 항상 최신 크기 기준.
- 취소: 해당 없음.

## Resource budget
- 무시 가능.

## Data flow
- Read: 패널 레이아웃 크기.
- Write: 미리보기 너비.
- Persistence: 없음.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 해당 없음 | — | — | — |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.prompt_preview` | `Container` | 너비 값 | 미리보기 영역 |

## UI 인터페이스
design(`plans/design/universe/composer-text-input.md`)이 작성창 시각 정의.

## Out of scope
- 프롬프트 내용 렌더링 (별도 spec).
