# Background: automatic-character-selection-highlight

## 한 줄 정의
사용자가 캐릭터를 선택하면 해당 캐릭터 행이 하이라이트되고 우측 패널에 페르소나 상세가 표시된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 캐릭터 행 클릭 | 캐릭터 목록에서 행 선택 | 클릭 시마다 |

## Lifecycle & State
```
idle ──[select]──→ highlighting ──[ok]──→ idle
```

## Concurrency
- 인스턴스 정책: 단일.
- 동시성 모델: 메인 스레드 동기.
- 재진입성: 새 선택이 이전 선택을 즉시 대체.
- 취소: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능.

## Data flow
- Read: 캐릭터 목록, 선택 상태.
- Write: 하이라이트 상태, 우측 패널 데이터.
- Persistence: 없음.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 캐릭터 데이터 없음 | null 체크 | 하이라이트 해제 | 무알림 |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.character_list.selected` | `Row` | 캐릭터 이름 | 선택된 캐릭터 |

## UI 인터페이스
design(`plans/design/universe/character-row.md`)이 행의 하이라이트 시각 정의.

## Out of scope
- 캐릭터 데이터 CRUD (별도 spec).
