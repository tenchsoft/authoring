# Background: automatic-cursor-placement-behavior

## 한 줄 정의
에디터에 텍스트가 렌더링될 때 커서가 마지막 텍스트 라인 아래에 자동으로 배치되며, 깜빡임 애니메이션이 적용된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| Paint 프레임 | 에디터 영역이 visible | 매 paint |
| 텍스트 편집 | `append_text`, `backspace`, `newline` | 편집 시마다 |
| 챕터 전환 | `select_chapter()` | 전환 시마다 |

## Lifecycle & State
```
positioned(y) ──[edit]──→ recompute_y ──[paint]──→ positioned(y')
```

- **positioned**: 커서가 `cursor_y` 위치에 표시. `cursor_y < max_y`일 때만 visible.
- **recompute_y**: `paint_chapter_content()`가 텍스트 라인 수에 따라 새 `cursor_y` 반환.
- 커서가 에디터 카드 하단 경계를 넘으면 숨김.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드 paint에서만.
- 동시성 모델: 동기 직렬.
- 재진입성: 안전. 매 paint마다 재계산.
- 취소: 없음.

## Resource budget
- CPU: 텍스트 라인 카운트. 텍스트 길이에 선형.
- 메모리: 추가 할당 없음.
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryState.chapter_text()` (현재 챕터 텍스트).
- Write: 없음 (paint에서 계산만).
- Persistence: 메모리만.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 빈 챕터 | `chapter_text()` == "" | 커서를 시작 위치에 배치 | 무알림 |
| 텍스트 오버플로우 | `cursor_y >= max_y` | 커서 숨김 | 무알림 |

## Observability
- Log: N/A.
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.cursor` | `Status` | `"Cursor"` | 커서 위치 인디케이터 |

## UI 인터페이스
design(`plans/design/story/story-manuscript-editor.md`)이 커서의 시각 정의. 이 background는 `cursor_y` 계산 책임만.

## Out of scope
- 멀티 커서 / 텍스트 선택 (별도 spec).
- 커서 이동 (화살표 키, 마우스 클릭 — 별도 spec).
- 스크롤 (별도 spec).
