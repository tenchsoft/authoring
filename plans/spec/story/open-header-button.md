# Spec: open-header-button

## 한 줄 정의
사용자가 Story에서 Open Header Button을/를 조작하여 수행한다.

## 진입점
- 해당 컨트롤 활성화

## 사용자 흐름
1. From the user's perspective, this header action bar control is independent and must not be merged with adjacent controls. When the user activates it by clicks the Open button, a native open dialog appears, and after confirmation the selected story project loads, selection moves to a valid chapter, dirty state clears, and the status bar records the open state.

## 성공 조건 (Acceptance Criteria)
- [ ] Confirm a valid story file; the title, chapters, right panel data, word counts, and editor content all refresh from the loaded document.
- [ ] Cancel the open dialog; no current story data changes.
- [ ] Open while the current story is dirty; the app prompts before discarding unsaved work.
- [ ] Open a corrupt or unsupported file; an actionable error is shown and the current story remains open.

## 실패 / 취소 흐름
- Cancel the open dialog; no current story data changes.
- Open a corrupt or unsupported file; an actionable error is shown and the current story remains open.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
