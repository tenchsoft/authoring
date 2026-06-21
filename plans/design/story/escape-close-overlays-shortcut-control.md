# Design: escape-close-overlays-shortcut-control

## 한 줄 정의
Escape 단축키. 열린 모든 오버레이 닫기.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Overlay dismiss | `Action` | (오버레이별 debug_id 사용) |

## Visual properties
모두 기존 컴포넌트 재사용. 별도 visual properties 불필요. 단축키는 시각 요소 없음.

## States
기존 오버레이 상태 사용. 단축키 자체는 별도 시각 상태 없음.
