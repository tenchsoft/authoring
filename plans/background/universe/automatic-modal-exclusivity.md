# Background: automatic-modal-exclusivity

## 한 줄 정의
모달이 열릴 때 기존 모달이 자동으로 닫히며, 한 번에 하나의 모달만 표시된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 모달 열기 요청 | 모달 열기 함수 호출 | 요청 시마다 |

## Lifecycle & State
```
no_modal ──[open]──→ modal_active ──[close]──→ no_modal
                         │
                         └──[open_other]──→ modal_active (기존 닫힘)
```

## Concurrency
- 인스턴스 정책: 단일 모달만 허용.
- 동시성 모델: 메인 스레드 동기.
- 재진입성: 새 모달 요청 시 기존 모달 즉시 닫힘.
- 취소: 해당 없음.

## Resource budget
- 무시 가능.

## Data flow
- Read: 현재 모달 스택.
- Write: 모달 표시 상태.
- Persistence: 없음.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 해당 없음 | — | — | — |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.modal.active` | `Dialog` | 모달 이름 / null | 활성 모달 |

## UI 인터페이스
design(`plans/design/universe/modal-close-x.md` 등)이 모달의 시각 정의.

## Out of scope
- 개별 모달 내용 (별도 spec).
