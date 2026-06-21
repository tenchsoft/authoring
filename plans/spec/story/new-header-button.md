# Spec: new-header-button

## 한 줄 정의
사용자가 Story에서 New Header Button을/를 조작하여 수행한다.

## 진입점
- 해당 컨트롤 활성화

## 사용자 흐름
1. From the user's perspective, this header action bar control is independent and must not be merged with adjacent controls. When the user activates it by clicks the New button, the current story resets to a new untitled project, Chapter 1 becomes selected, dirty state clears, and any active editor/sidebar state returns to its default baseline.

## 성공 조건 (Acceptance Criteria)
- [ ] Click New with unsaved text; the app asks for confirmation or preserves a recoverable undo/safety path before replacing the document.
- [ ] Confirm New; the chapter tree shows only Chapter 1 and the editor is empty.
- [ ] Cancel New; the existing story, selection, dirty flag, and overlays remain unchanged.
- [ ] Click New while export or command palette is open; the overlay closes only after the New decision is resolved.

## 실패 / 취소 흐름
- Confirm New; the chapter tree shows only Chapter 1 and the editor is empty.
- Cancel New; the existing story, selection, dirty flag, and overlays remain unchanged.
- Click New while export or command palette is open; the overlay closes only after the New decision is resolved.

## 경계 / 예외
- Click New with unsaved text; the app asks for confirmation or preserves a recoverable undo/safety path before replacing the document.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
