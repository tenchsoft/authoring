# Design: search-query-input-control

## 한 줄 정의
검색 오버레이의 쿼리 입력 필드. 타이핑 시 검색어 업데이트 및 결과 표시.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Search query input | `TextInput` | `story.search_bar.query` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 텍스트 입력 상태(default, focused, disabled) 사용.
