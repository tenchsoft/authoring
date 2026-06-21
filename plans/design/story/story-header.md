# Design: story-header

## 한 줄 정의
스토리 앱 상단 헤더 바. 프로젝트 제목(더티 표시 포함), 액션 버튼(New, Open, Save, Export, Focus, Command), 워드 카운트 배지로 구성.

## 시각적 레이아웃
```
┌─ Header (48px) ───────────────────────────────────────────────────────────────┐
│ Untitled Story * │ [New] [Open] [Save] [Export] [Focus] [Cmd] │  1234 words  │
└───────────────────────────────────────────────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Project title | `Label` | (header 내 텍스트, 별도 노드 없음) |
| Dirty indicator | `Label` | `story.dirty_title` |
| New button | `Button` | `story.header.new` |
| Open button | `Button` | `story.header.open` |
| Save button | `Button` | `story.header.save` |
| Export button | `Button` | `story.header.export` |
| Focus button | `Button` | `story.header.focus` |
| Command button | `Button` | `story.header.command` |
| Word count badge | `Label` | `story.word_count` |

## Visual properties
| 속성 | 값 |
|------|----|
| Header background | `theme.surface` |
| Header height | 48px |
| Title font | `theme.font_size_large` (18px), weight ExtraBold |
| Title color | `theme.on_background` |
| Dirty suffix | `" *"` appended to title, same style |
| Button width | 58px |
| Button height | 28px (y: 10–38) |
| Button gap | 6px (step 64px) |
| Button border radius | 6px |
| Button bg (default) | `theme.background` |
| Button bg (active) | `theme.primary` |
| Button border | `theme.border`, 1px |
| Button text size | 11px, weight Bold, centered |
| Button text color (default) | `theme.on_surface` |
| Button text color (active) | `theme.on_primary` |
| Word count badge bg | `theme.border`, radius 8px |
| Word count badge position | right-aligned, width ~164px |
| Word count text | 13px, weight Normal, `theme.on_surface` |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Header button | `theme.background` bg | lighten 8% | darken 8% | 2px outline `theme.primary` | opacity 0.5 |
| Focus button (on) | `theme.primary` bg, `theme.on_primary` fg | darken 8% | — | 2px outline | opacity 0.5 |
| Command button (on) | `theme.primary` bg, `theme.on_primary` fg | darken 8% | — | 2px outline | opacity 0.5 |
| Word count badge | `theme.border` bg | — | — | — | — |

## Animations / transitions
| Trigger | Property | Duration | Easing |
|---------|----------|----------|--------|
| Button active toggle | bg color | 80ms | linear |

## Responsive 변형
- **Desktop (1280x820)**: 위 명세 그대로.
- **Mobile (390x844)**: 버튼 텍스트를 아이콘으로 축소, 워드 카운트 배지는 헤더 아래 별도 행으로 이동.

## Accessibility
- 모든 버튼에 focus indicator 명시.
- 버튼 텍스트가 버튼 너비에 맞게 중앙 정렬.
- Escape 키로 활성 토글 해제.

## Design tokens — 사용 / 제안
- **사용**: `theme.surface`, `theme.background`, `theme.primary`, `theme.on_primary`, `theme.on_surface`, `theme.on_background`, `theme.border`, `theme.font_size_large`, `theme.border_radius`.
- **신규 제안**: 없음.

## Out of scope
- 커맨드 팔레트 오버레이 디자인 (`story-command-palette`, 별도 design).
- 익스포트 모달 오버레이 디자인 (`story-export-modal`, 별도 design).
- 더티 타이틀 인디케이터의 백그라운드 동작 (`automatic-dirty-title-indicator-behavior`).
