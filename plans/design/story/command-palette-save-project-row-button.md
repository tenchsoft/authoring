# Design: command-palette-save-project-row-button

## 한 줄 정의
커맨드 팔레트 오버레이의 Save project 커맨드 행. 클릭 시 현재 프로젝트 저장.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Save project row | `Button` | `story.command.save_project` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 커맨드 행 상태(default, hover, active, disabled) 사용.
