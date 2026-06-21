# Design: character-row

## 한 줄 정의
캐릭터 목록의 개별 행. 캐릭터 이름, 아바타, 선택 상태를 표시.

## 시각적 레이아웃
```
┌─ Character Row ───────────────────┐
│ [👤] 캐릭터 이름                  │
└──────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Character row | `Row` | `universe.character.<idx>` |
| Avatar | `Image` | `universe.character.<idx>.avatar` |
| Name | `Label` | `universe.character.<idx>.name` |

## Visual properties
| 속성 | 값 |
|------|----|
| 행 높이 | `theme.list_item_height` (40px) |
| 패딩 | `theme.spacing` (8px) |
| 선택 배경 | `theme.primary` opacity 12% |
| 아바타 크기 | 24px 원형 |

## States
| Component | Default | Hover | Selected | Focus |
|-----------|---------|-------|----------|-------|
| Row | `theme.surface` | lighten 4% | `theme.primary` 12% | 2px outline |

## Out of scope
- 캐릭터 상세 편집 (별도 design).
