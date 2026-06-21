# Spec: automatic-right-panel-content-render-behavior

## 한 줄 정의
Story에서 Right Panel Content Render Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. The right panel automatically renders content for the active tab from StoryEngine data.

## 성공 조건 (Acceptance Criteria)
- [ ] Switch to Characters; character rows are drawn from doc.characters.
- [ ] Switch to Stats; statistics rows are computed from StoryEngine.statistics.
- [ ] Update document data; active tab rows refresh on repaint.
- [ ] Enter focus mode; right panel content is hidden without clearing active_tab.

## 실패 / 취소 흐름
- 모달/다이얼로그가 열려 있으면 자동 동작이 억제된다.
- 문서가 유휴 상태면 동작이 발동하지 않는다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
