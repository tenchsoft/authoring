# Implement: command-palette-backdrop-dismiss-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/command-palette-backdrop-dismiss-control.md`): 클릭 outside command palette 시 즉시 닫히고 어떤 명령도 실행되지 않는다.
- design(`plans/design/story/command-palette-backdrop-dismiss-control.md`): Command Palette overlay의 backdrop dismiss 컨트롤.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` (command palette backdrop hit-test) | 팔레트 외부 클릭 시 close_overlays 호출 | `command_palette_rect` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` (backdrop 노드) | `story.command.backdrop` 노드 이미 존재 | `story.command.backdrop` 검색 |

## 필요한 변경 (의도 단위)
### 1. Backdrop 클릭 감지 및 팔레트 닫기
- **입력**: `PointerEvent::Down` 이벤트, `show_command_palette == true` 상태
- **처리**: 클릭 위치가 `command_palette_rect` 외부인지 확인. 외부면 `state.close_overlays()` 호출. 클릭이 command row 내부면 `hit_test_command_row`가 먼저 매칭되므로 backdrop 분기에 도달하지 않음.
- **출력/사이드 이펙트**: `show_command_palette = false`, `request_paint()`
- **순서/우선순위**: command row hit-test가 backdrop 검사보다 먼저 실행되어야 함 (현재 코드 순서 유지)

### 2. Escape 키로 동일한 닫기 동작
- **입력**: `TextEvent::Keyboard` — `NamedKey::Escape`
- **처리**: `state.close_overlays()` 호출 (이미 구현됨)
- **출력/사이드 이펙트**: 모든 오버레이 닫힘, `request_paint()`

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.command.backdrop` | `Button` | `"Command backdrop"` | `show_command_palette == true` |

## 의존
- 선행 implement: `command-palette-header-toggle-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `command_palette_rect`, `hit_test_command_row`, `close_overlays` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
