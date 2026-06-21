# Implement: automatic-word-count-badge-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 헤더 배지와 status bar의 단어 수가 콘텐츠 변경 시마다 자동으로 업데이트된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::paint` | 헤더 word count badge 렌더 | `grep -n 'word_count' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` | status bar 단어 수 표시 | `grep -n 'total_word_count\|chapter_word_count' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::total_word_count` | 엔진에서 전체 단어 수 조회 | `fn total_word_count` |
| `apps/story/src-tauri/src/ui/state.rs::chapter_word_count` | 엔진에서 현재 챕터 단어 수 조회 | `fn chapter_word_count` |
| `apps/story/src-tauri/src/ui/export.rs::word_count_label` | 단어 수 포맷팅 (`"{n} words"`) | `fn word_count_label` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.word_count` 노드 emit | `grep -n 'story.word_count'` |

## 필요한 변경 (의도 단위)

### 1. 헤더 배지 업데이트
- **입력**: 매 repaint, `total_word_count()` 반환값
- **처리**: `paint`에서 헤더 우측에 라운드 rect 배지(`theme.border` 배경) 렌더. 내부에 `word_count_label(total_word_count())` 텍스트 표시. 위치: `(width - 180.0, 10.0)` ~ `(width - 16.0, 38.0)`.
- **출력/사이드 이펙트**: 항상 최신 단어 수 표시.
- **순서/우선순위**: 헤더 액션 버튼 렌더 이후.

### 2. Status bar 단어 수 동기화
- **입력**: 매 repaint, `chapter_word_count()`, `total_word_count()`
- **처리**: status bar 포맷 문자열에 `{chapter_word_count} words this chapter`와 `{total_word_count} total` 포함.
- **출력/사이드 이펙트**: 두 위치에서 동일 단어 수 참조.
- **순서/우선순위**: status bar 렌더 시.

### 3. 콘텐츠 변경 시 자동 반영
- **입력**: `append_text`, `backspace`, `newline`, `add_chapter`, `delete_current_chapter`
- **처리**: 엔진의 챕터 콘텐츠가 변경되면 다음 repaint에서 `total_word_count()`와 `chapter_word_count()`가 새 값 반환. 별도 캐시 무효화 불필요 — 매 호출마다 엔진에서 재계산.
- **출력/사이드 이펙트**: 단어 수 즉시 갱신.
- **순서/우선순위**: 텍스트 편집 후 repaint 사이클에서 자동 반영.

### 4. 자동화 노드
- **입력**: 항상
- **처리**: `story.word_count` Status 노드를 배지 위치에 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 헤더 버튼 노드 이후.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.word_count` | Status | `"Word count"` | 항상 |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-status-bar-sync-behavior` (동일 단어 수 데이터).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'word_count\|total_word_count' mod.rs state.rs export.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
