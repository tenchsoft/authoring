# Implement: modal-backdrop-dismiss

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 모달 외부(백드롭)를 클릭하면 모달이 닫힌다.
- design: 모달 바깥 반투명 영역.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::hit_test` | 백드롭 히트 테스트 | `!modal_rect.contains(pos)` |
| `apps/universe/src-tauri/src/ui/state.rs::close_modals` | 모달 닫기 | `fn close_modals` |

## 필요한 변경
### 1. 백드롭 클릭
- **입력**: 모달 열림 상태에서 modal_rect 바깥 클릭
- **처리**: hit_test에서 `!editor::modal_rect(size).contains(pos)` → `UniverseHit::CloseModal`
- **출력**: 모달 닫힘, repaint

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.modal.backdrop` | `button` | `"Modal backdrop"` | 모달 열림 |

## 의존
- 선행 implement: 없음
