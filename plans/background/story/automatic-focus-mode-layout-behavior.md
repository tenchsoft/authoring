# Background: automatic-focus-mode-layout-behavior

## 한 줄 정의
포커스 모드 토글 시 좌측 챕터 트리(220px)와 우측 보조 패널(300px)이 즉시 숨겨지고, 에디터가 전체 폭을 차지한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| Focus 버튼 클릭 | 헤더 Focus 버튼 hit_test | 클릭 시마다 |
| Command palette | "Focus mode" 명령 선택 | 사용자 액션 |

## Lifecycle & State
```
normal ──[toggle_focus_mode]──→ focused ──[toggle_focus_mode]──→ normal
```

- **normal**: `focus_mode=false`. 좌측 220px, 우측 300px 표시.
- **focused**: `focus_mode=true`. 좌/우 패널 폭 0px. 에디터 전체 폭.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드에서만.
- 동시성 모델: 동기 직렬. boolean 토글.
- 재진입성: 안전. 토글이 idempotent.
- 취소: 없음. 토글로 복귀.

## Resource budget
- CPU/메모리 추가 비용 없음. boolean 확인만.
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryState.focus_mode`.
- Write: `StoryState.focus_mode` (toggle 시).
- Persistence: 메모리만.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 없음 | — | — | — |

## Observability
- Log: N/A.
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.focus_layout` | `Status` | `"Focus layout"` | 포커스 모드 활성 시 전체 폭 에디터 영역 |

## UI 인터페이스
design(`plans/design/story/story-automatic-ui.md`)이 포커스 모드 레이아웃의 시각 정의. 이 background는 `focus_mode` 플래그 관리 책임만.

## Out of scope
- 포커스 모드 내 미니맵 (별도 spec).
- 포커스 모드 타이머 / 자동 해제 (별도 spec).
