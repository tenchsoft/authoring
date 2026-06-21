# Background: automatic-local-runtime-status-render

## 한 줄 정의
로컬 AI 런타임의 상태(준비/로딩/오류)가 자동으로 UI에 반영된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 런타임 상태 변경 | Engine IPC 메시지 | 상태 변경 시 |
| 앱 시작 | 초기 연결 | 1회 |

## Lifecycle & State
```
disconnected ──[connect]──→ loading ──[ready]──→ ready
                              │
                              └──[error]──→ error ──[retry]──→ loading
```

## Concurrency
- 인스턴스 정책: 단일.
- 동시성 모델: IPC 메시지 수신.
- 재진입성: 상태 전이는 항상 최신 상태 반영.
- 취소: 앱 종료 시 연결 해제.

## Resource budget
- CPU: 유휴 시 0.
- 메모리: 런타임 상태 버퍼만.

## Data flow
- Read: Engine IPC 상태 메시지.
- Write: UI 상태 표시.
- Persistence: 없음.
- IPC: `engine::status` 채널.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| Engine 연결 실패 | IPC 타임아웃 | 5초 후 자동 재시도 | 상태 표시 "연결 중" |
| 모델 로드 실패 | IPC 에러 | error 상태 유지 | 상태 표시 "오류" |

## Observability
| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.runtime_status` | `Label` | "ready" / "loading" / "error" | 런타임 상태 |

## UI 인터페이스
design(`plans/design/universe/header-settings-icon.md`)이 상태 표시 위치의 시각 정의.

## Out of scope
- 모델 다운로드 (별도 spec).
- Engine 자체 동작.
