# Design: command-palette-add-chapter-row-button

## 한 줄 정의
커맨드 팔레트 오버레이의 Add chapter 커맨드 행. 클릭 시 새 챕터 생성.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Add chapter row | `Button` | `story.command.add_chapter` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 커맨드 행 상태(default, hover, active, disabled) 사용.
