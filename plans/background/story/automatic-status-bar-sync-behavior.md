# Background: automatic-status-bar-sync-behavior

## 한 줄 정의
하단 상태 바가 편집 상태(autosave 타임스탬프, 챕터별 워드 카운트, 챕터 번호, 전체 워드 카운트, 포커스 모드 여부)를 매 paint마다 자동으로 동기화한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| Paint 프레임 | 모든 paint | 매 프레임 |
| 저장 | `save()` 호출 | 사용자 액션 |
| 익스포트 | 포맷 선택 | 사용자 액션 |
| 챕터 전환 | `select_chapter()` | 사용자 액션 |
| 포커스 모드 토글 | `toggle_focus_mode()` | 사용자 액션 |
| 편집 | 텍스트 입력 | 키 입력 시마다 |

## Lifecycle & State
```
synced ──[any state change]──→ recomputing ──[paint]──→ synced
```

- **synced**: 상태 바가 현재 state를 정확히 반영.
- **recomputing**: paint 시 `StoryState` 필드에서 문자열 재조합.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드 paint에서만.
- 동시성 모델: 동기 직렬.
- 재진입성: 안전. 매 paint마다 전체 재계산.
- 취소: 없음.

## Resource budget
- CPU: 문자열 포맷팅만. 미미.
- 메모리: 포맷 문자열 할당 (~100 bytes).
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryState.saved_at`, `StoryState.chapter_word_count()`, `StoryState.selected_chapter_idx`, `StoryState.total_word_count()`, `StoryState.focus_mode`.
- Write: `StoryState.saved_at` (save/export 시 갱신).
- Persistence: 메모리만.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 상태 바 텍스트 오버플로우 | 렌더링 시 width 초과 | 텍스트 축소 또는 truncation | 무알림 |

## Observability
- Log: N/A.
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.status_bar` | `Status` | `"autosaved {time}    {n} words this chapter    Ch {idx}    {total} total"` | 상태 바 전체 텍스트 |

## UI 인터페이스
design(`plans/design/story/story-automatic-ui.md`)이 상태 바의 시각 정의. 이 background는 상태 바 문자열 조합 책임만.

## Out of scope
- autosave 타이머 (별도 background).
- 상태 바 클릭 인터랙션 (현재 없음).
