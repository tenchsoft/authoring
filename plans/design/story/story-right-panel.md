# Design: story-right-panel

## 한 줄 정의
우측 보조 패널. 9개 탭(Characters, World, Timeline, Comments, Stats, Glossary, Relationships, MindMap, AI)과 탭별 콘텐츠 영역으로 구성.

## 시각적 레이아웃
```
┌─ Right Panel (300px) ──────────────────────────────────────────┐
│ Chars World Time Notes Stats Gloss Rel Mind AI                 │
│ ─────────────────────────────────────────────────────────────── │
│ [Tab title]                                                     │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Row item title                                              │ │
│ │ Row item detail                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Row item title                                              │ │
│ │ Row item detail                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ ...                                                             │
└─────────────────────────────────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Tab: Characters | `Tab` | `story.tab.characters` |
| Tab: World | `Tab` | `story.tab.world` |
| Tab: Timeline | `Tab` | `story.tab.timeline` |
| Tab: Comments | `Tab` | `story.tab.comments` |
| Tab: Stats | `Tab` | `story.tab.stats` |
| Tab: Glossary | `Tab` | `story.tab.glossary` |
| Tab: Relationships | `Tab` | `story.tab.relationships` |
| Tab: MindMap | `Tab` | `story.tab.mind_map` |
| Tab: AI | `Tab` | `story.tab.ai_assist` |
| Right panel content | `Region` | `story.right_panel.content` |
| Character row (idx N) | `Button` | `story.character.{N}` |
| World row (idx N) | `Button` | `story.world.{N}` |
| Timeline row (idx N) | `Button` | `story.timeline.{N}` |
| Comment row (idx N) | `Button` | `story.comment.{N}` |
| Statistics refresh indicator | `Status` | `story.statistics.refresh` |
| Statistic row (idx N) | `Button` | `story.statistics.{N}` |
| Glossary row (idx N) | `Button` | `story.glossary.{N}` |
| Relationship row (idx N) | `Button` | `story.relationship.{N}` |
| MindMap: Premise | `Button` | `story.mind_map.premise` |
| MindMap: Conflict | `Button` | `story.mind_map.conflict` |
| MindMap: Setting | `Button` | `story.mind_map.setting` |
| MindMap: Character arc | `Button` | `story.mind_map.character_arc` |
| AI assist placeholder | `Button` | `story.ai.placeholder` |

## Visual properties
| 속성 | 값 |
|------|----|
| Panel background | `theme.surface` |
| Panel width | 300px |
| Tab bar height | 20px |
| Tab text size | 9px, weight Normal |
| Tab text color (default) | `theme.on_surface` |
| Tab text color (active) | `theme.primary` |
| Tab step | 32px horizontal |
| Section title font | 14px, weight Bold, `theme.on_background` |
| Row background | `theme.background`, radius 6px |
| Row title font | 13px, weight Normal, `theme.on_background` |
| Row detail font | 11px, weight Normal, `theme.on_surface` |
| Row height | 34px |
| Row step (Characters) | 40px |
| Row step (World) | 46px |
| Row step (Timeline) | 44px |
| Row step (Comments) | 44px |
| Row step (Stats) | 38px |
| Row step (Glossary) | 44px |
| Row step (Relationships) | 44px |
| MindMap node size | 104x38px, `theme.background` bg |
| MindMap node text | 11px, weight Bold, centered, `theme.on_surface` |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Tab label | `theme.on_surface` | lighten 8% | — | 2px outline `theme.primary` | opacity 0.5 |
| Tab label (active) | `theme.primary` | darken 8% | — | 2px outline | opacity 0.5 |
| Content row | `theme.background` bg | lighten 4% | — | 2px outline | opacity 0.5 |
| MindMap node | `theme.background` bg | lighten 4% | — | 2px outline | opacity 0.5 |

## Animations / transitions
| Trigger | Property | Duration | Easing |
|---------|----------|----------|--------|
| Tab switch | content swap | 즉시 | — |

## Responsive 변형
- **Desktop (1280x820)**: 300px 패널.
- **Mobile (390x844)**: 패널 숨김, 하단 시트로 전환.

## Accessibility
- 각 탭에 focus indicator.
- 콘텐츠 행 클릭 가능.
- Escape로 패널 닫기 불가 (패널은 항상 표시, 포커스 모드에서만 숨김).

## Design tokens — 사용 / 제안
- **사용**: `theme.surface`, `theme.background`, `theme.on_background`, `theme.on_surface`, `theme.primary`, `theme.border_radius`.
- **신규 제안**: 없음.

## Out of scope
- 캐릭터/월드/타임라인 등의 편집 UI (별도 spec).
- AI 어시스트 기능 (별도 spec).
- 통계 새로고침의 백그라운드 동작 (`automatic-statistics-refresh-behavior`).
