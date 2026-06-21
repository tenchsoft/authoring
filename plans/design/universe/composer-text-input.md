# Design: composer-text-input

## 한 줄 정의
하단 작성창의 텍스트 입력 영역. 멀티라인 입력 지원.

## 시각적 레이아웃
```
┌─ Composer ────────────────────────────────┐
│ [메시지를 입력하세요...]                    │
│                                [전송] [⚙]  │
└───────────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Composer container | `Container` | `universe.composer` |
| Text input | `TextInput` | `universe.composer.input` |
| Send button | `Button` | `universe.composer.send` |

## Visual properties
| 속성 | 값 |
|------|----|
| 배경 | `theme.surface` |
| 테두리 | `theme.border`, `border_radius = theme.border_radius` |
| 최소 높이 | `theme.input_height` (32px) |
| 최대 높이 | 120px |
| 패딩 | `theme.spacing` (8px) |

## States
| Component | Default | Focus | Disabled |
|-----------|---------|-------|----------|
| Text input | `theme.surface` | 2px outline `theme.primary` | opacity 0.5 |

## Out of scope
- 전송 동작 (별도 spec).
- 플레이스홀더 변경 (별도 background).
