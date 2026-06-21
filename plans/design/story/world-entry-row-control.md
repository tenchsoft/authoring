# Design: world-entry-row-control

## 한 줄 정의
World Building 패널의 월드 엔트리 행. 클릭 시 월드 엔트리 상세 에디터 열기.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| World entry row | `Button` | `story.right_panel.world_row` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 행 상태(default, hover, active, selected, disabled) 사용.
