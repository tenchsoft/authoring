# Spec: automatic-word-count-badge-behavior

## 한 줄 정의
Story에서 Word Count Badge Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. The header word count badge and status bar automatically recalculate when chapter content changes.

## 성공 조건 (Acceptance Criteria)
- [ ] Type a word; chapter and total word counts increase on repaint.
- [ ] Backspace a word; counts decrease on repaint.
- [ ] Switch chapters; chapter count changes to the selected chapter while total remains project-wide.
- [ ] Open a project; all counts reflect the loaded document.

## 실패 / 취소 흐름
- 모달/다이얼로그가 열려 있으면 자동 동작이 억제된다.
- 문서가 유휴 상태면 동작이 발동하지 않는다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
