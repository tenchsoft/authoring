# Spec: focus-header-toggle-button

## 한 줄 정의
사용자가 Story에서 Focus Header Toggle Button을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this header action bar control is independent and must not be merged with adjacent controls. When the user activates it by clicks the Focus button, focus mode toggles immediately: side panels hide when enabled, return when disabled, and the button active state changes.

## 성공 조건 (Acceptance Criteria)
- [ ] Click Focus while off; left chapter tree and right auxiliary panel disappear and editor expands.
- [ ] Click Focus while on; side panels return and current chapter selection is preserved.
- [ ] Toggle focus while command palette or search is open; overlay positioning remains correct.
- [ ] Toggle focus on a narrow width; center editor width stays non-negative and readable.

## 실패 / 취소 흐름
- 컨트롤이 비활성화 상태면 클릭해도 반응 없다.
- 다른 모달이 활성 중이면 입력이 무시된다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
