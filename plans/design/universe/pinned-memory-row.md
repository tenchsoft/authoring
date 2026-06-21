# Design: pinned-memory-row

## 한 줄 정의
고정된 메모리 항목 행. 메모리 텍스트와 핀 아이콘을 표시.

## 시각적 레이아웃
```
┌─ Pinned Memory Row ───────────────┐
│ [📌] 메모리 텍스트...              │
└──────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Memory row | `Row` | `universe.memory.<idx>` |
| Pin icon | `Icon` | `universe.memory.<idx>.pin` |
| Text | `Label` | `universe.memory.<idx>.text` |

## Visual properties
| 속성 | 값 |
|------|----|
| 행 높이 | `theme.list_item_height` (40px) |
| 패딩 | `theme.spacing` (8px) |
| 텍스트 | `theme.font_size`, weight 400 |

## Out of scope
- 메모리 CRUD (별도 spec).
