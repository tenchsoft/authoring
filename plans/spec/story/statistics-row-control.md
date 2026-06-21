# Spec: statistics-row-control

## 한 줄 정의
사용자가 Story에서 Statistics Row Control을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this Statistics panel control is independent and must not be merged with adjacent controls. When the user activates it by clicks a statistics row, the row expands or opens the relevant source view for that metric without changing manuscript content.

## 성공 조건 (Acceptance Criteria)
- [ ] Click Total Words; the app shows word-count details by chapter.
- [ ] Click Character Entries; the app navigates to or filters the Characters panel.
- [ ] Click a non-navigable metric; the row remains read-only and communicates that no detail exists.
- [ ] Click stats after editing text; metric values reflect the latest document state.

## 실패 / 취소 흐름
- 컨트롤이 비활성화 상태면 클릭해도 반응 없다.
- 다른 모달이 활성 중이면 입력이 무시된다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
