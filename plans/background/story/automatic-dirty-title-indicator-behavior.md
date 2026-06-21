# Background: automatic-dirty-title-indicator-behavior

## 한 줄 정의
문서에 저장되지 않은 변경이 있으면 프로젝트 제목 옆에 `" *"` 표시가 자동으로 나타나고, 저장 후 사라진다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 텍스트 편집 | `engine.is_dirty()`가 true로 전이 | 편집 시마다 |
| 저장 | `engine.mark_saved()` 호출 | 사용자 액션 |
| 프로젝트 열기 | `open_project()` 호출 | 사용자 액션 |
| 새 프로젝트 | `StoryState::default()` 초기화 | 1회 |

## Lifecycle & State
```
clean ──[edit]──→ dirty ──[save]──→ clean
                    │
                    └──[open_project]──→ clean
```

- **clean**: 제목에 `" *"` 없음. `story.dirty_title` 노드 미생성.
- **dirty**: 제목에 `" *"` 접미어. `story.dirty_title` 노드 생성됨.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드에서만 갱신.
- 동시성 모델: 동기 직렬. paint 시 `is_dirty()` 호출.
- 재진입성: 안전. dirty 플래그는 boolean.
- 취소: 없음.

## Resource budget
- CPU/메모리 추가 비용 없음. boolean 확인만.
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryEngine.is_dirty()` (engine 내부 dirty flag).
- Write: 없음 (읽기만, 시각 표현은 paint에서 처리).
- Persistence: engine의 dirty flag가 세션 유지.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| engine 오류 | `is_dirty()` panic | unreachable (engine은 infallible) | 무알림 |

## Observability
- Log: N/A.
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.dirty_title` | `Status` | `"Dirty title"` | dirty일 때만 노드 존재 |

## UI 인터페이스
design(`plans/design/story/story-header.md`)이 더티 인디케이터의 시각 정의. 이 background는 노드 존재 여부 결정 책임만.

## Out of scope
- 자동 저장 (별도 background).
- 저장 실패 처리 (별도 background).
