# Implement: chat-mode-tab

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 채팅 모드 탭을 클릭하면 채팅 모드로 전환된다.
- design: 헤더 내 탭 버튼, 활성 시 모드 accent 색상 텍스트, 비활성 시 NEUTRAL_300.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/state.rs::UniverseMode` | 모드 enum 정의 | `enum UniverseMode` |
| `apps/universe/src-tauri/src/ui/state.rs::set_mode` | 모드 전환 메서드 | `fn set_mode` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_header` | 탭 렌더링 | `fn paint_header` |
| `apps/universe/src-tauri/src/ui/chat.rs::mode_rect` | 탭 rect 계산 | `fn mode_rect` |
| `apps/universe/src-tauri/src/ui/mod.rs::on_pointer_event` | Mode(UniverseMode::Chat) 히트 | `UniverseHit::Mode` |

## 필요한 변경
### 1. 모드 전환
- **입력**: `UniverseHit::Mode(UniverseMode::Chat)` 클릭 이벤트
- **처리**: `state.set_mode(UniverseMode::Chat)` 호출 → mode 필드 갱신, toast 메시지 설정
- **출력**: mode 변경, repaint 요청

### 2. 탭 렌더링
- **입력**: 현재 `state.mode` 값
- **처리**: `mode == Chat`이면 배경 NEUTRAL_600 + accent 색상 텍스트, 아니면 NEUTRAL_300 텍스트
- **출력**: 헤더 내 탭 버튼 시각 표현

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.mode.chat` | `tab` | `"Chat"` | 항상 |

## 의존
- 선행 implement: 없음
- 영향 받는 implement: `automatic-active-mode-content-render`
