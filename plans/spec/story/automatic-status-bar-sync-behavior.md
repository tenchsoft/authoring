# Spec: automatic-status-bar-sync-behavior

## 한 줄 정의
Story에서 Status Bar Sync Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. The status bar automatically mirrors saved time, selected chapter number, chapter word count, total word count, and focus mode state.

## 성공 조건 (Acceptance Criteria)
- [ ] Save the story; saved_at text changes to the saved timestamp.
- [ ] Select another chapter; chapter number and chapter word count update.
- [ ] Toggle focus mode; focus mode text appears or disappears.
- [ ] Edit manuscript text; word counts update while saved_at remains unchanged.

## 실패 / 취소 흐름
- 모달/다이얼로그가 열려 있으면 자동 동작이 억제된다.
- 문서가 유휴 상태면 동작이 발동하지 않는다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
