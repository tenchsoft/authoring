# Implement: interactive-choice-call-ari-over

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: "Call Ari over" 선택지를 클릭하면 해당 선택이 기록된다.
- design: 인터랙티브 모드의 두 번째 선택지 버튼.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::choice_rect` | 선택지 rect (index=1) | `fn choice_rect` |
| `apps/universe/src-tauri/src/ui/state.rs::choose_interactive` | 선택 처리 | `fn choose_interactive` |

## 필요한 변경
### 1. 선택지 클릭
- **입력**: choice_rect(1, center) 내 클릭 → `UniverseHit::Choice(1)`
- **처리**: `state.choose_interactive(1)` → selected = Some(1)
- **출력**: 선택지 하이라이트, toast

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.choice.call_ari_over` | `button` | `"call_ari_over"` | Interactive 모드 |

## 의존
- 선행 implement: `interactive-mode-tab`
