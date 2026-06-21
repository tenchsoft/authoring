# Implement: header-settings-icon

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 헤더의 설정 아이콘을 클릭하면 설정 모달이 열린다.
- design: 헤더 우측 톱니바퀴 아이콘 버튼.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::settings_rect` | 버튼 rect | `fn settings_rect` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_header` | 아이콘 렌더링 | `"⚙"` |
| `apps/universe/src-tauri/src/ui/state.rs::open_settings` | 모달 열기 | `fn open_settings` |

## 필요한 변경
### 1. 설정 버튼 클릭
- **입력**: settings_rect 내 클릭 → `UniverseHit::Settings`
- **처리**: `state.open_settings()` → 기존 모달 닫고 `show_settings = true`
- **출력**: 설정 모달 표시, repaint

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.header.settings` | `button` | `"Settings"` | 항상 |

## 의존
- 선행 implement: 없음
- 영향 받는 implement: `automatic-modal-exclusivity`
