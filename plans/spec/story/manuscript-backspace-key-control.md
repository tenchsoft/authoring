# Spec: manuscript-backspace-key-control

## 한 줄 정의
사용자가 Story에서 Manuscript Backspace Key Control을/를 단축키로 수행한다.

## 진입점
- 단축키: (해당 단축키)

## 사용자 흐름
1. From the user's perspective, this center manuscript editor control is independent and must not be merged with adjacent controls. When the user activates it by presses Backspace while the editor is active, the previous character in the selected chapter is removed, dirty state updates, and the cursor moves backward safely.

## 성공 조건 (Acceptance Criteria)
- [ ] Backspace with non-empty content; exactly one character is removed.
- [ ] Backspace at empty content; no panic occurs and content remains empty.
- [ ] Backspace after a newline; the line merge behavior is correct.
- [ ] Backspace while overlays are focused; the overlay consumes it or closes according to its explicit rule.

## 실패 / 취소 흐름
- Backspace with non-empty content; exactly one character is removed.
- Backspace at empty content; no panic occurs and content remains empty.
- Backspace while overlays are focused; the overlay consumes it or closes according to its explicit rule.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
