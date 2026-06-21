# Implement: comment-row-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/comment-row-control.md`): Comment row 클릭 시 해당 코멘트 선택, 상세 편집기 열림, resolve/unresolve 가능.
- design(`plans/design/story/comment-row-control.md`): Comments panel의 comment row 컨트롤.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::hit_test_right_panel_row` (Comments 분기) | 코멘트 행 hit-test | `StoryTab::Comments` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` (Comments 분기) | 코멘트 행 자동화 노드 | `story.comment` 검색 |
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` (Comments 분기) | 코멘트 행 렌더링 | `"Comments"` 검색 |

## 필요한 변경 (의도 단위)
### 1. Comment row 클릭 처리
- **입력**: 포인터 다운 이벤트, `active_tab == StoryTab::Comments`, 코멘트 행 rect 내부
- **처리**: `hit_test_right_panel_row`에서 코멘트 인덱스 매칭. 선택된 코멘트의 상세 정보 표시. 코멘트가 resolved 상태면 "Resolved", 아니면 "Open" 라벨.
- **출력/사이드 이펙트**: 코멘트 선택, 상세 편집기 열림, `request_paint()`
- **순서/우선순위**: focus mode에서는 패널이 숨겨지므로 클릭 무시

### 2. Comment row 렌더링
- **입력**: `paint_tab_content` 호출, `active_tab == StoryTab::Comments`
- **처리**: `doc.comments` 순회하며 각 코멘트의 resolved 상태와 텍스트를 패널 행으로 렌더링
- **출력/사이드 이펙트**: 코멘트 행 그리기
- **순서/우선순위**: 패널 타이틀 아래, 첫 번째 행부터 순서대로

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.comment.{idx}` | `Button` | 코멘트 텍스트 | `active_tab == Comments && !focus_mode` |

## 의존
- 선행 implement: `comments-right-panel-tab-button`

## 작업 절차
1. spec/design 읽기
2. grep으로 `StoryTab::Comments`, `hit_test_right_panel_row` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
