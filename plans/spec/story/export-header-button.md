# Spec: export-header-button

## 한 줄 정의
사용자가 Story에서 Export Header Button을/를 조작하여 수행한다.

## 진입점
- 해당 컨트롤 활성화

## 사용자 흐름
1. From the user's perspective, this header action bar control is independent and must not be merged with adjacent controls. When the user activates it by clicks the Export button, the Export Story modal opens, command palette closes, and available export format rows become the next actionable choices.

## 성공 조건 (Acceptance Criteria)
- [ ] Click Export while no overlay is open; the export modal appears centered over the editor.
- [ ] Click Export while command palette is open; command palette closes and export modal opens.
- [ ] Click Export repeatedly; only one export modal is visible.
- [ ] Click Export in focus mode; modal opens over the focused layout and focus mode remains active.

## 실패 / 취소 흐름
- Click Export while command palette is open; command palette closes and export modal opens.

## 경계 / 예외
- Click Export repeatedly; only one export modal is visible.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
