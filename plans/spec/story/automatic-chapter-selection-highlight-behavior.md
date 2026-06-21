# Spec: automatic-chapter-selection-highlight-behavior

## 한 줄 정의
Story에서 Chapter Selection Highlight Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. 사용자가 Story에서 문서를 편집하거나 뷰를 조작한다.
2. 조건이 충족되면 Automatic Chapter Selection Highlight Behavior이 자동으로 동작한다.
3. 화면에 해당 결과가 반영된다.

## 성공 조건 (Acceptance Criteria)
- [ ] Select a chapter row; its background changes to active color.
- [ ] Delete the selected chapter; selection moves to a valid remaining chapter.
- [ ] Add a chapter; the new chapter becomes highlighted.
- [ ] Enter focus mode; highlights disappear because the tree is hidden but selection state is retained.

## 실패 / 취소 흐름
- 모달/다이얼로그가 열려 있으면 자동 동작이 억제된다.
- 문서가 유휴 상태면 동작이 발동하지 않는다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
