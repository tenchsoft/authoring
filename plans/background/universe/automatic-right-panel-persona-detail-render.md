# Background: automatic-right-panel-persona-detail-render

## 한 줄 정의
캐릭터 선택 시 우측 패널에 페르소나 상세 정보가 자동으로 렌더링된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 캐릭터 선택 | 캐릭터 행 클릭 | 선택 시마다 |
| 페르소나 편집 완료 | 저장 후 | 저장 시마다 |

## Lifecycle & State
```
empty ──[select]──→ loading ──[ok]──→ populated ──[deselect]──→ empty
```

## Concurrency
- 인스턴스 정책: 단일.
- 동시성 모델: 메인 스레드 동기.
- 재진입성: 새 선택이 이전을 대체.
- 취소: 해당 없음.

## Resource budget
- 무시 가능.

## Data flow
- Read: 선택된 캐릭터의 페르소나 데이터.
- Write: 우측 패널 내용.
- Persistence: 없음.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 데이터 없음 | null 체크 | 빈 패널 표시 | 무알림 |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.right_panel.persona` | `Container` | 캐릭터 이름 | 페르소나 상세 |

## UI 인터페이스
design(`plans/design/universe/persona-edit.md`)이 페르소나 상세의 시각 정의.

## Out of scope
- 페르소나 편집 동작 (별도 spec).
