# Spec: automatic-statistics-refresh-behavior

## 한 줄 정의
Story에서 Statistics Refresh Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. Statistics automatically recalculate from StoryEngine data whenever document content or metadata changes.

## 성공 조건 (Acceptance Criteria)
- [ ] Edit manuscript text; total words, characters, sentences, and reading time refresh.
- [ ] Add a chapter; chapter count refreshes.
- [ ] Add character/world/glossary data; corresponding counts refresh.
- [ ] Undo or redo a document mutation; stats revert or reapply with the document state.

## 실패 / 취소 흐름
- 모달/다이얼로그가 열려 있으면 자동 동작이 억제된다.
- 문서가 유휴 상태면 동작이 발동하지 않는다.

## 경계 / 예외
- Undo or redo a document mutation; stats revert or reapply with the document state.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
