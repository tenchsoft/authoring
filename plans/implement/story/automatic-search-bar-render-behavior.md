# Implement: automatic-search-bar-render-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- `show_search == true`일 때 검색 바가 쿼리 텍스트와 대소문자 구분 토글을 표시한다.
- 쿼리가 비어있으면 placeholder 텍스트("Type to search...")를 표시한다.
- 대소문자 구분 상태가 라벨에 반영된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/editor.rs::paint_search_bar` | 검색 바 프레임 + 쿼리 + 라벨 렌더 | `fn paint_search_bar` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` | overlay 섹션에서 `show_search` 분기 | `grep -n 'show_search' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 검색 바 클릭, 대소문자 토글 클릭 | `grep -n 'search_case_rect\|search_bar_rect' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::toggle_search` | `show_search` 토글, `input_focus` 전환 | `fn toggle_search` |
| `apps/story/src-tauri/src/ui/state.rs::toggle_search_case_sensitive` | `search_case_sensitive` 토글 | `fn toggle_search_case_sensitive` |
| `apps/story/src-tauri/src/ui/state.rs::append_search_text` | 쿼리 버퍼에 텍스트 추가 | `fn append_search_text` |
| `apps/story/src-tauri/src/ui/state.rs::backspace_search` | 쿼리 버퍼에서 마지막 문자 제거 | `fn backspace_search` |

## 필요한 변경 (의도 단위)

### 1. 검색 바 렌더
- **입력**: `show_search == true`, `search_query`, `search_case_sensitive`
- **처리**: `paint_search_bar`에서 바 rect(`search_bar_rect`)에 `theme.surface` 배경, `theme.border` 스트로크. 라벨: `search_case_sensitive`이면 "Search (case-sensitive)", 아니면 "Search". 쿼리 표시: 비어있으면 "Type to search..." (`theme.secondary`), 있으면 실제 쿼리 (`theme.on_background`).
- **출력/사이드 이펙트**: 시각적 오버레이.
- **순서/우선순위**: overlay 섹션 세 번째 (export, command palette 이후).

### 2. 검색 바 클릭 → input_focus 전환
- **입력**: pointer down, `show_search == true`, `search_bar_rect` 내 클릭
- **처리**: `input_focus = StoryInputFocus::Search`.
- **출력/사이드 이펙트**: 키보드 입력이 검색 쿼리로 라우팅.
- **순서/우선순위**: 오버레이 hit-test 내.

### 3. 대소문자 토글 클릭
- **입력**: pointer down, `search_case_rect` 내 클릭
- **처리**: `toggle_search_case_sensitive()` → `search_case_sensitive` 토글.
- **출력/사이드 이펙트**: 라벨 변경, repaint.
- **순서/우선순위**: 검색 바 클릭 체크 전 (좌측 영역).

### 4. 텍스트 입력 라우팅
- **입력**: Character 키, `input_focus == Search`
- **처리**: `append_search_text(ch)` → 쿼리 버퍼에 추가.
- **출력/사이드 이펙트**: 쿼리 텍스트 업데이트, repaint.
- **순서/우선순위**: Manuscript 입력 분기 전.

### 5. 자동화 노드
- **입력**: `show_search == true`
- **처리**: `story.search.bar`(Dialog), `story.search.query`(Textbox), `story.search.case_sensitive`(Checkbox) 노드 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: command palette 노드 이후.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.search.bar` | Dialog | `"Search"` | `show_search == true` |
| `story.search.query` | Textbox | `"Search query"` | `show_search == true` |
| `story.search.case_sensitive` | Checkbox | `"Case sensitive"` | `show_search == true` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-overlay-exclusivity-behavior` (동일 오버레이 메커니즘).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'show_search\|paint_search_bar\|search_query' mod.rs editor.rs state.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
