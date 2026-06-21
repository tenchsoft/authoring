# Implement: automatic-status-bar-sync-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Status bar가 `saved_at`, 챕터 번호, 챕터 단어 수, 전체 단어 수, focus mode 상태를 항상 최신으로 반영한다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::paint` | status bar 텍스트 포맷 | `grep -n 'status_bar\|autosaved' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::saved_at` | 마지막 저장 시간 필드 | `pub saved_at` |
| `apps/story/src-tauri/src/ui/state.rs::chapter_word_count` | 현재 챕터 단어 수 | `fn chapter_word_count` |
| `apps/story/src-tauri/src/ui/state.rs::total_word_count` | 전체 단어 수 | `fn total_word_count` |
| `apps/story/src-tauri/src/ui/state.rs::selected_chapter_idx` | 챕터 번호 (1-based) | `pub selected_chapter_idx` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.status_bar` 노드 emit | `grep -n 'story.status_bar'` |

## 필요한 변경 (의도 단위)

### 1. Status bar 텍스트 동기화
- **입력**: 매 repaint, `state`의 현재 필드 값
- **처리**: 포맷 문자열: `"autosaved {saved_at}    {chapter_word_count} words this chapter    Ch {selected_chapter_idx + 1:02}    {total_word_count} total{focus_mode ? "    focus mode" : ""}"`. `theme.surface` 배경, `theme.secondary` 텍스트, 11.0 크기.
- **출력/사이드 이펙트**: 항상 최신 상태 반영.
- **순서/우선순위**: 모든 패널/에디터 렌더 이후, 오버레이 렌더 전.

### 2. saved_at 갱신 트리거
- **입력**: Save 버튼 클릭, Open, Export 포맷 선택
- **처리**: `save()` → `saved_at = "now"`. `open_project()` → `saved_at = "opened"`. Export 포맷 클릭 → `saved_at = "exported {format}"`.
- **출력/사이드 이펙트**: status bar 텍스트 즉시 갱신.
- **순서/우선순위**: 해당 액션 수행 직후.

### 3. 자동화 노드
- **입력**: 항상
- **처리**: `story.status_bar` Status 노드를 하단 rect에 emit. rect: `(0.0, height - 24.0)` ~ `(width, height)`.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: cursor 노드 이후.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.status_bar` | Status | `"Status bar"` | 항상 |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-word-count-badge-behavior` (단어 수 중복 표시), `automatic-dirty-title-indicator-behavior` (saved_at 연동).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'autosaved\|status_bar\|saved_at' mod.rs state.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
