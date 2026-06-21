# Spec: manuscript-enter-key-control

## 한 줄 정의
사용자가 Story에서 Manuscript Enter Key Control을/를 단축키로 수행한다.

## 진입점
- 단축키: (해당 단축키)

## 사용자 흐름
1. From the user's perspective, this center manuscript editor control is independent and must not be merged with adjacent controls. When the user activates it by presses Enter while the editor is active, a newline is inserted into the selected chapter and the cursor moves to the next visual line.

## 성공 조건 (Acceptance Criteria)
- [ ] Press Enter between paragraphs; a line break appears in chapter content.
- [ ] Press Enter repeatedly; multiple blank lines are stored predictably.
- [ ] Press Enter while command palette is focused; it activates the highlighted command instead of inserting manuscript text.
- [ ] Press Enter near the bottom of the editor; content scroll behavior or clipping remains predictable.

## 실패 / 취소 흐름
- 모달/다이얼로그가 활성 중이면 단축키가 무시된다.
- 입력 필드에 포커스가 있으면 단축키가 입력으로 처리된다.

## 경계 / 예외
- Press Enter repeatedly; multiple blank lines are stored predictably.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
