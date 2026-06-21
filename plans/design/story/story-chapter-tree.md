# Design: story-chapter-tree

## 한 줄 정의
좌측 패널의 챕터 트리. 프로젝트 내 모든 챕터를 수직 리스트로 표시하며, 선택된 챕터는 하이라이트.

## 시각적 레이아웃
```
┌─ Left Panel (220px) ─────────┐
│ Chapters                      │
│ ┌──────────────────────────┐  │
│ │ Chapter 1                │  │  ← selected
│ └──────────────────────────┘  │
│ ┌──────────────────────────┐  │
│ │ Chapter 2                │  │
│ └──────────────────────────┘  │
│ ┌──────────────────────────┐  │
│ │ Chapter 3                │  │
│ └──────────────────────────┘  │
│ ...                           │
└───────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Left panel background | `Container` | (패널 자체, 별도 노드 없음) |
| "Chapters" heading | `Label` | (정적 텍스트) |
| Chapter row (idx N) | `Button` | `story.chapter.{N}` |
| Selected chapter indicator | `Status` | `story.chapter.selected` |

## Visual properties
| 속성 | 값 |
|------|----|
| Panel background | `theme.surface` |
| Panel width | 220px |
| Heading font | 14px, weight Bold, `theme.on_background` |
| Row height | 30px |
| Row step | 36px |
| Row padding | x: 8px (왼쪽), 8px (오른쪽) |
| Row border radius | 6px |
| Row bg (default) | `theme.surface` |
| Row bg (selected) | `theme.primary` |
| Row text color (default) | `theme.on_background` |
| Row text color (selected) | `theme.on_primary` |
| Row text size | 12px, weight Normal |
| Row text x-offset | 16px |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Chapter row | `theme.surface` bg | lighten 4% | — | 2px outline `theme.primary` | opacity 0.5 |
| Chapter row (selected) | `theme.primary` bg | darken 8% | — | 2px outline | opacity 0.5 |

## Animations / transitions
| Trigger | Property | Duration | Easing |
|---------|----------|----------|--------|
| Selection change | bg color | 80ms | linear |

## Responsive 변형
- **Desktop (1280x820)**: 220px 패널.
- **Mobile (390x844)**: 패널은 숨김, 햄버거 메뉴로 전환.

## Accessibility
- 각 챕터 행에 focus indicator.
- 키보드 화살표로 챕터 간 이동 가능.

## Design tokens — 사용 / 제안
- **사용**: `theme.surface`, `theme.primary`, `theme.on_primary`, `theme.on_background`, `theme.border_radius`.
- **신규 제안**: 없음.

## Out of scope
- 챕터 컨텍스트 메뉴 (별도 spec).
- 챕터 드래그 앤 드롭 순서 변경 (별도 spec).
- 챕터 선택 하이라이트의 백그라운드 동작 (`automatic-chapter-selection-highlight-behavior`).
