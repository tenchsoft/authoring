# Design: manuscript-backspace-key-control

## 한 줄 정의
중앙 원고 에디터의 Backspace 키. 에디터 활성 시 이전 문자 삭제.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Manuscript editor | `TextInput` | `story.manuscript.editor` |
| Cursor | `Cursor` | `story.cursor` |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 에디터 상태(active, inactive, dirty) 사용.
