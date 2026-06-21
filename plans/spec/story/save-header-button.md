# Spec: save-header-button

## 한 줄 정의
사용자가 Story에서 Save Header Button을/를 조작하여 수행한다.

## 진입점
- 해당 컨트롤 활성화

## 사용자 흐름
1. From the user's perspective, this header action bar control is independent and must not be merged with adjacent controls. When the user activates it by clicks the Save button, the current story is persisted locally, the dirty marker disappears, and the status bar saved time updates immediately.

## 성공 조건 (Acceptance Criteria)
- [ ] Save a dirty story with an existing path; data writes to that path and the title loses the asterisk.
- [ ] Save a story without a path; a Save As dialog appears and success stores the chosen path.
- [ ] Cancel the Save As dialog; dirty state remains and no misleading saved timestamp appears.
- [ ] Save fails due to filesystem errors; an actionable error appears and dirty state remains true.

## 실패 / 취소 흐름
- Cancel the Save As dialog; dirty state remains and no misleading saved timestamp appears.
- Save fails due to filesystem errors; an actionable error appears and dirty state remains true.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
