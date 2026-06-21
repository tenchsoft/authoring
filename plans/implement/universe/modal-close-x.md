# Implement: modal-close-x

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 모달의 X 버튼을 클릭하면 모달이 닫힌다.
- design: 모달 우측 상단 X 버튼.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/editor.rs::modal_close_rect` | 버튼 rect | `fn modal_close_rect` |
| `apps/universe/src-tauri/src/ui/state.rs::close_modals` | 모달 닫기 | `fn close_modals` |
| `apps/universe/src-tauri/src/ui/mod.rs::on_pointer_event` | CloseModal 히트 | `UniverseHit::CloseModal` |

## 필요한 변경
### 1. X 버튼 클릭
- **입력**: modal_close_rect 내 클릭 → `UniverseHit::CloseModal`
- **처리**: `state.close_modals()` → 모든 show_* 플래그 false
- **출력**: 모달 닫힘, repaint

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.modal.close` | `button` | `"Close"` | 모달 열림 |

## 의존
- 선행 implement: 없음
