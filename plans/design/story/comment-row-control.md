# Design: comment-row-control

## 한 줄 정의
Comments 패널의 코멘트 행. 클릭 시 코멘트 열기 및 원고 앵커 표시.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Comment row | `Button` | `story.right_panel.comment_row` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 행 상태(default, hover, active, selected, disabled) 사용.
