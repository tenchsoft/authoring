# Implement: control-f-search-shortcut-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Ctrl+F 키보드 단축키로 검색 오버레이를 토글한다. 이미 열려 있으면 닫고, 닫혀 있으면 연다.
- design: search overlay 토글 동작.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_text_event` | Ctrl+F 분기가 `state.toggle_search()` 호출 | `grep -n 'LogicalKey::Character'` 또는 `fn on_text_event` |
| `apps/story/src-tauri/src/ui/state.rs::toggle_search` | `show_search` 토글, `input_focus` 전환 | `fn toggle_search` |

## 필요한 변경 (의도 단위)

### 1. 키보드 이벤트 핸들러에 Ctrl+F 분기
- **입력**: `TextEvent::Keyboard` 이벤트, `logical_key == Character("f")` + `modifiers.control == true`, `is_pressed == true`, `is_repeat == false`
- **처리**: `state.toggle_search()` 호출. 이미 구현된 분기(`grep -n 'ch == "f"'`)가 있으므로 해당 분기가 올바르게 동작하는지 확인만 한다.
- **출력/사이드 이펙트**: `show_search` 토글, `input_focus` 가 `Search` 또는 `Manuscript` 로 전환, `ctx.request_paint()`
- **순서/우선순위**: 다른 Ctrl+문자 분기들과 동일 우선순위. `Escape` 분기 이후, 일반 `Character` 분기 이전에 위치.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| (없음 — 기존 `story.search.bar`, `story.search.query`, `story.search.case_sensitive` 노드 사용) | | | |

## 의존
- 선행 implement: 없음 (검색 오버레이 노드는 이미 `story_automation_nodes`에 존재)

## 작업 절차
1. spec/design/background 읽기
2. `grep -n 'ch == "f"' apps/story/src-tauri/src/ui/mod.rs`로 Ctrl+F 분기 위치 확정
3. `grep -n 'fn toggle_search' apps/story/src-tauri/src/ui/state.rs`로 상태 메서드 확인
4. 의도대로 코드 변경 (필요 시)
5. `cargo check --workspace --locked` 통과 확인
