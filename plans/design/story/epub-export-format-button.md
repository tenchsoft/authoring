# Design: epub-export-format-button

## 한 줄 정의
Export Story 모달의 EPUB (.epub) 포맷 행. 클릭 시 EPUB 익스포트 시작.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| EPUB row | `Button` | `story.export.epub` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 익스포트 포맷 행 상태(default, hover, active, disabled) 사용.
