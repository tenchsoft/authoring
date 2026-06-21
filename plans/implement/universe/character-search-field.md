# Implement: character-search-field

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 캐릭터 검색 필드에 텍스트를 입력하면 캐릭터 목록이 필터링된다.
- design: 좌측 패널 상단 둥근 입력 필드.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::search_rect` | 필드 rect | `fn search_rect` |
| `apps/universe/src-tauri/src/ui/state.rs::focus_search` | 포커스 전환 | `fn focus_search` |
| `apps/universe/src-tauri/src/ui/state.rs::push_input_text` | 텍스트 입력 | `UniverseInputFocus::CharacterSearch` |

## 필요한 변경
### 1. 검색 필드 포커스
- **입력**: search_rect 내 클릭 → `UniverseHit::Search`
- **처리**: `state.focus_search()` → input_focus를 CharacterSearch로 전환
- **출력**: 키보드 입력이 search_query로 라우팅

### 2. 검색어 입력
- **입력**: CharacterSearch 포커스 상태에서 키 입력
- **처리**: `push_input_text`가 search_query에 append
- **출력**: 검색 필드에 텍스트 표시

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.character.search` | `textbox` | 검색어 | 항상 |

## 의존
- 선행 implement: 없음
