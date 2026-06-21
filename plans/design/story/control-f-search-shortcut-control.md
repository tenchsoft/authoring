# Design: control-f-search-shortcut-control

## 한 줄 정의
Ctrl+F 단축키. 검색 오버레이 토글.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Search overlay | `Dialog` | `story.search_bar` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요. 단축키는 시각 요소 없음.

## States
기존 검색 오버레이 상태 사용. 단축키 자체는 별도 시각 상태 없음.
