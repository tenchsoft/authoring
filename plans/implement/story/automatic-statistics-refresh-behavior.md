# Implement: automatic-statistics-refresh-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Stats 탭이 활성일 때마다 엔진에서 통계를 재계산하여 패널에 표시한다.
- 매 repaint마다 최신 데이터가 반영된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/panels.rs::paint_tab_content` | `StoryTab::Stats` 분기에서 `engine.statistics()` 호출 | `grep -n 'Stats' panels.rs` |
| `crates/story-core/src/engine.rs` | `statistics()` 메서드 | `pub fn statistics` |
| `apps/story/src-tauri/src/ui/mod.rs::push_panel_row_nodes` | `StoryTab::Stats` 분기에서 `story.statistics.refresh` + 행 노드 emit | `grep -n 'story.statistics' mod.rs` |

## 필요한 변경 (의도 단위)

### 1. 통계 재계산
- **입력**: `active_tab == Stats`, 매 repaint
- **처리**: `paint_tab_content`의 Stats 분기에서 `state.engine.statistics()` 호출. 반환된 `Statistics` 구조체에서 total_words, total_characters, total_sentences, avg_sentence_length, reading_time_minutes, chapter_count, character_count, world_entry_count, timeline_event_count, glossary_entry_count를 10개 행으로 렌더.
- **출력/사이드 이펙트**: 최신 통계 값 표시. 캐싱 없음 — 매 paint마다 재계산.
- **순서/우선순위**: 패널 타이틀 렌더 이후.

### 2. 자동화 노드
- **입력**: `active_tab == Stats && !focus_mode`
- **처리**: `story.statistics.refresh` Status 노드를 첫 행 위치에 emit. 이후 10개 `story.statistics.{idx}` Button 노드를 38.0 간격으로 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 다른 패널 행 노드와 동일.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.statistics.refresh` | Status | `"Statistics refresh"` | `active_tab == Stats && !focus_mode` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-right-panel-content-render-behavior` (동일 패널).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'Stats\|statistics' panels.rs mod.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
