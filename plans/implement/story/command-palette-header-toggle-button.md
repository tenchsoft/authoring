# Implement: command-palette-header-toggle-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/story/command-palette-header-toggle-button.md`): Cmd 버튼 클릭 시 커맨드 팔레트 토글, export 모달 닫힘, Cmd 버튼 활성 상태 변경.
- design(`plans/design/story/command-palette-header-toggle-button.md`): Header action bar의 Cmd 토글 버튼.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` (header "Cmd" 분기) | `toggle_command_palette` 호출 | `"Cmd"` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::paint` (Cmd 버튼 active 상태) | `show_command_palette` 시 active 색상 | `show_command_palette` 검색 |
| `apps/story/src-tauri/src/ui/state.rs::toggle_command_palette` | 배타적 오버레이 처리 | `fn toggle_command_palette` 검색 |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | `story.header.command` 노드 | `story.header.command` 검색 |

## 필요한 변경 (의도 단위)
### 1. Cmd 버튼 클릭 처리
- **입력**: 포인터 다운 이벤트, y < 48.0, "Cmd" 버튼 rect 내부
- **처리**: `state.toggle_command_palette()` 호출. 이 함수는 `show_command_palette`를 토글하고, 열릴 때 `show_export = false`, `show_search = false`로 설정.
- **출력/사이드 이펙트**: 팔레트 열림/닫힘, export 닫힘, `request_paint()`
- **순서/우선순위**: export 모달이 열려 있으면 팔레트 클릭이 먼저 처리됨

### 2. Cmd 버튼 활성 상태 렌더링
- **입력**: `paint()` 호출 시 `self.state.show_command_palette` 값
- **처리**: `show_command_palette == true`면 버튼 배경을 `theme.primary`로, 텍스트를 `theme.on_primary`로 렌더링
- **출력/사이드 이펙트**: 버튼 시각적 피드백
- **순서/우선순위**: 다른 header 버튼과 동일한 렌더링 로직

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.header.command` | `Button` | `"Command"` | 항상 |

## 의존
- 선행 implement: 없음 (헤더 기본 버튼)

## 작업 절차
1. spec/design 읽기
2. grep으로 `"Cmd"`, `toggle_command_palette` 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
