# Implement: command-palette-undo-row-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/command-palette-undo-row-button.md`): Undo command row 클릭 시 이전 StoryEngine undo 스냅샷 복원, 팔레트 닫힘.
- design(`plans/design/story/command-palette-undo-row-button.md`): Command Palette overlay의 Undo row.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::dispatch_command_palette` (index 7 분기) | `engine.undo()` 호출 | `engine.undo` 검색 |
| `apps/story/src-tauri/src/ui/commands.rs::command_labels` (index 7) | "Undo" 라벨 | `command_labels` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.command.undo` 노드 | `story.command.undo` 검색 |

## 필요한 변경 (의도 단위)
### 1. Undo command dispatch
- **입력**: command palette에서 index 7 행 클릭
- **처리**: `state.engine.undo()` 호출. `show_command_palette = false` 설정. undo 스택이 비어있으면 no-op.
- **출력/사이드 이펙트**: 문서 상태 복원, dirty = true, 팔레트 닫힘, `request_paint()`
- **순서/우선순위**: 다른 command row와 동일

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.command.undo` | `Button` | `"Undo"` | `show_command_palette == true` |

## 의존
- 선행 implement: `command-palette-header-toggle-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `dispatch_command_palette`, `engine.undo` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
