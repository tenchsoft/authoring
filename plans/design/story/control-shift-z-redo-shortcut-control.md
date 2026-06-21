# Design: control-shift-z-redo-shortcut-control

## 한 줄 정의
Ctrl+Shift+Z 단축키. redo 스냅샷 복원.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Manuscript editor | `TextInput` | `story.manuscript.editor` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요. 단축키는 시각 요소 없음.

## States
기존 에디터 상태 사용. 단축키 자체는 별도 시각 상태 없음.
