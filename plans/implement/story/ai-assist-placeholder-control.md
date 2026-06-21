# Implement: ai-assist-placeholder-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- AI Assist 탭이 활성일 때 placeholder 영역을 클릭하면 AI writing action 또는 setup 메시지를 트리거한다.
- design: AI Assist 패널 내 단일 placeholder 버튼. 클릭 시 상태 변화 없으면 토스트/인라인 메시지로 안내.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` | `StoryTab::AiAssist` 분기에서 placeholder 클릭 가능 영역 렌더 | `grep -n 'AiAssist' panels.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | right panel row hit-test에서 `StoryTab::AiAssist` 분기가 placeholder 클릭을 처리 | `grep -n 'AiAssist' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` | `StoryTab::AiAssist` 분기가 placeholder rect hit-test 반환 | `fn hit_test_right_panel_row` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` | `StoryTab::AiAssist` 분기에서 `story.ai.placeholder` 노드 emit | `grep -n 'story.ai.placeholder'` |
| `apps/story/src-tauri/src/ui/state.rs` | (선택) AI 관련 상태 필드 추가 시 | `pub fn select_tab` |

## 필요한 변경 (의도 단위)

### 1. AI Assist placeholder 페인트
- **입력**: `state.active_tab == StoryTab::AiAssist`
- **처리**: `paint_tab_content`의 `AiAssist` 분기에서 placeholder 텍스트("AI features will be added in a future phase.")를 클릭 가능한 카드 형태로 렌더. 배경 `theme.background`, 라운드 rect.
- **출력/사이드 이펙트**: 시각적 변화만, state 변경 없음.
- **순서/우선순위**: 기존 `AiAssist` 분기 내부 교체.

### 2. Placeholder 클릭 hit-test
- **입력**: pointer down 이벤트, `active_tab == AiAssist`, `!focus_mode`
- **처리**: `hit_test_right_panel_row`의 `StoryTab::AiAssist` 분기가 이미 `panel_row_rect`로 hit-test 수행. 클릭 시 `on_pointer_event`에서 `"ai"` label 반환 → 현재는 `saved_at` 업데이트만. 실제 AI 액션(엔진 연동)은 후속 phase에서 구현.
- **출력/사이드 이펙트**: 현재는 `saved_at = "selected ai"`. 향후 AI 액션 트리거로 교체.
- **순서/우선순위**: 다른 right panel row hit-test와 동일 순서.

### 3. 자동화 노드 emit
- **입력**: `push_panel_row_nodes` 호출, `active_tab == AiAssist`
- **처리**: `story.ai.placeholder` 버튼 노드를 `panel_row_rect` 위치에 emit. role `"button"`, value `"AI assist placeholder"`.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 다른 패널 row 노드와 동일.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.ai.placeholder` | Button | `"AI assist placeholder"` | `active_tab == AiAssist && !focus_mode` |

## 의존
- 선행 implement: 없음 (기존 코드에 이미 구현됨, 문서화 목적).
- 영향 받는 implement: `automatic-right-panel-content-render-behavior` (동일 영역).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'AiAssist' panels.rs mod.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
