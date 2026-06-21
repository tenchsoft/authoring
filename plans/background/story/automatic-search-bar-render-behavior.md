# Background: automatic-search-bar-render-behavior

## 한 줄 정의
Ctrl+F 또는 커맨드 팔레트 "Search" 명령으로 검색 바가 열리면, 검색 입력 필드가 자동으로 포커스를 받고 사용자 타이핑에 따라 쿼리가 실시간으로 갱신된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| Ctrl+F | `toggle_search()` | 사용자 액션 |
| Command palette "Search" | `dispatch_command_palette(14)` | 사용자 액션 |
| 문자 입력 | `input_focus == Search` | 키 입력 시마다 |
| Backspace | `input_focus == Search` | 키 입력 시마다 |
| Escape | `close_overlays()` | 사용자 액션 |

## Lifecycle & State
```
hidden ──[toggle_search]──→ visible ──[toggle_search/escape]──→ hidden

visible:
  input_focus=Search, search_query 갱신 중
  ──[char]──→ append_search_text ──[paint]──→ render query
  ──[backspace]──→ pop query ──[paint]──→ render query
  ──[Enter]──→ input_focus=Manuscript (bar stays visible)
```

- **hidden**: `show_search=false`. 검색 바 미표시.
- **visible**: `show_search=true`. 검색 바 표시, `input_focus=Search`.

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드에서만.
- 동시성 모델: 동기 직렬.
- 재진입성: 안전. 토글이 idempotent.
- 취소: Escape로 즉시 닫기.

## Resource budget
- CPU/메모리 추가 비용 없음. 문자열 버퍼만.
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryState.search_query`, `search_case_sensitive`.
- Write: `StoryState.search_query` (append/pop), `search_case_sensitive` (toggle), `show_search` (toggle), `input_focus` (toggle).
- Persistence: 메모리만.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 빈 쿼리 | `search_query.is_empty()` | placeholder 텍스트 표시 | 무알림 |

## Observability
- Log: N/A.
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.search.bar` | `Dialog` | `"Search"` | 검색 바 컨테이너 |
| `story.search.query` | `Textbox` | 현재 쿼리 텍스트 | 검색 입력 |
| `story.search.case_sensitive` | `Checkbox` | `"Case sensitive"` | 대소문자 구분 토글 |

## UI 인터페이스
design(`plans/design/story/story-automatic-ui.md`)이 검색 바의 시각 정의. 이 background는 검색 바 상태 관리 책임만.

## Out of scope
- 검색 결과 하이라이트 (별도 spec).
- 찾기/바꾸기 (별도 spec).
