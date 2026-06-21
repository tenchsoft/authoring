# Implement: composer-text-input

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 작성기 입력 필드에 텍스트를 입력할 수 있다.
- design: 하단 작성기 바의 둥근 입력 필드, ACCENT_UNIVERSE 테두리.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::composer_input_rect` | 필드 rect | `fn composer_input_rect` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_composer` | 입력 필드 렌더링 | `fn paint_composer` |
| `apps/universe/src-tauri/src/ui/state.rs::focus_composer` | 포커스 전환 | `fn focus_composer` |
| `apps/universe/src-tauri/src/ui/state.rs::push_input_text` | 텍스트 입력 | `UniverseInputFocus::Composer` |

## 필요한 변경
### 1. 입력 필드 포커스
- **입력**: composer_input_rect 내 클릭 → `UniverseHit::ComposerInput`
- **처리**: `state.focus_composer()` → input_focus를 Composer로 전환
- **출력**: 키보드 입력이 input_text로 라우팅

### 2. 텍스트 입력 및 표시
- **입력**: Composer 포커스 상태에서 키 입력
- **처리**: `push_input_text`가 input_text에 append, 빈 문자열이면 placeholder 표시
- **출력**: 입력 필드에 텍스트 표시

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.composer.input` | `textbox` | 입력 텍스트 | 항상 |

## 의존
- 선행 implement: 없음
