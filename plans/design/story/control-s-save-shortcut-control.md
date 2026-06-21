# Design: control-s-save-shortcut-control

## 한 줄 정의
Ctrl+S 단축키. 현재 스토리 저장.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Save action | `Action` | `story.header.save` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요. 단축키는 시각 요소 없음.

## States
기존 헤더 Save 버튼 상태 사용. 단축키 자체는 별도 시각 상태 없음.
