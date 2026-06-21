# Spec: world-entry-row-control

## 한 줄 정의
사용자가 Story에서 World Entry Row Control을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this World Building panel control is independent and must not be merged with adjacent controls. When the user activates it by clicks a world entry row control, the world entry detail editor opens or selects that world entry row.

## 성공 조건 (Acceptance Criteria)
- [ ] Click one row; only that row becomes selected and the correct detail data appears.
- [ ] Click another row; previous selection clears and detail switches to the new row.
- [ ] Click a stale row after data changes; no panic occurs and the UI refreshes from current data.
- [ ] Click a row while focus mode is active; hidden panel rows do not receive input.

## 실패 / 취소 흐름
- 컨트롤이 비활성화 상태면 클릭해도 반응 없다.
- 다른 모달이 활성 중이면 입력이 무시된다.

## 경계 / 예외
- Click a stale row after data changes; no panic occurs and the UI refreshes from current data.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
