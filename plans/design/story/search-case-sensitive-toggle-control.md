# Design: search-case-sensitive-toggle-control

## 한 줄 정의
검색 오버레이의 대소문자 구분 토글. 클릭 시 대소문자 구분 on/off 전환.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Case sensitive toggle | `Switch` | `story.search_bar.case_sensitive` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 토글 상태(off, on, hover, focus, disabled) 사용.
