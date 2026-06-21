# Design: markdown-export-format-button

## 한 줄 정의
Export Story 모달의 Markdown (.md) 포맷 행. 클릭 시 Markdown 익스포트 시작.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Markdown row | `Button` | `story.export.markdown` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 익스포트 포맷 행 상태(default, hover, active, disabled) 사용.
