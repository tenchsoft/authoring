# Background: automatic-statistics-refresh-behavior

## 한 줄 정의
우측 패널에서 Statistics 탭이 활성화되면 `engine.statistics()`를 호출하여 통계 데이터를 자동으로 갱신한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| Statistics 탭 선택 | `active_tab == StoryTab::Stats` | 탭 전환 시 1회 |
| Paint 프레임 | 탭이 Stats인 상태에서 편집 발생 | 매 paint |

## Lifecycle & State
```
idle ──[tab=Stats]──→ refreshing ──[ok]──→ displaying
                                            │
                         ──[tab changed]──→ idle
```

- **idle**: Stats 탭이 비활성. 통계 계산 안 함.
- **refreshing**: `engine.statistics()` 호출하여 데이터 획득.
- **displaying**: 통계 행 10개 렌더링.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드에서만.
- 동시성 모델: 동기 직렬. paint 시마다 `engine.statistics()` 호출.
- 재진입성: 안전. 매 paint마다 재계산.
- 취소: 없음.

## Resource budget
- CPU: 통계 계산. 문서 크기에 선형.
- 메모리: `Statistics` 구조체 (~200 bytes).
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryEngine.statistics()` (total_words, total_characters, total_sentences, avg_sentence_length, reading_time_minutes, chapter_count, character_count, world_entry_count, timeline_event_count, glossary_entry_count).
- Write: 없음 (읽기만).
- Persistence: 메모리만.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 대용량 문서 통계 지연 | paint 프레임 드롭 | 이전 값 유지, 다음 프레임에 갱신 | 무알림 |

## Observability
- Log: N/A.
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.statistics.refresh` | `Status` | `"Statistics refresh"` | 통계 갱신 인디케이터 |
| `story.statistics.{0..9}` | `Button` | `"Statistic"` | 개별 통계 행 |

## UI 인터페이스
design(`plans/design/story/story-right-panel.md`)이 통계 탭의 시각 정의. 이 background는 `engine.statistics()` 값 제공 책임만.

## Out of scope
- 통계 데이터 캐싱 (현재 매 paint마다 재계산, 최적화는 후속).
- 통계 내보내기 (별도 spec).
