# Spec: search-query-input-control

## 한 줄 정의
사용자가 Story에서 Search Query Input Control을/를 입력하여 수행한다.

## 진입점
- 입력: 해당 필드에 포커스 후 타이핑

## 사용자 흐름
1. From the user's perspective, this search overlay control is independent and must not be merged with adjacent controls. When the user activates it by types while the Search overlay is active, the query text updates in the search bar, matching results are found in the story, and current match navigation state updates.

## 성공 조건 (Acceptance Criteria)
- [ ] From the user's perspective, this search overlay control is independent and must not be merged with adjacent controls. When the user activates it by types while the Search overlay is active, the query text updates in the search bar, matching results are found in the story, and current match navigation state updates.
- [ ] Type a word in the search bar; the placeholder disappears and query appears.
- [ ] Clear the query; placeholder returns and all search highlights clear.
- [ ] Search with case sensitivity off; matching ignores letter case.
- [ ] Search with no matches; the UI reports zero results without moving the manuscript cursor unexpectedly.

## 실패 / 취소 흐름
- 필드가 비활성화되면 입력이 무시된다.
- 다른 모달이 활성 중이면 입력이 해당 모달로 라우팅된다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
