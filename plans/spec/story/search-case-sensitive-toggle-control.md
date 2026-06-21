# Spec: search-case-sensitive-toggle-control

## 한 줄 정의
사용자가 Story에서 Search Case Sensitive Toggle Control을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. 사용자가 Search Case Sensitive Toggle Control 컨트롤을 확인한다.
2. 사용자가 해당 컨트롤을 활성화한다.
3. 화면에 결과가 즉시 반영된다.

## 성공 조건 (Acceptance Criteria)
- [ ] Toggle from off to on; lowercase query no longer matches uppercase-only text unless exact case matches.
- [ ] Toggle from on to off; matches become case-insensitive again.
- [ ] Toggle with an empty query; state changes but no results appear.
- [ ] Close and reopen search; the case sensitivity state follows the chosen persistence contract.

## 실패 / 취소 흐름
- Toggle with an empty query; state changes but no results appear.
- Close and reopen search; the case sensitivity state follows the chosen persistence contract.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
