# Implement: automatic-overlay-exclusivity-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Export modal, command palette, search bar는 동시에 열 수 없다. 하나를 열면 다른 것들이 닫힌다.
- Escape 키가 모든 오버레이를 닫는다.
- 어느 하나라도 열려 있으면 `story.overlay.exclusive` 노드가 노출된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/state.rs::open_export` | `show_export = true`, `show_command_palette = false`, `show_search = false` | `fn open_export` |
| `apps/story/src-tauri/src/ui/state.rs::toggle_command_palette` | `show_command_palette` 토글, 열 때 `show_export = false`, `show_search = false` | `fn toggle_command_palette` |
| `apps/story/src-tauri/src/ui/state.rs::toggle_search` | `show_search` 토글, `input_focus` 전환 | `fn toggle_search` |
| `apps/story/src-tauri/src/ui/state.rs::close_overlays` | 세 플래그 모두 false, `input_focus = Manuscript` | `fn close_overlays` |
| `apps/story/src-tauri/src/ui/mod.rs::on_text_event` | Escape 키 → `close_overlays()` | `grep -n 'Escape' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.overlay.exclusive` 노드 emit | `grep -n 'story.overlay.exclusive'` |

## 필요한 변경 (의도 단위)

### 1. 오버레이 상호배타적 열기
- **입력**: Export/Cmd/Search 버튼 클릭 또는 단축키
- **처리**: `open_export` → 다른 두 플래그 false. `toggle_command_palette` → 열 때 다른 두 플래그 false. `toggle_search`는 현재 다른 플래그를 명시적으로 닫지 않음 (필요시 추가).
- **출력/사이드 이펙트**: 항상 최대 1개 오버레이만 열림.
- **순서/우선순위**: 각 토글 함수 내에서 즉시 처리.

### 2. Escape로 전체 닫기
- **입력**: Escape 키 down
- **처리**: `close_overlays()` 호출 → `show_export = false`, `show_command_palette = false`, `show_search = false`, `input_focus = Manuscript`.
- **출력/사이드 이펙트**: 모든 오버레이 닫힘, repaint.
- **순서/우선순위**: 키보드 이벤트 핸들링 최우선 (다른 키 처리 전).

### 3. Backdrop 클릭으로 개별 닫기
- **입력**: pointer down, 오버레이 영역 밖 클릭
- **처리**: 각 오버레이의 hit-test에서 backdrop 클릭 감지 시 `close_overlays()` 호출.
- **출력/사이드 이펙트**: 해당 오버레이 닫힘, repaint.
- **순서/우선순위**: 오버레이 hit-test가 다른 hit-test보다 우선.

### 4. 자동화 노드
- **입력**: `show_export || show_command_palette || show_search`
- **처리**: `story.overlay.exclusive` Status 노드를 전체 화면 rect에 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 모든 오버레이 노드 이후 마지막.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.overlay.exclusive` | Status | `"Overlay exclusive"` | `show_export || show_command_palette || show_search` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-export-modal-render-behavior`, `automatic-command-palette-render-behavior`, `automatic-search-bar-render-behavior` (모두 동일 오버레이 메커니즘).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'close_overlays\|show_export\|show_command_palette\|show_search' state.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
