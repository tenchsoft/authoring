# Implement: automatic-composer-placeholder

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 입력이 없으면 플레이스홀더 텍스트가 자동으로 표시된다.
- background: input_text 빈 문자열 여부에 따라 자동 트리거.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::paint_composer` | placeholder 표시 | `"Type a message..."` |
| `apps/universe/src-tauri/src/ui/mod.rs::universe_automation_nodes` | placeholder 노드 | `universe.composer.placeholder` |

## 필요한 변경
### 1. placeholder 렌더링
- **입력**: `state.input_text.is_empty()`
- **처리**: 빈 문자열이면 NEUTRAL_400 색상으로 "Type a message..." 표시, 아니면 input_text를 NEUTRAL_100으로 표시
- **출력**: composer 입력 필드에 조건부 텍스트

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.composer.placeholder` | `text` | `"Type a message"` | input_text 빈 문자열 |

## 의존
- 선행 implement: `composer-text-input`
