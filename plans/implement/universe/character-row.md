# Implement: character-row

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 캐릭터 행을 클릭하면 해당 캐릭터가 선택된다.
- design: 좌측 패널 캐릭터 목록 행, 선택 시 좌측 ACCENT_UNIVERSE 바 + NEUTRAL_600 배경.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::character_rect` | 행 rect | `fn character_rect` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_left_panel` | 캐릭터 목록 렌더링 | `active_character_idx` |
| `apps/universe/src-tauri/src/ui/state.rs::select_character` | 캐릭터 선택 | `fn select_character` |

## 필요한 변경
### 1. 캐릭터 선택
- **입력**: character_rect 내 클릭 → `UniverseHit::Character(index)`
- **처리**: `state.select_character(index)` → active_character_idx 갱신, toast
- **출력**: 선택 캐릭터 하이라이트, repaint

### 2. 캐릭터 행 렌더링
- **입력**: characters 리스트, active_character_idx
- **처리**: 활성 캐릭터에 좌측 바 + 배경색, 아바타(첫 글자), 이름, 무드 색 점, 메모리 수
- **출력**: 좌측 패널 캐릭터 목록

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.character.{idx}` | `button` | 캐릭터 이름 | 항상 |
| `universe.character.selected` | `status` | 선택된 캐릭터 이름 | 해당 캐릭터가 선택됨 |

## 의존
- 선행 implement: 없음
- 영향 받는 implement: `automatic-character-selection-highlight`
