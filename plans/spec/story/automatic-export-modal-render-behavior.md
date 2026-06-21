# Spec: automatic-export-modal-render-behavior

## 한 줄 정의
Story에서 Export Modal Render Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. When show_export is true, the Export Story modal automatically appears with project name, total word count, and export format rows.

## 성공 조건 (Acceptance Criteria)
- [ ] Open Export; modal appears centered with current project name.
- [ ] Edit text and open Export; word count in modal reflects the latest total.
- [ ] Close Export; modal disappears and underlying story remains unchanged.
- [ ] Resize the window; modal remains centered or clamps to a visible area.

## 실패 / 취소 흐름
- Close Export; modal disappears and underlying story remains unchanged.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
