# Design: header-settings-icon

## 한 줄 정의
상단 헤더의 설정 아이콘 버튼. 클릭 시 설정 패널이 열린다.

## 시각적 레이아웃
```
┌─ Header ──────────────────────────────┐
│  [Tab] [Tab] [Tab] [Tab]    [⚙]      │
└───────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Settings icon button | `Button` | `universe.header.settings` |

## Visual properties
| 속성 | 값 |
|------|----|
| 버튼 크기 | `theme.icon_size` (24px) |
| 아이콘 | 설정 톱니바퀴 |
| 패딩 | `theme.spacing` (8px) |

## States
| Component | Default | Hover | Active | Focus | Disabled |
|-----------|---------|-------|--------|-------|----------|
| Settings button | `theme.surface` | lighten 8% | lighten 16% | 2px outline `theme.primary` | opacity 0.5 |

## Out of scope
- 설정 패널 내용 (별도 design).
