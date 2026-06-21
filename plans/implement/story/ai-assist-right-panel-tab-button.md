# Implement: ai-assist-right-panel-tab-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- AI 탭 버튼 클릭 시 AI Assist 콘텐츠가 우측 패널에 표시되고, AI 탭이 활성(highlight) 상태가 된다.
- design: RIGHT_PANEL_TABS의 마지막 요소("AI", StoryTab::AiAssist). 활성 시 `theme.primary` 컬러.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/commands.rs::RIGHT_PANEL_TABS` | AI 탭 정의 포함 확인 | `grep -n 'AiAssist' commands.rs` |
| `apps/story/src-tauri/src/ui/commands.rs::hit_test_tab` | AI 탭 hit-test 포함 확인 | `fn hit_test_tab` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` | tab bar 렌더에서 AI 탭 활성 색상 적용 | `grep -n 'is_active' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | tab 클릭 시 `select_tab(StoryTab::AiAssist)` 호출 | `grep -n 'select_tab' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::tab_debug_id` | `StoryTab::AiAssist → "story.tab.ai_assist"` 매핑 | `fn tab_debug_id` |
| `apps/story/src-tauri/src/ui/state.rs::select_tab` | `active_tab` 갱신 | `fn select_tab` |

## 필요한 변경 (의도 단위)

### 1. Tab bar 페인트에서 AI 탭 렌더
- **입력**: `!focus_mode`, `RIGHT_PANEL_TABS` 배열 순회
- **처리**: 9번째 탭("AI")이 `active_tab == AiAssist`일 때 `theme.primary`로 텍스트 컬러 변경. 그렇지 않으면 `theme.on_surface`.
- **출력/사이드 이펙트**: 시각적 하이라이트만.
- **순서/우선순위**: 다른 탭과 동일 루프 내 처리.

### 2. AI 탭 클릭 hit-test
- **입력**: pointer down, y ∈ [48.0, 68.0], x가 9번째 탭 rect 내
- **처리**: `hit_test_tab`이 `Some(StoryTab::AiAssist)` 반환 → `on_pointer_event`에서 `select_tab(AiAssist)` 호출 → `active_tab = AiAssist`.
- **출력/사이드 이펙트**: `active_tab` 변경, repaint 요청.
- **순서/우선순위**: overlay hit-test 이후, chapter tree hit-test 이전.

### 3. 자동화 노드 emit
- **입력**: `!focus_mode`
- **처리**: `story.tab.ai_assist` Tab 노드를 tab bar 위치에 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 다른 탭 노드와 동일 루프.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.tab.ai_assist` | Tab | `"AI"` | `!focus_mode` |

## 의존
- 선행 implement: 없음 (기존 코드에 이미 구현됨, 문서화 목적).
- 영향 받는 implement: `automatic-right-panel-content-render-behavior` (탭 선택이 콘텐츠 렌더를 트리거).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'AiAssist' commands.rs mod.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
