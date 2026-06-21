# Spec: control-y-redo-shortcut-control

## 한 줄 정의
사용자가 Story에서 Control Y Redo Shortcut Control을/를 단축키로 수행한다.

## 진입점
- 단축키: (해당 단축키)

## 사용자 흐름
1. From the user's perspective, this keyboard shortcuts control is independent and must not be merged with adjacent controls. When the user activates it by presses Control+Y, the redo snapshot restores through the alternate redo shortcut.

## 성공 조건 (Acceptance Criteria)
- [ ] Use the shortcut with no overlay focus conflict; the intended global action runs once.
- [ ] Use the shortcut while manuscript has unsaved changes; dirty/save/undo behavior remains correct.
- [ ] Use the shortcut repeatedly; toggles and history operations remain deterministic.
- [ ] Use the shortcut while text input overlay is focused; shortcut precedence follows the explicit focus contract.

## 실패 / 취소 흐름
- 모달/다이얼로그가 활성 중이면 단축키가 무시된다.
- 입력 필드에 포커스가 있으면 단축키가 입력으로 처리된다.

## 경계 / 예외
- Use the shortcut while manuscript has unsaved changes; dirty/save/undo behavior remains correct.
- Use the shortcut repeatedly; toggles and history operations remain deterministic.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
