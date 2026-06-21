# Implement: automatic-active-mode-content-render

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 현재 모드에 따라 center 패널 콘텐츠가 자동으로 전환된다.
- background: mode 변경 시 자동 트리거, paint_center_panel에서 mode 분기.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::paint_center_panel` | 모드 분기 렌더링 | `fn paint_center_panel` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_chat_messages` | Chat 렌더링 | `fn paint_chat_messages` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_novel` | Novel 렌더링 | `fn paint_novel` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_interactive` | Interactive 렌더링 | `fn paint_interactive` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_script` | Script 렌더링 | `fn paint_script` |

## 필요한 변경
### 1. 모드 분기 렌더링
- **입력**: `state.mode` 값 변경
- **처리**: `paint_center_panel`에서 match 분기 → 각 모드별 paint 함수 호출
- **출력**: center 패널에 모드에 맞는 콘텐츠 렌더링
- **순서**: composer 렌더링 전에 수행

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.active_content` | `region` | `"Active mode content"` | 항상 |

## 의존
- 선행 implement: `chat-mode-tab`, `novel-mode-tab`, `interactive-mode-tab`, `script-mode-tab`
