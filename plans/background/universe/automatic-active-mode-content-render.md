# Background: automatic-active-mode-content-render

## 한 줄 정의
현재 활성 모드(채팅/소설/대화/스크립트)에 따라 메인 콘텐츠 영역이 자동으로 해당 모드의 레이아웃으로 전환된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 모드 탭 전환 | 사용자가 모드 탭 클릭 | 전환 시마다 |
| 세션 로드 | 기존 세션 열기 | 1회 |

## Lifecycle & State
```
idle ──[mode_change]──→ rendering ──[ok]──→ idle
                           │
                           └──[error]──→ idle (이전 모드 유지)
```

## Concurrency
- 인스턴스 정책: 단일.
- 동시성 모델: 메인 스레드 동기 렌더.
- 재진입성: 전환 중 새 전환 무시.
- 취소: 해당 없음.

## Resource budget
- CPU: 전환 시 즉시 완료. 모바일/데스크톱 동일.
- 메모리: 현재 모드 데이터만 유지.

## Data flow
- Read: 현재 선택된 모드 상태.
- Write: 콘텐츠 영역 레이아웃.
- Persistence: 없음 (메모리만).
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 렌더 실패 | 레이아웃 계산 오류 | 이전 모드 유지 | 무알림 |

## Observability
- Log: `tracing::debug!("mode render mode={mode}")`.
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `universe.content_area` | `Container` | 현재 모드 이름 | 활성 모드 |

## UI 인터페이스
design(`plans/design/universe/automatic-active-mode-content-render.md`)이 콘텐츠 영역의 시각적 레이아웃 정의.

## Out of scope
- 모드별 콘텐츠 데이터 로드 (별도 spec).
