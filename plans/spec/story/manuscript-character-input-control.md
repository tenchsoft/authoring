# Spec: manuscript-character-input-control

## 한 줄 정의
사용자가 Story에서 Manuscript Character Input Control을/를 입력하여 수행한다.

## 진입점
- 입력: 해당 필드에 포커스 후 타이핑

## 사용자 흐름
1. From the user's perspective, this center manuscript editor control is independent and must not be merged with adjacent controls. When the user activates it by types a character key while the editor is active, the character appends to the selected chapter content, the dirty marker appears, word counts refresh, and the cursor moves after the new text.

## 성공 조건 (Acceptance Criteria)
- [ ] Type normal letters; they append exactly once and the editor repaints.
- [ ] Type while command palette, export modal, or search is focused; text goes to the active overlay or is ignored according to focus rules, not into the manuscript.
- [ ] Type multi-byte or IME text; committed text inserts as one logical input without corrupting content.
- [ ] Type when no chapter exists; the app creates a safe chapter or reports an actionable no-chapter state.

## 실패 / 취소 흐름
- 필드가 비활성화되면 입력이 무시된다.
- 다른 모달이 활성 중이면 입력이 해당 모달로 라우팅된다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
