# Design: command-palette-redo-row-button

## 한 줄 정의
커맨드 팔레트 오버레이의 Redo 커맨드 행. 클릭 시 redo 스냅샷 복원.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Redo row | `Button` | `story.command.redo` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 커맨드 행 상태(default, hover, active, disabled) 사용.
