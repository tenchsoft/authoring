# Background: automatic-command-palette-render-behavior

## 한 줄 정의
Ctrl+P 또는 헤더 Cmd 버튼으로 커맨드 팔레트가 열리면, 16개 명령 리스트가 렌더링되고 클릭/선택 시 해당 명령이 즉시 실행된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| Ctrl+P | `toggle_command_palette()` | 사용자 액션 |
| Cmd 버튼 클릭 | 헤더 Cmd 버튼 hit_test | 사용자 액션 |
| Escape | `close_overlays()` | 사용자 액션 |
| 명령 행 클릭 | `hit_test_command_row()` 통과 | 클릭 시마다 |
| 백드롭 클릭 | 팔레트 영역 외부 | 사용자 액션 |

## Lifecycle & State
```
hidden ──[toggle]──→ visible ──[command click]──→ hidden (dispatch)
                     │
                     ──[escape/backdrop]──→ hidden
```

- **hidden**: `show_command_palette=false`. 팔레트 미표시.
- **visible**: `show_command_palette=true`. 팔레트 표시, 16개 명령 행 렌더링.
- **dispatch**: 선택된 인덱스의 명령 실행 후 팔레트 닫음 (export 제외 — export는 모달 열고 팔레트만 닫음).

## Concurrency
- 인스턴스 정책: 단일. 메인 스레드에서만.
- 동시성 모델: 동기 직렬.
- 재진입성: 안전. 토글이 idempotent.
- 취소: Escape / 백드롭으로 즉시 닫기.

## Resource budget
- CPU/메모리 추가 비용 없음. 정적 명령 리스트.
- 모바일/데스크톱 동일.

## Data flow
- Read: `commands::command_labels()` (정적 배열 16개).
- Write: `StoryState.show_command_palette` (toggle), 및 dispatch에 따른 state 변경.
- Persistence: 메모리만.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 잘못된 인덱스 | `index >= 16` | `_ => {}` 무시 | 무알림 |

## Observability
- Log: N/A.
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.command.backdrop` | `Button` | `"Command backdrop"` | 팔레트 외부 영역 |
| `story.command.palette` | `Dialog` | `"Command palette"` | 팔레트 컨테이너 |
| `story.command.new_project` | `Button` | `"New project"` | 명령 0 |
| `story.command.open_project` | `Button` | `"Open project"` | 명령 1 |
| `story.command.save_project` | `Button` | `"Save project"` | 명령 2 |
| `story.command.export` | `Button` | `"Export"` | 명령 3 |
| `story.command.focus_mode` | `Button` | `"Focus mode"` | 명령 4 |
| `story.command.add_chapter` | `Button` | `"Add chapter"` | 명령 5 |
| `story.command.delete_chapter` | `Button` | `"Delete chapter"` | 명령 6 |
| `story.command.undo` | `Button` | `"Undo"` | 명령 7 |
| `story.command.redo` | `Button` | `"Redo"` | 명령 8 |
| `story.command.characters_panel` | `Button` | `"Characters panel"` | 명령 9 |
| `story.command.world_panel` | `Button` | `"World building panel"` | 명령 10 |
| `story.command.timeline_panel` | `Button` | `"Timeline panel"` | 명령 11 |
| `story.command.glossary_panel` | `Button` | `"Glossary panel"` | 명령 12 |
| `story.command.statistics_panel` | `Button` | `"Statistics panel"` | 명령 13 |
| `story.command.search` | `Button` | `"Search (Ctrl+F)"` | 명령 14 |
| `story.command.ai_assist_panel` | `Button` | `"AI assist panel"` | 명령 15 |

## UI 인터페이스
design(`plans/design/story/story-automatic-ui.md`)이 커맨드 팔레트의 시각 정의. 이 background는 팔레트 상태 관리 및 명령 dispatch 책임만.

## Out of scope
- 커맨드 필터링 / 검색 (별도 spec).
- 키보드 화살표 탐색 (별도 spec).
