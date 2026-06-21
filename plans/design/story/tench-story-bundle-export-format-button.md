# Design: tench-story-bundle-export-format-button

## 한 줄 정의
Export Story 모달의 Tench Story Bundle (.tench-story) 포맷 행. 클릭 시 번들 익스포트 시작.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Tench Story Bundle row | `Button` | `story.export.bundle` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 익스포트 포맷 행 상태(default, hover, active, disabled) 사용.
