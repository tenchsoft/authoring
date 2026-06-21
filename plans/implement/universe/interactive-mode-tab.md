# Implement: interactive-mode-tab

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 인터랙티브 모드 탭을 클릭하면 인터랙티브 스토리 모드로 전환된다.
- design: 헤더 내 세 번째 탭, 활성 시 STATUS_WARNING 텍스트.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/state.rs::UniverseMode::Interactive` | 모드 변형 | `Interactive` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_interactive` | 인터랙티브 렌더링 | `fn paint_interactive` |
| `apps/universe/src-tauri/src/ui/chat.rs::choice_rect` | 선택지 rect | `fn choice_rect` |

## 필요한 변경
### 1. 모드 전환
- **입력**: `UniverseHit::Mode(UniverseMode::Interactive)` 클릭
- **처리**: `state.set_mode(UniverseMode::Interactive)`
- **출력**: mode 변경, 선택지 버튼 표시

### 2. 인터랙티브 렌더링
- **입력**: `interactive_blocks` 데이터
- **처리**: 시나리오 텍스트 + 4개 선택지 버튼 렌더링, 선택 시 ACCENT_UNIVERSE 하이라이트
- **출력**: center 패널에 인터랙티브 콘텐츠

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.mode.interactive` | `tab` | `"Interactive"` | 항상 |

## 의존
- 선행 implement: 없음
- 영향 받는 implement: `automatic-interactive-choice-selection-render`
