# Spec: automatic-dirty-title-indicator-behavior

## 한 줄 정의
Story에서 Dirty Title Indicator Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. The header title automatically shows an asterisk after unsaved edits and removes it after a successful save.

## 성공 조건 (Acceptance Criteria)
- [ ] Type manuscript text; the project title gains an asterisk.
- [ ] Save successfully; the asterisk disappears.
- [ ] Open a project; dirty state is clear and no asterisk appears.
- [ ] Failed save; asterisk remains because changes are still unsaved.

## 실패 / 취소 흐름
- Failed save; asterisk remains because changes are still unsaved.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
