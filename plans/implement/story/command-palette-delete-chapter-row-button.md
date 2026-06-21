# Implement: command-palette-delete-chapter-row-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/command-palette-delete-chapter-row-button.md`): Delete chapter command row 클릭 시 현재 챕터 삭제, 선택 조정, 팔레트 닫힘.
- design(`plans/design/story/command-palette-delete-chapter-row-button.md`): Command Palette overlay의 Delete chapter row.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::dispatch_command_palette` (index 6 분기) | `delete_current_chapter` 호출 | `delete_current_chapter` 검색 |
| `apps/story/src-tauri/src/ui/commands.rs::command_labels` (index 6) | "Delete chapter" 라벨 | `command_labels` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.command.delete_chapter` 노드 | `story.command.delete_chapter` 검색 |

## 필요한 변경 (의도 단위)
### 1. Delete chapter command dispatch
- **입력**: command palette에서 index 6 행 클릭
- **처리**: `state.delete_current_chapter()` 호출. `show_command_palette = false` 설정. 챕터가 1개뿐이면 삭제하지 않거나 안내 메시지 표시.
- **출력/사이드 이펙트**: 현재 챕터 삭제, `selected_chapter_idx` 조정, 팔레트 닫힘, `request_paint()`
- **순서/우선순위**: 다른 command row 분기와 동일

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.command.delete_chapter` | `Button` | `"Delete chapter"` | `show_command_palette == true` |

## 의존
- 선행 implement: `command-palette-header-toggle-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `dispatch_command_palette`, `delete_current_chapter` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
