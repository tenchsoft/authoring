# Implement: pinned-memory-row

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 고정 메모리 행을 클릭하면 해당 메모리가 선택된다.
- design: 우측 패널 메모리 행, 좌측 ACCENT_UNIVERSE 바.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::memory_rect` | 행 rect | `fn memory_rect` |
| `apps/universe/src-tauri/src/ui/state.rs::select_memory` | 메모리 선택 | `fn select_memory` |

## 필요한 변경
### 1. 메모리 선택
- **입력**: memory_rect 내 클릭 → `UniverseHit::Memory(index)`
- **처리**: `state.select_memory(index)` → selected_memory 갱신, toast
- **출력**: 선택 메모리 표시, repaint

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.memory.{idx}` | `button` | `"Pinned memory"` | 항상 |

## 의존
- 선행 implement: 없음
