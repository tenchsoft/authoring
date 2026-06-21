# Implement: glossary-right-panel-tab-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 우측 패널의 Gloss 탭 버튼을 클릭하면 용어집(Glossary) 탭이 활성화되고, 용어 행들이 표시되며 해당 탭이 하이라이트된다.
- design: right panel Gloss 탭 버튼 → 탭 전환.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 탭 바 히트 테스트에서 `StoryTab::Glossary` 반환 시 `state.select_tab(tab)` 호출 | `grep -n 'hit_test_tab'` 또는 `fn on_pointer_event` |
| `apps/story/src-tauri/src/ui/commands.rs::hit_test_tab` | 탭 바 히트 테스트 함수, `RIGHT_PANEL_TABS`의 index 5가 Glossary | `fn hit_test_tab` |
| `apps/story/src-tauri/src/ui/commands.rs::RIGHT_PANEL_TABS` | `("Gloss", StoryTab::Glossary)` 항목 (index 5) | `grep -n 'Gloss'` |
| `apps/story/src-tauri/src/ui/mod.rs::tab_debug_id` | `StoryTab::Glossary` → `"story.tab.glossary"` 매핑 | `fn tab_debug_id` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.tab.glossary` 노드 emit | `grep -n 'story.tab.glossary'` |
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` | `StoryTab::Glossary` 분기에서 용어집 행 렌더링 | `grep -n 'StoryTab::Glossary'` |

## 필요한 변경 (의도 단위)

### 1. 포인터 이벤트에서 Gloss 탭 클릭 처리
- **입력**: `PointerEvent::Down`, `!state.focus_mode`, 탭 바 영역 내 클릭, `hit_test_tab`이 `Some(StoryTab::Glossary)` 반환
- **처리**: `state.select_tab(StoryTab::Glossary)` 호출. 이미 구현된 분기가 있으므로 올바르게 동작하는지 확인만 한다.
- **출력/사이드 이펙트**: `active_tab = StoryTab::Glossary`, 패널 콘텐츠 repaint, `ctx.request_paint()`
- **순서/우선순위**: 챕터 트리 히트 테스트 이후, 탭 바 히트 테스트에서 처리.

### 2. 페인트에서 Glossary 탭 하이라이트
- **입력**: `state.active_tab == StoryTab::Glossary`
- **처리**: 탭 바 루프에서 `is_active == true`이면 `theme.primary` 색상으로 텍스트 렌더링.
- **출력/사이드 이펙트**: 탭 라벨 "Gloss"가 primary 색상으로 표시.

### 3. 자동화 노드 확인
- **입력**: `story_automation_nodes`에서 탭 바 루프
- **처리**: `story.tab.glossary` 노드가 `Rect::new(left_w + center_w + 8.0 + 5*32.0, panel_y, ...)` 위치에 `role = "tab"`, value = `"Gloss"`로 emit되는지 확인.
- **출력/사이드 이펙트**: 노드 emit.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.tab.glossary` | Tab | `"Gloss"` | `!state.focus_mode` |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design/background 읽기
2. `grep -n 'hit_test_tab' apps/story/src-tauri/src/ui/mod.rs`로 클릭 처리 위치 확정
3. `grep -n 'Gloss' apps/story/src-tauri/src/ui/commands.rs`로 탭 정의 확인
4. `grep -n 'story.tab.glossary' apps/story/src-tauri/src/ui/mod.rs`로 자동화 노드 확인
5. 의도대로 코드 변경 (필요 시)
6. `cargo check --workspace --locked` 통과 확인
