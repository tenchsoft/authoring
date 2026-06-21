# Implement: characters-right-panel-tab-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Chars 탭 버튼 클릭 시 Characters 콘텐츠가 우측 패널에 표시되고, Chars 탭이 활성(highlight) 상태가 된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/commands.rs::RIGHT_PANEL_TABS` | 첫 번째 요소 ("Chars", StoryTab::Characters) | `grep -n 'Characters' commands.rs` |
| `apps/story/src-tauri/src/ui/commands.rs::hit_test_tab` | Chars 탭 hit-test | `fn hit_test_tab` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` | tab bar 렌더에서 Chars 탭 활성 색상 | `grep -n 'is_active' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | tab 클릭 시 `select_tab(StoryTab::Characters)` | `grep -n 'select_tab' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::tab_debug_id` | `StoryTab::Characters → "story.tab.characters"` 매핑 | `fn tab_debug_id` |
| `apps/story/src-tauri/src/ui/state.rs::select_tab` | `active_tab` 갱신 | `fn select_tab` |

## 필요한 변경 (의도 단위)

### 1. Tab bar 페인트에서 Chars 탭 렌더
- **입력**: `!focus_mode`, `RIGHT_PANEL_TABS` 배열 순회
- **처리**: 첫 번째 탭("Chars")이 `active_tab == Characters`일 때 `theme.primary`로 텍스트 컬러 변경. 그렇지 않으면 `theme.on_surface`.
- **출력/사이드 이펙트**: 시각적 하이라이트.
- **순서/우선순위**: tab bar 루프 내 첫 번째.

### 2. Chars 탭 클릭 hit-test
- **입력**: pointer down, y ∈ [48.0, 68.0], x가 첫 번째 탭 rect 내
- **처리**: `hit_test_tab`이 `Some(StoryTab::Characters)` 반환 → `on_pointer_event`에서 `select_tab(Characters)` 호출 → `active_tab = Characters`.
- **출력/사이드 이펙트**: `active_tab` 변경, repaint 요청.
- **순서/우선순위**: overlay hit-test 이후, chapter tree hit-test 이전.

### 3. 자동화 노드
- **입력**: `!focus_mode`
- **처리**: `story.tab.characters` Tab 노드를 tab bar 첫 번째 위치에 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: tab 노드 루프 내 첫 번째.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.tab.characters` | Tab | `"Chars"` | `!focus_mode` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-right-panel-content-render-behavior` (탭 선택이 콘텐츠 렌더를 트리거), `character-row-control` (Characters 탭 활성 시 캐릭터 행 표시).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'Characters\|select_tab' commands.rs mod.rs state.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
