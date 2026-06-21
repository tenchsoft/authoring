# Design: command-palette-world-building-panel-row-button

## 한 줄 정의
커맨드 팔레트 오버레이의 World building panel 커맨드 행. 클릭 시 World Building 우측 패널 열기.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| World building panel row | `Button` | `story.command.world_building_panel` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 커맨드 행 상태(default, hover, active, disabled) 사용.
