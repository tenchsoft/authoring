# Spec: chapter-tree-row-control

## 한 줄 정의
사용자가 Story에서 Chapter Tree Row Control을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this left chapter tree control is independent and must not be merged with adjacent controls. When the user activates it by clicks a chapter row, that chapter becomes selected, the selected row highlight moves there, and the editor title/content switch to that chapter immediately.

## 성공 조건 (Acceptance Criteria)
- [ ] Click Chapter 1 while another chapter is selected; the editor loads Chapter 1 text.
- [ ] Click outside any chapter row; no selection changes.

## 실패 / 취소 흐름
- Click outside any chapter row; no selection changes.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
