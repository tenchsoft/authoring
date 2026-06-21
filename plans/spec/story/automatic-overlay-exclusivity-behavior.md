# Spec: automatic-overlay-exclusivity-behavior

## 한 줄 정의
Story에서 Overlay Exclusivity Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. Export modal, command palette, and search overlay follow a predictable exclusivity and close behavior without extra user input.

## 성공 조건 (Acceptance Criteria)
- [ ] Open Export, then open command palette; export closes automatically.
- [ ] Press Escape with any overlay open; overlays close.
- [ ] Click outside export or palette; the visible overlay closes.
- [ ] Open search while another overlay is visible; focus/exclusivity follows one explicit rule.

## 실패 / 취소 흐름
- Open Export, then open command palette; export closes automatically.
- Press Escape with any overlay open; overlays close.
- Click outside export or palette; the visible overlay closes.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
