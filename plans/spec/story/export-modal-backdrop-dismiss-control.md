# Spec: export-modal-backdrop-dismiss-control

## 한 줄 정의
사용자가 Story에서 Export Modal Backdrop Dismiss Control을/를 다이얼로그에서 수행한다.

## 진입점
- 다이얼로그: 해당 다이얼로그 열기

## 사용자 흐름
1. From the user's perspective, this Export Story modal control is independent and must not be merged with adjacent controls. When the user activates it by clicks outside the export modal, the export modal closes immediately and no export format is selected.

## 성공 조건 (Acceptance Criteria)
- [ ] Open export and click outside the modal; show_export becomes false.
- [ ] Click inside a format row; the row action runs instead of backdrop dismissal.
- [ ] Click outside after changing nothing; story content and dirty state remain unchanged.
- [ ] Press Escape while export is open; the same close behavior occurs.

## 실패 / 취소 흐름
- Open export and click outside the modal; show_export becomes false.
- Click outside after changing nothing; story content and dirty state remain unchanged.
- Press Escape while export is open; the same close behavior occurs.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
