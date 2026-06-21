# Design: story-manuscript-editor

## 한 줄 정의
중앙 메인 영역의 원고 에디터. 챕터 제목, 본문 텍스트, 깜빡이는 커서로 구성. 카드 형태의 서피스 위에 렌더링.

## 시각적 레이아웃
```
┌─ Center Panel ──────────────────────────────────────────────────┐
│  ┌─ Editor Card ──────────────────────────────────────────────┐ │
│  │ Chapter Title (16px Bold)                                  │ │
│  │                                                             │ │
│  │ Line 1 of chapter content...                               │ │
│  │ Line 2 of chapter content...                               │ │
│  │ Line 3 of chapter content...|  ← blinking cursor           │ │
│  │                                                             │ │
│  └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Editor card | `Textbox` | `story.manuscript.editor` |
| Cursor indicator | `Status` | `story.cursor` |

## Visual properties
| 속성 | 값 |
|------|----|
| Center panel background | `theme.background` |
| Editor card background | `theme.surface` |
| Editor card border radius | 8px |
| Editor card margin | 16px (all sides) |
| Chapter title font | 16px, weight Bold, `theme.on_background` |
| Chapter title position | card 내부 x+12, y+24 |
| Content font | 14px, weight Normal, `theme.on_background` |
| Content line height | 22px |
| Content start y | chapter title 아래 28px |
| Content max y | card bottom - 40px |
| Cursor color | `theme.primary` |
| Cursor size | 2px wide, 16px tall |
| Cursor position | 마지막 텍스트 라인 아래 |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Editor card | `theme.surface` bg | — | — | — | — |
| Cursor | `theme.primary`, visible | — | — | — | hidden (포커스 없을 시) |

## Animations / transitions
| Trigger | Property | Duration | Easing |
|---------|----------|----------|--------|
| Cursor blink | opacity 0↔1 | 530ms | linear |

## Responsive 변형
- **Desktop (1280x820)**: 패널 폭 = 전체 - 220px(좌) - 300px(우).
- **Mobile (390x844)**: 전체 폭 사용, 카드 마진 8px.
- **Focus mode**: 좌/우 패널 숨김, 에디터가 전체 폭 차지.

## Accessibility
- 커서가 시각적으로 항상 표시 (포커스 시).
- 키보드 입력으로 텍스트 편집 가능 (문자, Enter, Backspace).

## Design tokens — 사용 / 제안
- **사용**: `theme.background`, `theme.surface`, `theme.on_background`, `theme.primary`, `theme.border_radius`.
- **신규 제안**: 없음.

## Out of scope
- 리치 텍스트 편집 (현재 plain text).
- 텍스트 선택 범위 하이라이트 (별도 spec).
- 커서 배치의 백그라운드 동작 (`automatic-cursor-placement-behavior`).
