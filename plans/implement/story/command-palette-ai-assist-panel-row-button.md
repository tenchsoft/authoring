# Implement: command-palette-ai-assist-panel-row-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 커맨드 팔레트에서 "AI assist panel" 행 클릭 시 AI Assist 우측 패널이 열리고, 팔레트가 닫힌다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/commands.rs::command_labels` | 인덱스 15 = "AI assist panel" | `fn command_labels` |
| `apps/story/src-tauri/src/ui/mod.rs::dispatch_command_palette` | 인덱스 15 → `select_tab(StoryTab::AiAssist)` | `fn dispatch_command_palette` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 커맨드 행 클릭 → `dispatch_command_palette` 호출 | `grep -n 'dispatch_command_palette' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::select_tab` | `active_tab` 갱신 | `fn select_tab` |
| `apps/story/src-tauri/src/ui/mod.rs::command_debug_ids` | 인덱스 15 = `"story.command.ai_assist_panel"` | `fn command_debug_ids` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.command.ai_assist_panel` 노드 emit | `grep -n 'story.command.ai_assist_panel'` |

## 필요한 변경 (의도 단위)

### 1. AI assist panel 행 렌더
- **입력**: `show_command_palette == true`
- **처리**: `paint_command_palette`에서 `command_labels()` 순회. 인덱스 15번째(마지막) 행에 "AI assist panel" 텍스트 렌더. 위치: `palette.y0 + 60.0 + 15 * 22.0`.
- **출력/사이드 이펙트**: 시각적 행.
- **순서/우선순위**: 다른 커맨드 행과 함께, 마지막 행.

### 2. 행 클릭 → AI Assist 패널 열기
- **입력**: pointer down, `show_command_palette == true`, `hit_test_command_row`가 15 반환
- **처리**: `dispatch_command_palette(15)` → `state.select_tab(StoryTab::AiAssist)` 호출. `active_tab = AiAssist`. `show_command_palette = false` (index != 3이므로).
- **출력/사이드 이펙트**: AI Assist 패널 활성, 팔레트 닫힘, repaint.
- **순서/우선순위**: 팔레트 행 hit-test 성공 시.

### 3. 자동화 노드
- **입력**: `show_command_palette == true`
- **처리**: `story.command.ai_assist_panel` Button 노드를 15번째 행 위치에 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 다른 커맨드 노드와 함께, 마지막.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.command.ai_assist_panel` | Button | `"AI assist panel"` | `show_command_palette == true` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-command-palette-render-behavior` (동일 팔레트), `ai-assist-right-panel-tab-button` (AI 탭 활성).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'AiAssist\|ai_assist' mod.rs state.rs commands.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
