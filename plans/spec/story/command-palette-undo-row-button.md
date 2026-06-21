# Spec: command-palette-undo-row-button

## 한 줄 정의
사용자가 Story에서 Command Palette Undo Row Button을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this Command Palette overlay control is independent and must not be merged with adjacent controls. When the user activates it by clicks the Undo command row, the previous StoryEngine undo snapshot restores and the palette closes.

## 성공 조건 (Acceptance Criteria)
- [ ] Open the command palette and click Undo; exactly that command runs once.
- [ ] Click Undo with missing prerequisites; the app shows a clear message or disabled row state.
- [ ] Click outside the palette; the palette closes and Undo does not run.
- [ ] Use keyboard navigation to highlight Undo and press Enter; the same action runs.

## 실패 / 취소 흐름
- Click Undo with missing prerequisites; the app shows a clear message or disabled row state.
- Click outside the palette; the palette closes and Undo does not run.

## 경계 / 예외
- Open the command palette and click Undo; exactly that command runs once.
- Click Undo with missing prerequisites; the app shows a clear message or disabled row state.
- Click outside the palette; the palette closes and Undo does not run.
- Use keyboard navigation to highlight Undo and press Enter; the same action runs.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
