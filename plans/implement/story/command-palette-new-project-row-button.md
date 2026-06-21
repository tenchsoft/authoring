# Implement: command-palette-new-project-row-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/command-palette-new-project-row-button.md`): New project command row 클릭 시 팔레트 닫히고 New project 플로우 실행.
- design(`plans/design/story/command-palette-new-project-row-button.md`): Command Palette overlay의 New project row.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::dispatch_command_palette` (index 0 분기) | `StoryState::default()`로 리셋 | `StoryState::default()` 검색 |
| `apps/story/src-tauri/src/ui/commands.rs::command_labels` (index 0) | "New project" 라벨 | `command_labels` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.command.new_project` 노드 | `story.command.new_project` 검색 |

## 필요한 변경 (의도 단위)
### 1. New project command dispatch
- **입력**: command palette에서 index 0 행 클릭
- **처리**: `self.state = StoryState::default()` 호출로 전체 상태 리셋. `show_command_palette = false` 설정.
- **출력/사이드 이펙트**: 새 프로젝트 생성 (Chapter 1 포함), dirty 상태 초기화, 팔레트 닫힘, `request_paint()`
- **순서/우선순위**: 다른 command row와 동일

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.command.new_project` | `Button` | `"New project"` | `show_command_palette == true` |

## 의존
- 선행 implement: `command-palette-header-toggle-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `dispatch_command_palette` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
