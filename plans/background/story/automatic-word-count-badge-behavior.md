# Background: automatic-word-count-badge-behavior

## 한 줄 정의
편집 시마다 헤더 우측 워드 카운트 배지가 자동으로 갱신된다. 현재 챕터 및 전체 워드 카운트를 반영.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 텍스트 편집 | `append_text`, `backspace`, `newline` | 편집 시마다 |
| 챕터 전환 | `select_chapter()` | 전환 시마다 |
| 챕터 추가/삭제 | `add_chapter`, `delete_chapter` | 사용자 액션 |

## Lifecycle & State
```
display(N) ──[edit]──→ recompute ──[ok]──→ display(N')
```

- **display**: `total_word_count()` 값을 배지에 표시.
- **recompute**: paint 시 `engine.get_document().total_word_count()` 호출. 별도 상태 없이 매 paint마다 재계산.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드 paint에서만.
- 동시성 모델: 동기 직렬.
- 재진입성: 안전. 매 paint마다 새 값 계산.
- 취소: 없음.

## Resource budget
- CPU: 텍스트 분할 카운트. 문서 크기에 선형.
- 메모리: 추가 할당 없음.
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryEngine.get_document().total_word_count()` (engine이 계산).
- Write: 없음 (읽기만).
- Persistence: 메모리만.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 대용량 문서 카운트 지연 | paint 프레임 드롭 | 현재 프레임에 구값 표시, 다음 프레임에 갱신 | 무알림 |

## Observability
- Log: N/A.
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.word_count` | `Status` | `"Word count"` | 전체 워드 카운트 배지 |

## UI 인터페이스
design(`plans/design/story/story-header.md`)이 워드 카운트 배지의 시각 정의. 이 background는 `total_word_count()` 값 제공 책임만.

## Out of scope
- 워드 카운트 계산 알고리즘 (engine에서 처리).
- 챕터별 워드 카운트 (상태 바에서 표시, 별도 동작).
