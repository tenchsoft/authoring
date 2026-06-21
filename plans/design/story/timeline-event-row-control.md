# Design: timeline-event-row-control

## 한 줄 정의
Timeline 패널의 타임라인 이벤트 행. 클릭 시 이벤트 상세 에디터 열기.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Timeline event row | `Button` | `story.right_panel.timeline_row` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 행 상태(default, hover, active, selected, disabled) 사용.
