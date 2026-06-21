# Implement: automatic-right-panel-content-render-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 우측 패널이 `active_tab`에 해당하는 콘텐츠를 엔진 데이터에서 가져와 렌더한다.
- 탭 전환 시 콘텐츠가 즉시 교체된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` | `active_tab` match로 각 탭 콘텐츠 렌더 | `fn paint_tab_content` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` | right panel 영역에서 `paint_tab_content` 호출 | `grep -n 'paint_tab_content' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::select_tab` | `active_tab` 갱신 | `fn select_tab` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.right_panel.content` 노드 emit | `grep -n 'story.right_panel.content'` |

## 필요한 변경 (의도 단위)

### 1. 탭 콘텐츠 렌더 디스패치
- **입력**: `state.active_tab`, `state.engine.get_document()`, 패널 좌표
- **처리**: `paint_tab_content`에서 `match state.active_tab`으로 분기:
  - Characters: `doc.characters` 순회, 이름/역할 행 렌더.
  - World: `doc.world_entries` 순회, 이름/카테고리 행 렌더.
  - Timeline: `doc.timeline_events` 순회, 날짜/제목 행 렌더.
  - Comments: `doc.comments` 순회, 상태/텍스트 행 렌더.
  - Stats: `engine.statistics()` 호출 후 10개 통계 행 렌더.
  - Glossary: `doc.glossary` 순회, 용어/정의 행 렌더.
  - Relationships: `doc.relationships` 순회, 캐릭터 이름 조회 후 관계 행 렌더.
  - MindMap: 4개 고정 노드(premise/conflict/setting/character_arc) 그리드 렌더.
  - AiAssist: placeholder 텍스트 렌더.
- **출력/사이드 이펙트**: 해당 탭 콘텐츠 시각적 렌더.
- **순서/우선순위**: 탭 바 렌더 이후.

### 2. 탭 전환 시 즉시 반영
- **입력**: 탭 클릭 → `select_tab(new_tab)` → `active_tab = new_tab`
- **처리**: 다음 repaint에서 `paint_tab_content`가 새 `active_tab`으로 분기.
- **출력/사이드 이펙트**: 콘텐츠 즉시 교체.
- **순서/우선순위**: `select_tab` 호출 직후 `request_paint`.

### 3. 자동화 노드
- **입력**: `!focus_mode`
- **처리**: `story.right_panel.content` Region 노드를 우측 패널 전체 영역에 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 탭 노드 이후, 패널 행 노드 전.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.right_panel.content` | Region | `"Right panel content"` | `!focus_mode` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: 모든 탭별 콘텐츠 implement (characters, world, timeline, stats, glossary, relationships, mind_map, ai_assist).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'paint_tab_content' panels.rs mod.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
