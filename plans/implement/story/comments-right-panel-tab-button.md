# Implement: comments-right-panel-tab-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/comments-right-panel-tab-button.md`): Notes 탭 클릭 시 코멘트 행 표시, Notes 탭 하이라이트.
- design(`plans/design/story/comments-right-panel-tab-button.md`): 우측 패널 탭 바의 Notes 탭 버튼.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/commands.rs::RIGHT_PANEL_TABS` | Notes/Comments 탭 정의 | `RIGHT_PANEL_TABS` 검색 |
| `apps/story/src-tauri/src/ui/commands.rs::hit_test_tab` | 탭 hit-test | `fn hit_test_tab` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::tab_debug_id` | `story.tab.comments` 매핑 | `tab_debug_id` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | 탭 노드 emit | `story.tab.comments` 검색 |

## 필요한 변경 (의도 단위)
### 1. Notes 탭 클릭 처리
- **입력**: 포인터 다운 이벤트, 우측 패널 탭 바 영역 내 Notes 탭 rect
- **처리**: `hit_test_tab`이 `StoryTab::Comments` 반환. `state.select_tab(StoryTab::Comments)` 호출.
- **출력/사이드 이펙트**: 우측 패널 Comments 콘텐츠 표시, 탭 하이라이트, `request_paint()`
- **순서/우선순위**: 다른 탭과 동일한 처리

### 2. Notes 탭 활성 상태 렌더링
- **입력**: `paint()` 호출 시 `active_tab == StoryTab::Comments`
- **처리**: 활성 탭 텍스트를 `theme.primary` 색상으로 렌더링
- **출력/사이드 이펙트**: 탭 하이라이트 표시

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.tab.comments` | `Tab` | `"Notes"` | `!focus_mode` |

## 의존
- 선행 implement: 없음 (기본 탭 버튼)

## 작업 절차
1. spec/design 읽기
2. grep으로 `RIGHT_PANEL_TABS`, `hit_test_tab`, `tab_debug_id` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
