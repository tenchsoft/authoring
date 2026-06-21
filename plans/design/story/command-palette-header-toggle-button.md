# Design: command-palette-header-toggle-button

## 한 줄 정의
헤더 액션 바의 Cmd 버튼. 클릭 시 커맨드 팔레트 토글.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Command button | `Button` | `story.header.command` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 헤더 버튼 상태(default, hover, active, disabled) 사용.
