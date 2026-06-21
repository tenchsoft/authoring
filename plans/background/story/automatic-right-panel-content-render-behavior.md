# Background: automatic-right-panel-content-render-behavior

## 한 줄 정의
우측 패널의 활성 탭이 변경되거나 `StoryEngine` 데이터가 갱신되면, 해당 탭에 맞는 행(row) 콘텐츠가 자동으로 리렌더링된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 탭 전환 | `active_tab` 값 변경 | 클릭 시마다 |
| 문서 데이터 변경 | 캐릭터/월드/용어집 등 수정 | 편집 시마다 |
| 포커스 모드 진입 | `focus_mode=true` | 토글 시 |
| paint 사이클 | 우측 패널 visible | 매 프레임 |

## Lifecycle & State
```
rendering(tab=X) ──[tab click→Y]──→ rendering(tab=Y)
       │
       ├──[data change]──→ rendering(tab=X, data=updated)
       │
       └──[focus_mode=true]──→ hidden ──[focus_mode=false]──→ rendering(tab=X)
```

- **rendering**: `active_tab`에 해당하는 데이터를 `StoryEngine`에서 읽어 행 목록 생성.
- **hidden**: 포커스 모드 시 패널 숨김. `active_tab` 값은 유지.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드에서만.
- 동시성 모델: 동기 직렬. paint 직전 state 읽기.
- 재진입성: 안전. 연속 데이터 변경 시 마지막 것만 반영.
- 취소: 없음. 즉시 반영.

## Resource budget
- CPU: 데이터 스캔 비용만. 캐릭터/월드 항목 수에 비례하나 1000개 이하에서 무시 가능.
- 메모리: 행 목록 임시 할당. paint 후 해제.
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryState.engine.get_document().characters`, `.world_entries`, `.glossary`, `.timeline_events`, `.statistics`, `.comments`, `.relationships`, `.mind_map_nodes` (탭별).
- Write: 없음 (읽기 전용 렌더링).
- Persistence: 없음.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 빈 데이터 | 항목 수 0 | "항목 없음" 빈 상태 표시 | 무알림 |
| stale 인덱스 | 선택된 행 인덱스 >= len | 선택 해제 | 무알림 |

## Observability
- Log: N/A (UI 상태만).
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.right_panel.content` | `Region` | 탭별 행 목록 | 활성 탭 콘텐츠 |
| `story.tab.*` | `Tab` | active / inactive | 탭 선택 상태 |

## UI 인터페이스
design(`plans/design/story/story-right-panel.md`)이 우측 패널의 시각 정의. 이 background는 탭 전환 시 데이터 갱신 책임만.

## Out of scope
- 탭별 상세 편집 동작 (별도 spec).
- 행 추가/삭제 (별도 spec).
