# Implement: escape-close-overlays-shortcut-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Escape 키를 누르면 열려 있는 모든 오버레이(익스포트 모달, 커맨드 팔레트, 검색 바)를 닫는다.
- design: 오버레이 닫기 단축키.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_text_event` | `NamedKey::Escape` 분기가 `state.close_overlays()` 호출 | `grep -n 'NamedKey::Escape'` 또는 `fn on_text_event` |
| `apps/story/src-tauri/src/ui/state.rs::close_overlays` | `show_export`, `show_command_palette`, `show_search` 모두 false로 설정, `input_focus`를 `Manuscript`으로 | `fn close_overlays` |

## 필요한 변경 (의도 단위)

### 1. 키보드 이벤트 핸들러에 Escape 분기
- **입력**: `TextEvent::Keyboard` 이벤트, `logical_key == NamedKey::Escape`, `is_pressed == true`, `is_repeat == false`
- **처리**: `state.close_overlays()` 호출. 이미 구현된 분기(`grep -n 'NamedKey::Escape'`)가 있으므로 해당 분기가 올바르게 동작하는지 확인만 한다.
- **출력/사이드 이펙트**: `show_export = false`, `show_command_palette = false`, `show_search = false`, `input_focus = Manuscript`, `ctx.request_paint()`
- **순서/우선순위**: 모든 키보드 분기 중 가장 먼저 평가됨. 다른 단축키보다 우선.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| (없음 — 키보드 단축키는 자동화 노드 불필요. 기존 `story.overlay.exclusive` 노드가 사라짐) | | | |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design/background 읽기
2. `grep -n 'NamedKey::Escape' apps/story/src-tauri/src/ui/mod.rs`로 Escape 분기 위치 확정
3. `grep -n 'fn close_overlays' apps/story/src-tauri/src/ui/state.rs`로 상태 메서드 확인
4. 의도대로 코드 변경 (필요 시)
5. `cargo check --workspace --locked` 통과 확인
