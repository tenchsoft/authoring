# Implement: automatic-character-selection-highlight

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 선택된 캐릭터에 하이라이트가 자동으로 표시된다.
- background: active_character_idx 변경 시 자동 트리거.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::paint_left_panel` | 선택 캐릭터 렌더링 | `active_character_idx` |
| `apps/universe/src-tauri/src/ui/mod.rs::universe_automation_nodes` | selected 노드 | `universe.character.selected` |

## 필요한 변경
### 1. 선택 하이라이트 렌더링
- **입력**: `state.active_character_idx` 값
- **처리**: `paint_left_panel`에서 index == active_character_idx인 캐릭터에 NEUTRAL_600 배경 + 좌측 ACCENT_UNIVERSE 바
- **출력**: 선택 캐릭터 시각적 구분

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.character.selected` | `status` | 캐릭터 이름 | 해당 캐릭터가 선택됨 |

## 의존
- 선행 implement: `character-row`
