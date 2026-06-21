# Implement: manuscript-backspace-key-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- 원고 편집기에 포커스가 있을 때 Backspace 키를 누르면 선택된 챕터의 마지막 문자가 삭제된다.
- design: 원고 편집기 텍스트 입력 제어.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_text_event` | `NamedKey::Backspace` 분기에서 `input_focus == Manuscript`일 때 `state.backspace()` 호출 | `grep -n 'NamedKey::Backspace'` 또는 `fn on_text_event` |
| `apps/story/src-tauri/src/ui/state.rs::backspace` | 현재 챕터의 `content.to_plain_text()`에서 마지막 문자 제거 후 `engine.set_chapter_content()` | `fn backspace` |
| `crates/story-core/src/engine.rs` | `StoryEngine::set_chapter_content` | `pub fn set_chapter_content` |

## 필요한 변경 (의도 단위)

### 1. 키보드 이벤트 핸들러에 Backspace 분기
- **입력**: `TextEvent::Keyboard` 이벤트, `logical_key == NamedKey::Backspace`, `is_pressed == true`, `is_repeat == false`
- **처리**: `state.input_focus`가 `StoryInputFocus::Manuscript`이면 `state.backspace()` 호출. `StoryInputFocus::Search`이면 `state.backspace_search()` 호출. 이미 구현된 분기가 있으므로 올바르게 동작하는지 확인만 한다.
- **출력/사이드 이펙트**: 현재 챕터 텍스트에서 마지막 문자 제거, 엔진 dirty 상태, `ctx.request_paint()`
- **순서/우선순위**: `Escape` 분기 이후, `Enter` 분기 이후, 일반 `Character` 분기 이전.

### 2. state.backspace 동작 확인
- **입력**: 현재 선택된 챕터 인덱스
- **처리**: 챕터의 `content.to_plain_text()`에서 `text.pop()` 후 `TenchDocument::plain_text(&text)`로 새 content 생성, `engine.set_chapter_content()` 호출.
- **출력/사이드 이펙트**: 챕터 content 갱신, 엔진 undo 스택에 기록.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| (없음 — 키보드 입력은 자동화 노드 불필요. 기존 `story.manuscript.editor` 노드 사용) | | | |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design/background 읽기
2. `grep -n 'NamedKey::Backspace' apps/story/src-tauri/src/ui/mod.rs`로 Backspace 분기 위치 확정
3. `grep -n 'fn backspace' apps/story/src-tauri/src/ui/state.rs`로 상태 메서드 확인
4. 의도대로 코드 변경 (필요 시)
5. `cargo check --workspace --locked` 통과 확인
