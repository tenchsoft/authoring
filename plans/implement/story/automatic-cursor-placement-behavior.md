# Implement: automatic-cursor-placement-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 커서가 렌더된 텍스트 콘텐츠 바로 뒤에 나타난다.
- 텍스트 입력(append/backspace/newline) 시 커서가 함께 이동한다.
- 커서는 에디터 카드 경계 내에서만 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/editor.rs::paint_chapter_content` | 텍스트 라인 렌더 후 마지막 cursor_y 반환 | `fn paint_chapter_content` |
| `apps/story/src-tauri/src/ui/editor.rs::paint_cursor` | 2px 너비 커서 rect 페인트 | `fn paint_cursor` |
| `apps/story/src-tauri/src/ui/mod.rs::paint` | cursor_y < max_y 조건으로 커서 표시 여부 결정 | `grep -n 'cursor_y\|paint_cursor' mod.rs` |
| `apps/story/src-tauri/src/ui/state.rs::append_text` | 텍스트 추가 → 다음 repaint 시 cursor_y 증가 | `fn append_text` |
| `apps/story/src-tauri/src/ui/state.rs::backspace` | 텍스트 삭제 → cursor_y 감소 가능 | `fn backspace` |
| `apps/story/src-tauri/src/ui/state.rs::newline` | `\n` 추가 → cursor_y 22.0 증가 | `fn newline` |

## 필요한 변경 (의도 단위)

### 1. 커서 위치 계산
- **입력**: `chapter_text()` 결과, 시작 y(`panel_y + 68.0`), 최대 y(`panel_y + panel_h - 40.0`)
- **처리**: `paint_chapter_content`가 텍스트를 `\n`으로 분리하여 각 라인을 22.0 간격으로 렌더. 마지막 렌더된 라인 아래의 `cursor_y` 반환.
- **출력/사이드 이펙트**: 반환된 `cursor_y` 값.
- **순서/우선순위**: 챕터 타이틀 렌더 이후, 커서 페인트 이전.

### 2. 커서 표시 조건
- **입력**: `cursor_y < panel_y + panel_h - 40.0` (에디터 카드 하단 경계)
- **처리**: 조건 충족 시 `paint_cursor` 호출. rect: `(x, cursor_y - 12.0)` ~ `(x + 2.0, cursor_y + 4.0)`. 색상 `theme.primary`.
- **출력/사이드 이펙트**: 커서 시각적 표시.
- **순서/우선순위**: 챕터 콘텐츠 렌더 직후.

### 3. 텍스트 입력에 따른 커서 이동
- **입력**: 키보드 이벤트 (Character, Backspace, Enter)
- **처리**: `append_text` → 텍스트 길이 증가 → 다음 repaint에서 `paint_chapter_content`가 더 긴 텍스트를 렌더 → `cursor_y` 증가. `backspace` → 반대. `newline` → `\n` 추가로 라인 증가 → `cursor_y` 22.0 점프.
- **출력/사이드 이펙트**: `cursor_y` 값 변화, 커서 위치 이동.
- **순서/우선순위**: 텍스트 편집 후 repaint 사이클에서 자동 반영.

### 4. 자동화 노드
- **입력**: 항상
- **처리**: `story.cursor` Status 노드를 에디터 영역 내에 emit. rect: `(left_w + 28.0, panel_y + 56.0)` ~ `(left_w + 32.0, panel_y + 74.0)`.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: manuscript editor 노드 이후.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.cursor` | Status | `"Cursor"` | 항상 |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: 없음.

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'cursor_y\|paint_cursor' mod.rs editor.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
