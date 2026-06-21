# Design: statistics-row-control

## 한 줄 정의
Statistics 패널의 통계 행. 클릭 시 관련 소스 뷰 확장.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Statistics row | `Button` | `story.right_panel.statistics_row` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 행 상태(default, hover, active, disabled) 사용.
