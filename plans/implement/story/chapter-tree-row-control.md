# Implement: chapter-tree-row-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 챕터 행 클릭 시 해당 챕터가 선택되고, 에디터가 해당 챕터 콘텐츠로 전환된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/chapter_tree.rs::hit_test` | 포인터 위치 → 챕터 인덱스 변환 | `fn hit_test` |
| `apps/story/src-tauri/src/ui/chapter_tree.rs::chapter_titles` | 엔진에서 챕터 제목 목록 조회 | `fn chapter_titles` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 챕터 트리 hit-test 분기 | `grep -n 'chapter_tree::hit_test' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::select_chapter` | `selected_chapter_idx` 갱신, `input_focus = Manuscript` | `fn select_chapter` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.chapter.{idx}` 버튼 노드 emit | `grep -n 'story.chapter' mod.rs` |

## 필요한 변경 (의도 단위)

### 1. 챕터 행 hit-test
- **입력**: pointer down, `!focus_mode`, x < 220.0, y >= 92.0
- **처리**: `chapter_tree::hit_test(pos, chapter_count())` 호출. y 좌표를 36.0 간격 그리드로 변환하여 인덱스 계산. `idx < chapter_count`이면 `Some(idx)` 반환.
- **출력/사이드 이펙트**: 챕터 인덱스 또는 None.
- **순서/우선순위**: 오버레이, 헤더 버튼 hit-test 이후.

### 2. 챕터 선택 → 에디터 전환
- **입력**: `hit_test`가 `Some(idx)` 반환
- **처리**: `select_chapter(idx)` 호출 → `selected_chapter_idx = idx`, `input_focus = Manuscript`. 다음 repaint에서 `chapter_text()`와 `chapter_title()`이 새 인덱스의 챕터 데이터 반환.
- **출력/사이드 이펙트**: 에디터 콘텐츠 교체, 챕터 행 하이라이트 이동, repaint.
- **순서/우선순위**: hit-test 성공 직후.

### 3. 챕터 행 렌더
- **입력**: `!focus_mode`, `chapter_titles(state)` 목록
- **처리**: 각 챕터 제목을 36.0 간격 행에 렌더. 선택된 행은 `theme.primary` 배경, 나머지는 `theme.surface`.
- **출력/사이드 이펙트**: 시각적 챕터 목록.
- **순서/우선순위**: left panel 배경 렌더 이후.

### 4. 자동화 노드
- **입력**: `!focus_mode`
- **처리**: 각 챕터마다 `story.chapter.{idx}` Button 노드를 해당 행 위치에 emit. rect: `(8.0, panel_y + 44.0 + idx * 36.0)` ~ `(left_w - 8.0, panel_y + 74.0 + idx * 36.0)`.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 헤더 노드, dirty_title 노드 이후.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.chapter.{idx}` | Button | `"Chapter {idx}"` | `!focus_mode && idx < chapter_count()` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-chapter-selection-highlight-behavior` (동일 영역).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'chapter_tree::hit_test\|select_chapter' mod.rs state.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
