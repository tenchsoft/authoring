# Implement: command-palette-export-row-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/command-palette-export-row-button.md`): Export command row 클릭 시 팔레트 닫히고 Export 모달 열림.
- design(`plans/design/story/command-palette-export-row-button.md`): Command Palette overlay의 Export row.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::dispatch_command_palette` (index 3 분기) | `open_export` 호출 후 팔레트 닫지 않음 예외 처리 | `index != 3` 검색 |
| `apps/story/src-tauri/src/ui/commands.rs::command_labels` (index 3) | "Export" 라벨 | `command_labels` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.command.export` 노드 | `story.command.export` 검색 |

## 필요한 변경 (의도 단위)
### 1. Export command dispatch
- **입력**: command palette에서 index 3 행 클릭
- **처리**: `state.open_export()` 호출. export 모달이 열리므로 `show_command_palette = false`는 자동으로 `open_export()` 내에서 처리됨. 단 dispatch 끝의 `if index != 3` 조건으로 팔레트 닫기가 건너뛰어지지 않도록 확인.
- **출력/사이드 이펙트**: export 모달 열림, 팔레트 닫힘, `request_paint()`
- **순서/우선순위**: 다른 command row와 동일. export 모달은 팔레트 위에 바로 열림.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.command.export` | `Button` | `"Export"` | `show_command_palette == true` |

## 의존
- 선행 implement: `command-palette-header-toggle-button`, `export-header-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `dispatch_command_palette`, `open_export` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
