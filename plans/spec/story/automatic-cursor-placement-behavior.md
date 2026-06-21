# Spec: automatic-cursor-placement-behavior

## 한 줄 정의
Story에서 Cursor Placement Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. The manuscript cursor automatically appears after the rendered chapter content and moves as lines are inserted or removed.

## 성공 조건 (Acceptance Criteria)
- [ ] Type text; cursor moves down or right according to rendered lines.
- [ ] Insert newlines; cursor y position advances by line height.
- [ ] Delete text; cursor moves backward safely.
- [ ] Content exceeds the visible editor height; cursor only renders when within the visible area or scrolling handles it.

## 실패 / 취소 흐름
- 모달/다이얼로그가 열려 있으면 자동 동작이 억제된다.
- 문서가 유휴 상태면 동작이 발동하지 않는다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
