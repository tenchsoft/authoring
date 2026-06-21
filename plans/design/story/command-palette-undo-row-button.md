# Design: command-palette-undo-row-button

## 한 줄 정의
커맨드 팔레트 오버레이의 Undo 커맨드 행. 클릭 시 undo 스냅샷 복원.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Undo row | `Button` | `story.command.undo` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 커맨드 행 상태(default, hover, active, disabled) 사용.
