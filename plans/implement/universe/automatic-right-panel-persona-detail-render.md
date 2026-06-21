# Implement: automatic-right-panel-persona-detail-render

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 캐릭터 선택 시 우측 패널에 페르소나 상세 정보가 자동으로 렌더링된다.
- background: 선택된 캐릭터 변경 시 우측 패널 내용 자동 업데이트.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::render` | 우측 패널 렌더 | `right_panel` |
| `apps/universe/src-tauri/src/ui/state.rs::UniverseState` | selected_character | `selected_character` |

## 필요한 변경
### 1. 페르소나 상세 자동 렌더
- **입력**: selected_character 상태 변경
- **처리**: render에서 selected_character가 Some이면 우측 패널에 페르소나 이름, 설명, 핀 메모리 목록 렌더
- **출력**: 선택된 캐릭터의 페르소나 상세가 우측 패널에 표시

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.right-panel.persona-detail` | `region` | `"Persona Detail"` | 캐릭터 선택됨 |

## 의존
- 선행 implement: `character-row` (선택 트리거)
