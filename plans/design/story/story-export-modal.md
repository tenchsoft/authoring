# Design: story-export-modal

## 한 줄 정의
익스포트 모달. 7개 포맷 버튼(DOCX, PDF, EPUB, Markdown, HTML, Plain Text, Bundle)을 수직 리스트로 표시. 백드롭 오버레이 포함.

## 시각적 레이아웃
```
┌─ Backdrop (full screen, dim) ─────────────────────────────────────────────────┐
│                                                                                │
│         ┌─ Modal (340x280, centered) ──────────────────────────┐              │
│         │ Export Story                              [backdrop] │              │
│         │ Project Name | 1234 words                              │              │
│         │ ──────────────────────────────────────────────────── │              │
│         │ ┌───────────────────────────────────────────────────┐│              │
│         │ │ DOCX (.docx)                                      ││              │
│         │ └───────────────────────────────────────────────────┘│              │
│         │ ┌───────────────────────────────────────────────────┐│              │
│         │ │ PDF (.pdf)                                        ││              │
│         │ └───────────────────────────────────────────────────┘│              │
│         │ ┌───────────────────────────────────────────────────┐│              │
│         │ │ EPUB (.epub)                                      ││              │
│         │ └───────────────────────────────────────────────────┘│              │
│         │ ┌───────────────────────────────────────────────────┐│              │
│         │ │ Markdown (.md)                                    ││              │
│         │ └───────────────────────────────────────────────────┘│              │
│         │ ┌───────────────────────────────────────────────────┐│              │
│         │ │ HTML (.html)                                      ││              │
│         │ └───────────────────────────────────────────────────┘│              │
│         │ ┌───────────────────────────────────────────────────┐│              │
│         │ │ Plain Text (.txt)                                 ││              │
│         │ └───────────────────────────────────────────────────┘│              │
│         │ ┌───────────────────────────────────────────────────┐│              │
│         │ │ Tench Story Bundle (.tench-story)                 ││              │
│         │ └───────────────────────────────────────────────────┘│              │
│         └──────────────────────────────────────────────────────┘              │
└───────────────────────────────────────────────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Export backdrop | `Button` | `story.export.backdrop` |
| Export modal | `Dialog` | `story.export.modal` |
| DOCX button | `Button` | `story.export.docx` |
| PDF button | `Button` | `story.export.pdf` |
| EPUB button | `Button` | `story.export.epub` |
| Markdown button | `Button` | `story.export.markdown` |
| HTML button | `Button` | `story.export.html` |
| Plain Text button | `Button` | `story.export.plain_text` |
| Bundle button | `Button` | `story.export.bundle` |

## Visual properties
| 속성 | 값 |
|------|----|
| Modal size | 340x280, centered |
| Modal background | `theme.surface` |
| Modal border | `theme.border`, 1px |
| Modal border radius | `theme.border_radius` |
| Title font | 16px, weight Bold, `theme.on_surface` |
| Subtitle font | 12px, weight Normal, `theme.secondary` |
| Format row bg | `theme.background`, radius 6px |
| Format row height | 30px |
| Format row step | 36px |
| Format row padding | modal 좌우 16px |
| Format row text | 12px, weight Bold, `theme.on_surface` |
| Format row text x-offset | row 내부 12px |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Backdrop | dim overlay | — | — | — | — |
| Format row | `theme.background` bg | lighten 4% | darken 8% | 2px outline `theme.primary` | opacity 0.5 |

## Animations / transitions
| Trigger | Property | Duration | Easing |
|---------|----------|----------|--------|
| Modal open | opacity 0→1 + translateY 8px→0 | 150ms | ease-out |
| Modal close | 역방향 | 100ms | ease-in |

## Responsive 변형
- **Desktop (1280x820)**: 위 명세 그대로.
- **Mobile (390x844)**: 모달 full-width minus 16px margin, height auto.

## Accessibility
- Escape 키로 모달 닫기.
- 백드롭 클릭으로 모달 닫기.
- 포맷 버튼에 focus indicator.
- 탭 순서: 포맷 버튼 → 백드롭.

## Design tokens — 사용 / 제안
- **사용**: `theme.surface`, `theme.background`, `theme.on_surface`, `theme.secondary`, `theme.border`, `theme.border_radius`, `theme.primary`.
- **신규 제안**: 없음.

## Out of scope
- 익스포트 진행 상태 표시 (별도 spec).
- 익스포트 파일 저장 위치 선택 (OS native dialog).
