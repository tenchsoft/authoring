# Background: automatic-overlay-exclusivity-behavior

## 한 줄 정의
익스포트 모달, 커맨드 팔레트, 검색 바 중 동시에 하나만 표시된다. 새 오버레이를 열면 기존 오버레이가 자동으로 닫힌다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| Export 열기 | `open_export()` | 사용자 액션 |
| Command palette 토글 | `toggle_command_palette()` | 사용자 액션 |
| Search 토글 | `toggle_search()` | 사용자 액션 |
| Escape 키 | `close_overlays()` | 사용자 액션 |
| 백드롭 클릭 | 오버레이 영역 외부 클릭 | 사용자 액션 |

## Lifecycle & State
```
none ──[open_export]──→ export
    ──[toggle_command]──→ command
    ──[toggle_search]──→ search

export ──[toggle_command]──→ command (export 닫힘)
       ──[toggle_search]──→ search (export 닫힘)
       ──[escape/backdrop]──→ none

command ──[open_export]──→ export (command 닫힘)
         ──[toggle_search]──→ search (command 닫힘)
         ──[escape/backdrop]──→ none

search ──[open_export]──→ export (search 닫힘)
        ──[toggle_command]──→ command (search 닫힘)
        ──[escape/backdrop]──→ none
```

- 각 `open_*` / `toggle_*` 메서드는 다른 오버레이 플래그를 `false`로 설정.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드에서만.
- 동시성 모델: 동기 직렬.
- 재진입성: 안전. 플래그 설정이 atomic.
- 취소: Escape / 백드롭으로 즉시 닫기.

## Resource budget
- CPU/메모리 추가 비용 없음. boolean 플래그 3개.
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryState.show_export`, `show_command_palette`, `show_search`.
- Write: 동일 필드 (toggle/open 시).
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
| `story.overlay.exclusive` | `Status` | `"Overlay exclusive"` | 오버레이가 하나 이상 활성일 때 노드 존재 |

## UI 인터페이스
design(`plans/design/story/story-automatic-ui.md`)이 오버레이 배타성의 시각 정의. 이 background는 오버레이 플래그 상호 배타 관리 책임만.

## Out of scope
- 오버레이 내부 동작 (각 오버레이별 별도 background/design).
- 오버레이 애니메이션 (design에서 정의).
