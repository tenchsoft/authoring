# Spec: pdf-export-format-button

## 한 줄 정의
사용자가 Story에서 PDF Export Format Button을/를 조작하여 수행한다.

## 진입점
- 해당 컨트롤 활성화

## 사용자 흐름
1. From the user's perspective, this Export Story modal control is independent and must not be merged with adjacent controls. When the user activates it by clicks the PDF (.pdf) row, a PDF export starts or is configured, the export request uses .pdf format, and the modal closes or shows progress after confirmation.

## 성공 조건 (Acceptance Criteria)
- [ ] Click PDF (.pdf); the export path dialog or export progress for that exact format begins.
- [ ] Cancel the export path dialog; the modal remains open or closes according to the explicit cancel rule without creating a file.
- [ ] Export succeeds; saved/export status appears and the output file matches the selected format.
- [ ] Export fails; the modal stays usable and an actionable error is shown.

## 실패 / 취소 흐름
- Cancel the export path dialog; the modal remains open or closes according to the explicit cancel rule without creating a file.
- Export fails; the modal stays usable and an actionable error is shown.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
