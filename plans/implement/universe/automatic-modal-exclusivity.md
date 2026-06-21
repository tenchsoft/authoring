# Implement: automatic-modal-exclusivity

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 모달은 동시에 하나만 열 수 있다.
- background: open_* 메서드 호출 시 기존 모달 자동 닫기.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/state.rs::open_character_editor` | 모달 열기 | `fn open_character_editor` |
| `apps/universe/src-tauri/src/ui/state.rs::open_persona_editor` | 모달 열기 | `fn open_persona_editor` |
| `apps/universe/src-tauri/src/ui/state.rs::open_sessions` | 모달 열기 | `fn open_sessions` |
| `apps/universe/src-tauri/src/ui/state.rs::open_template_picker` | 모달 열기 | `fn open_template_picker` |
| `apps/universe/src-tauri/src/ui/state.rs::open_settings` | 모달 열기 | `fn open_settings` |
| `apps/universe/src-tauri/src/ui/state.rs::close_modals` | 모달 닫기 | `fn close_modals` |

## 필요한 변경
### 1. 모달 배타성 보장
- **입력**: 임의의 open_* 메서드 호출
- **처리**: 각 open_* 메서드 내에서 `self.close_modals()`를 먼저 호출한 후 해당 show_* 플래그를 true로 설정
- **출력**: 항상 최대 하나의 모달만 열림

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.modal.surface` | `dialog` | `"Modal"` | 모달 열림 |

## 의존
- 선행 implement: 없음
