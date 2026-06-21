# Design: story-automatic-ui

## 한 줄 정의
스토리 앱의 자동 UI 동작. 사용자 액션 없이 화면에 자동으로 표시되는 모든 시각 요소를 정의.

## 시각적 레이아웃
각 자동 UI 요소는 헤더, 챕터 트리, 에디터, 우측 패널, 상태 바 위에 오버레이 또는 인라인으로 표시.

## Component breakdown

### Highlight / indicator behaviors
| Component | role | debug_id | 설명 |
|-----------|------|----------|------|
| Chapter selection highlight | `Status` | `story.chapter.selected` | 선택된 챕터 행에 `theme.primary` bg |
| Dirty title indicator | `Status` | `story.dirty_title` | 프로젝트 제목에 `" *"` 접미어 |
| Word count badge | `Status` | `story.word_count` | 헤더 우측 워드 카운트 |
| Status bar sync | `Status` | `story.status_bar` | 하단 상태 바 자동 갱신 |
| Statistics refresh | `Status` | `story.statistics.refresh` | 통계 탭 선택 시 자동 갱신 |
| Focus mode layout | `Status` | `story.focus_layout` | 포커스 모드 시 좌/우 패널 숨김 |
| Overlay exclusivity | `Status` | `story.overlay.exclusive` | 동시에 하나의 오버레이만 표시 |
| Cursor indicator | `Status` | `story.cursor` | 에디터 내 깜빡이는 커서 |

### Overlay render behaviors
| Component | role | debug_id | 설명 |
|-----------|------|----------|------|
| Export modal render | `Dialog` | `story.export.modal` | `show_export=true` 시 모달 표시 |
| Command palette render | `Dialog` | `story.command.palette` | `show_command_palette=true` 시 팔레트 표시 |
| Search bar render | `Dialog` | `story.search.bar` | `show_search=true` 시 검색 바 표시 |
| Right panel content render | `Region` | `story.right_panel.content` | 활성 탭에 따라 콘텐츠 자동 전환 |

## Visual properties
| 속성 | 값 |
|------|----|
| Chapter selection bg | `theme.primary`, fg `theme.on_primary` |
| Dirty indicator | `" *"` 텍스트, `theme.on_background` |
| Word count badge bg | `theme.border`, radius 8px |
| Status bar bg | `theme.surface`, text `theme.secondary`, 11px |
| Focus mode | 좌/우 패널 폭 0px, 에디터 전체 폭 |
| Overlay exclusivity | export, command, search 중 하나만 표시 |
| Cursor color | `theme.primary` |
| Cursor blink | opacity 0↔1, 530ms linear |

## Animations / transitions
| Trigger | Property | Duration | Easing |
|---------|----------|----------|--------|
| Cursor blink | opacity 0↔1 | 530ms | linear |
| Chapter selection | bg color | 80ms | linear |
| Focus mode toggle | panel width | 150ms | ease-out |

## Out of scope
- 각 동작의 트리거 조건 (background에서 정의).
- 익스포트 모달 내부 디자인 (`story-export-modal`).
- 커맨드 팔레트 내부 디자인 (별도 design).
