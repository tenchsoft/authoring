# Spec: command-palette-header-toggle-button

## 한 줄 정의
사용자가 Story에서 Command Palette Header Toggle Button을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this header action bar control is independent and must not be merged with adjacent controls. When the user activates it by clicks the Cmd button, the command palette toggles open or closed immediately, Export modal closes when the palette opens, and the Cmd button active state changes.

## 성공 조건 (Acceptance Criteria)
- [ ] Click Cmd while closed; command palette opens with all command rows visible.
- [ ] Click Cmd while open; command palette closes.
- [ ] Click Cmd while export modal is open; export closes and command palette opens.
- [ ] Click Cmd in focus mode; palette still opens above the editor without restoring side panels.

## 실패 / 취소 흐름
- Click Cmd while closed; command palette opens with all command rows visible.
- Click Cmd while open; command palette closes.
- Click Cmd while export modal is open; export closes and command palette opens.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
