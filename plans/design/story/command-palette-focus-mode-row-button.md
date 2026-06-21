# Design: command-palette-focus-mode-row-button

## 한 줄 정의
커맨드 팔레트 오버레이의 Focus mode 커맨드 행. 클릭 시 포커스 모드 토글.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Focus mode row | `Button` | `story.command.focus_mode` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 커맨드 행 상태(default, hover, active, disabled) 사용.
