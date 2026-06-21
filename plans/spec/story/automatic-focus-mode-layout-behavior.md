# Spec: automatic-focus-mode-layout-behavior

## 한 줄 정의
Story에서 Focus Mode Layout Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. When focus mode changes, side panels automatically hide or return and the editor width recalculates from the available window width.

## 성공 조건 (Acceptance Criteria)
- [ ] Enable focus mode; left and right panel widths become zero and center editor expands.
- [ ] Disable focus mode; left panel returns to 220px and right panel returns to 300px.
- [ ] Resize the window while focused; center editor uses the current width.
- [ ] Open overlays while focused; overlay positions remain centered or otherwise valid.

## 실패 / 취소 흐름
- 모달/다이얼로그가 열려 있으면 자동 동작이 억제된다.
- 문서가 유휴 상태면 동작이 발동하지 않는다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
