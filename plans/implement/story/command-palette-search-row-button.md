# Implement: command-palette-search-row-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/command-palette-search-row-button.md`): Search (Ctrl+F) command row 클릭 시 Search 오버레이 열리고 팔레트 닫힘.
- design(`plans/design/story/command-palette-search-row-button.md`): Command Palette overlay의 Search row.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::dispatch_command_palette` (index 14 분기) | `toggle_search` 호출 | `toggle_search` 검색 |
| `apps/story/src-tauri/src/ui/commands.rs::command_labels` (index 14) | "Search (Ctrl+F)" 라벨 | `command_labels` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.command.search` 노드 | `story.command.search` 검색 |

## 필요한 변경 (의도 단위)
### 1. Search command dispatch
- **입력**: command palette에서 index 14 행 클릭
- **처리**: `state.toggle_search()` 호출. `show_command_palette = false` 설정. `toggle_search`는 `show_search`를 토글하고 `input_focus`를 Search로 설정.
- **출력/사이드 이펙트**: 검색 오버레이 열림, 팔레트 닫힘, `request_paint()`
- **순서/우선순위**: 다른 command row와 동일

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.command.search` | `Button` | `"Search (Ctrl+F)"` | `show_command_palette == true` |

## 의존
- 선행 implement: `command-palette-header-toggle-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `dispatch_command_palette`, `toggle_search` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
