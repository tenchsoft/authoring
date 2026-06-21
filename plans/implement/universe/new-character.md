# Implement: new-character

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 새 캐릭터 버튼을 클릭하면 캐릭터 편집 모달이 열린다.
- design: 좌측 패널 "+ New" 버튼, primary 스타일.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::new_character_rect` | 버튼 rect | `fn new_character_rect` |
| `apps/universe/src-tauri/src/ui/state.rs::open_character_editor` | 모달 열기 | `fn open_character_editor` |

## 필요한 변경
### 1. 버튼 클릭
- **입력**: new_character_rect 내 클릭 → `UniverseHit::NewCharacter`
- **처리**: `state.open_character_editor()` → 기존 모달 닫고 `show_character_editor = true`
- **출력**: 캐릭터 편집 모달 표시

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.character.new` | `button` | `"New character"` | 항상 |

## 의존
- 선행 implement: 없음
- 영향 받는 implement: `automatic-modal-exclusivity`
