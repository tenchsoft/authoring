# Implement: left-panel-settings

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 좌측 패널 설정 버튼을 클릭하면 설정 모달이 열린다.
- design: 좌측 패널 하단 "Settings" 버튼.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::left_settings_rect` | 버튼 rect | `fn left_settings_rect` |
| `apps/universe/src-tauri/src/ui/state.rs::open_settings` | 모달 열기 | `fn open_settings` |

## 필요한 변경
### 1. 버튼 클릭
- **입력**: left_settings_rect 내 클릭 → `UniverseHit::Settings`
- **처리**: `state.open_settings()` → `show_settings = true`
- **출력**: 설정 모달 표시

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.left.settings` | `button` | `"Left settings"` | 항상 |

## 의존
- 선행 implement: 없음
- 영향 받는 implement: `automatic-modal-exclusivity`
