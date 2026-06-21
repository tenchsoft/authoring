# Design: chapter-tree-row-control

## 한 줄 정의
좌측 챕터 트리의 챕터 행. 클릭 시 해당 챕터 선택 및 에디터 전환.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Chapter row | `Button` | `story.chapter.<idx>` |
| Selected indicator | `Label` | `story.chapter.selected` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 행 상태(default, hover, active, selected) 사용.
