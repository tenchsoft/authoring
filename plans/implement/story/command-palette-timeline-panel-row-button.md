# Implement: command-palette-timeline-panel-row-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/command-palette-timeline-panel-row-button.md`): Timeline panel command row 클릭 시 Timeline 우측 패널 열리고 팔레트 닫힘.
- design(`plans/design/story/command-palette-timeline-panel-row-button.md`): Command Palette overlay의 Timeline panel row.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::dispatch_command_palette` (index 11 분기) | `StoryTab::Timeline` 선택 | `StoryTab::Timeline` 검색 |
| `apps/story/src-tauri/src/ui/commands.rs::command_labels` (index 11) | "Timeline panel" 라벨 | `command_labels` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.command.timeline_panel` 노드 | `story.command.timeline_panel` 검색 |

## 필요한 변경 (의도 단위)
### 1. Timeline panel command dispatch
- **입력**: command palette에서 index 11 행 클릭
- **처리**: `state.select_tab(StoryTab::Timeline)` 호출. `show_command_palette = false` 설정.
- **출력/사이드 이펙트**: 우측 패널 Timeline 탭 활성화, 팔레트 닫힘, `request_paint()`
- **순서/우선순위**: 다른 command row와 동일

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.command.timeline_panel` | `Button` | `"Timeline panel"` | `show_command_palette == true` |

## 의존
- 선행 implement: `command-palette-header-toggle-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `dispatch_command_palette`, `StoryTab::Timeline` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
