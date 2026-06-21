# Background: automatic-chapter-selection-highlight-behavior

## 한 줄 정의
사용자가 챕터 트리에서 챕터를 클릭하거나 엔진이 챕터를 추가/삭제할 때, 선택된 챕터 인덱스가 자동으로 갱신되고 해당 행에 하이라이트가 표시된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 챕터 트리 클릭 | 포인터가 챕터 행 hit_test 통과 | 클릭 시마다 |
| 챕터 추가 | `engine.add_chapter()` 호출 | 사용자 액션 |
| 챕터 삭제 | `engine.delete_chapter()` 호출 | 사용자 액션 |
| 챕터 이동 | `engine.move_chapter()` 호출 | 사용자 액션 |

## Lifecycle & State
```
selected(idx=N) ──[click idx=K]──→ selected(idx=K)
      │
      └──[add_chapter]──→ selected(idx=last)
      │
      └──[delete_chapter]──→ selected(idx=min(N, len-1))
      │
      └──[move_chapter]──→ selected(idx=new_position)
```

- **selected**: `selected_chapter_idx`가 유효한 인덱스를 가리킴. 에디터와 상태 바가 해당 챕터 데이터를 렌더링.
- 인덱스가 범위를 벗어나면 `saturating_sub(1)`로 보정.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드에서만 갱신.
- 동시성 모델: 동기 직렬. paint 직전에 state 읽기.
- 재진입성: 안전. 클릭 이벤트가 연속 와도 마지막 것만 반영.
- 취소: 없음. 즉시 반영.

## Resource budget
- CPU/메모리 추가 비용 없음. `selected_chapter_idx` 정수 갱신만.
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryState.engine.get_document().chapters` (챕터 수, 타이틀).
- Write: `StoryState.selected_chapter_idx` (직접 mutate).
- Persistence: 메모리만. 세션 유지.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 인덱스 범위 초과 (삭제 후) | `idx >= chapters.len()` | `saturating_sub(1)`로 보정 | 무알림 |
| 빈 챕터 목록 | `chapters.is_empty()` | idx=0 유지, 에디터 빈 상태 | 무알림 |

## Observability
- Log: N/A (UI 상태만).
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.chapter.selected` | `Status` | 선택된 챕터 인덱스 위치 | 하이라이트된 행 |

## UI 인터페이스
design(`plans/design/story/story-chapter-tree.md`)이 선택 하이라이트의 시각 정의. 이 background는 `selected_chapter_idx` 갱신 책임만.

## Out of scope
- 챕터 내용 로딩 (별도 동작).
- 챕터 순서 변경의 undo/redo (engine에서 처리).
