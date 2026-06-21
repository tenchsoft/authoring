# Background: automatic-modal-layout

## 한 줄 정의
모달이 열릴 때 화면 중앙에 자동으로 배치되고, 창 크기 변경 시 위치가 재조정된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 모달 열기 | 모달 활성화 | 열기 시마다 |
| 창 크기 변경 | 모달 활성 상태 | 리사이즈 시마다 |

## Lifecycle & State
```
closed ──[open]──→ positioned ──[resize]──→ repositioned ──[close]──→ closed
```

## Concurrency
- 인스턴스 정책: 단일.
- 동시성 모델: 메인 스레드 동기.
- 재진입성: 항상 최신 창 크기 기준.
- 취소: 해당 없음.

## Resource budget
- 무시 가능.

## Data flow
- Read: 창 크기, 모달 크기.
- Write: 모달 위치.
- Persistence: 없음.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 해당 없음 | — | — | — |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.modal.active` | `Dialog` | 모달 이름 | 활성 모달 위치 |

## UI 인터페이스
design(`plans/design/universe/modal-close-x.md`)이 모달 레이아웃의 시각 정의.

## Out of scope
- 모달 내용 (별도 spec).
