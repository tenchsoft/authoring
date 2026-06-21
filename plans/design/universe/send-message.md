# Design: send-message

## 한 줄 정의
작성창의 메시지 전송 버튼. 기존 버튼 컴포넌트 재사용.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Send button | `Button` | `universe.composer.send` |

## Visual properties
| 속성 | 값 |
|------|----|
| 버튼 높이 | `theme.button_height` (36px) |
| 아이콘 | 전송 화살표 |
| 배경 | `theme.primary` |
| 전경 | `theme.on_primary` |

## States
| Component | Default | Hover | Active | Disabled |
|-----------|---------|-------|--------|----------|
| Send button | `theme.primary` | darken 8% | darken 16% | opacity 0.5 (빈 입력 시) |

## Out of scope
- 메시지 전송 로직 (별도 spec).
