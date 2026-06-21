# Spec: command-palette-delete-chapter-row-button

## 한 줄 정의
사용자가 Story에서 Command Palette Delete Chapter Row Button을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this Command Palette overlay control is independent and must not be merged with adjacent controls. When the user activates it by clicks the Delete chapter command row, the current chapter is deleted after confirmation, selection adjusts, and the command palette closes.

## 성공 조건 (Acceptance Criteria)
- [ ] Open the command palette and click Delete chapter; exactly that command runs once.
- [ ] Click Delete chapter with missing prerequisites; the app shows a clear message or disabled row state.
- [ ] Click outside the palette; the palette closes and Delete chapter does not run.
- [ ] Use keyboard navigation to highlight Delete chapter and press Enter; the same action runs.

## 실패 / 취소 흐름
- Click Delete chapter with missing prerequisites; the app shows a clear message or disabled row state.
- Click outside the palette; the palette closes and Delete chapter does not run.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
