# Implement: automatic-command-palette-render-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- `show_command_palette == true`일 때 모든 커맨드 행이 팔레트 내에 렌더된다.
- backdrop 클릭 또는 Escape로 깔끔하게 닫힌다.
- Cmd 헤더 버튼이 활성 상태일 때 `theme.primary` 배경으로 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::paint` | overlay 섹션에서 `show_command_palette` 분기 | `grep -n 'show_command_palette' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::paint_command_palette` | 팔레트 프레임 + 커맨드 행 렌더 | `fn paint_command_palette` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 팔레트 행 클릭, backdrop 클릭 처리 | `grep -n 'command_palette' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::dispatch_command_palette` | 인덱스 → 액션 매핑 | `fn dispatch_command_palette` |
| `apps/story/src-tauri/src/ui/commands.rs::command_labels` | 16개 커맨드 라벨 | `fn command_labels` |
| `apps/story/src-tauri/src/ui/state.rs::toggle_command_palette` | 오버레이 상호배타적 토글 | `fn toggle_command_palette` |
| `apps/story/src-tauri/src/ui/state.rs::close_overlays` | Escape 시 전체 닫기 | `fn close_overlays` |

## 필요한 변경 (의도 단위)

### 1. 커맨드 팔레트 렌더
- **입력**: `show_command_palette == true`
- **처리**: `paint_command_palette`에서 팔레트 rect(`command_palette_rect`)에 `theme.surface` 배경, `theme.border` 스트로크. "Command Palette" 타이틀 후 `command_labels()` 순회하며 각 라벨 텍스트 렌더. 각 행 높이 22.0, 시작 y = `palette.y0 + 60.0`.
- **출력/사이드 이펙트**: 시각적 오버레이.
- **순서/우선순위**: export modal, search bar와 함께 overlay 섹션에서 렌더. `show_command_palette` 체크가 세 번째.

### 2. Backdrop 클릭으로 닫기
- **입력**: pointer down, `show_command_palette == true`, 클릭 위치가 `command_palette_rect` 밖
- **처리**: `close_overlays()` 호출 → `show_command_palette = false`, `input_focus = Manuscript`.
- **출력/사이드 이펙트**: 팔레트 닫힘, repaint.
- **순서/우선순위**: export modal hit-test 이후.

### 3. 커맨드 행 클릭 → 액션 디스패치
- **입력**: pointer down, `show_command_palette == true`, `hit_test_command_row`가 인덱스 반환
- **처리**: `dispatch_command_palette(index)` 호출. 각 인덱스에 매핑된 액션 실행. `index != 3`(export)이면 `show_command_palette = false`.
- **출력/사이드 이펙트**: 상태 변경, 팔레트 닫힘(export 제외), repaint.
- **순서/우선순위**: backdrop 닫기 체크 전.

### 4. Cmd 버튼 활성 상태
- **입력**: `show_command_palette == true`
- **처리**: 헤더 버튼 렌더 루프에서 `action == "Cmd" && show_command_palette`일 때 `active = true` → `theme.primary` 배경, `theme.on_primary` 텍스트.
- **출력/사이드 이펙트**: 버튼 시각적 활성 상태.
- **순서/우선순위**: 헤더 렌더 내.

### 5. 자동화 노드
- **입력**: `show_command_palette == true`
- **처리**: `story.command.backdrop`(Button), `story.command.palette`(Dialog), `story.command.*` 16개 행(Button) 노드 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: export 노드 이후.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.command.backdrop` | Button | `"Command backdrop"` | `show_command_palette == true` |
| `story.command.palette` | Dialog | `"Command palette"` | `show_command_palette == true` |
| `story.command.new_project` | Button | `"New project"` | `show_command_palette == true` |
| `story.command.open_project` | Button | `"Open project"` | `show_command_palette == true` |
| `story.command.save_project` | Button | `"Save project"` | `show_command_palette == true` |
| `story.command.export` | Button | `"Export"` | `show_command_palette == true` |
| `story.command.focus_mode` | Button | `"Focus mode"` | `show_command_palette == true` |
| `story.command.add_chapter` | Button | `"Add chapter"` | `show_command_palette == true` |
| `story.command.delete_chapter` | Button | `"Delete chapter"` | `show_command_palette == true` |
| `story.command.undo` | Button | `"Undo"` | `show_command_palette == true` |
| `story.command.redo` | Button | `"Redo"` | `show_command_palette == true` |
| `story.command.characters_panel` | Button | `"Characters panel"` | `show_command_palette == true` |
| `story.command.world_panel` | Button | `"World building panel"` | `show_command_palette == true` |
| `story.command.timeline_panel` | Button | `"Timeline panel"` | `show_command_palette == true` |
| `story.command.glossary_panel` | Button | `"Glossary panel"` | `show_command_palette == true` |
| `story.command.statistics_panel` | Button | `"Statistics panel"` | `show_command_palette == true` |
| `story.command.search` | Button | `"Search (Ctrl+F)"` | `show_command_palette == true` |
| `story.command.ai_assist_panel` | Button | `"AI assist panel"` | `show_command_palette == true` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-overlay-exclusivity-behavior` (동일 오버레이 메커니즘).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'show_command_palette\|paint_command_palette\|dispatch_command_palette' mod.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
