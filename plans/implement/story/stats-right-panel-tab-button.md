# Implement: stats-right-panel-tab-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click the "Stats" tab in the right-panel tab bar switches to the Statistics tab, showing statistics rows and highlighting the tab label.
- design: tab bar with 9 tabs; "Stats" is the 5th tab (index 4).

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/commands.rs::RIGHT_PANEL_TABS` | Tab definition array with ("Stats", StoryTab::Stats) at index 4 | `grep -n 'RIGHT_PANEL_TABS'` |
| `apps/story/src-tauri/src/ui/commands.rs::hit_test_tab` | Hit-tests tab bar positions | `fn hit_test_tab` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` (tab selection) | Calls `self.state.select_tab(tab)` on hit | `grep -n 'hit_test_tab'` within `on_pointer_event` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` (tab bar rendering) | Paints "Stats" label with primary color when active | `grep -n 'RIGHT_PANEL_TABS'` within `paint` |
| `apps/story/src-tauri/src/ui/mod.rs::tab_debug_id` | Maps `StoryTab::Stats` to `"story.tab.stats"` | `fn tab_debug_id` |
| `apps/story/src-tauri/src/ui/state.rs::StoryState::select_tab` | Sets `active_tab` field | `fn select_tab` |

## 필요한 변경 (의도 단위)

### 1. Tab hit-test and dispatch
- **입력**: PointerEvent::Down when not in focus mode, point inside the "Stats" tab rect (left_w + center_w + 8.0 + 4 * 32.0, 48.0, 30x20)
- **처리**: `hit_test_tab` returns `Some(StoryTab::Stats)`. `on_pointer_event` calls `self.state.select_tab(StoryTab::Stats)`, which sets `active_tab = StoryTab::Stats`.
- **출력/사이드 이펙트**: Tab content area repaints with statistics rows, tab label highlighted with `theme.primary` color. `request_paint()` triggered.
- **순서/우선순위**: Tab hit-test runs after chapter tree, before panel row hit-test.

### 2. Tab bar rendering
- **입력**: `paint` iterates `RIGHT_PANEL_TABS` when not in focus mode
- **처리**: For the "Stats" entry, if `active_tab == StoryTab::Stats`, paint label with `theme.primary` color; otherwise `theme.on_surface`.
- **출력/사이드 이펙트**: Visual highlight on active tab.

### 3. Automation node emission
- **입력**: `story_automation_nodes` iterates `RIGHT_PANEL_TABS` when not in focus mode
- **처리**: Emit node with `debug_id = "story.tab.stats"`, `role = "tab"`, `label = "Stats"`, bounds at tab position.
- **출력/사이드 이펙트**: Only visible when not in focus mode.
- **순서/우선순위**: After chapter nodes, before panel row nodes.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.tab.stats` | Tab | `"Stats"` | `focus_mode == false` |

## 의존
- 선행 implement: none
- 영향 받는 implement: `statistics-row-control` (rows depend on tab being selected)

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `RIGHT_PANEL_TABS` 및 `hit_test_tab` 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
