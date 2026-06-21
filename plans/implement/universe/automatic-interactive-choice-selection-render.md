# Implement: automatic-interactive-choice-selection-render

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 선택한 인터랙티브 선택지에 하이라이트가 자동으로 표시된다.
- background: interactive_blocks.selected 변경 시 자동 트리거.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::paint_interactive` | 선택 하이라이트 | `block.selected` |
| `apps/universe/src-tauri/src/ui/mod.rs::universe_automation_nodes` | selected 노드 | `universe.choice.selected` |

## 필요한 변경
### 1. 선택 하이라이트 렌더링
- **입력**: `interactive_blocks[0].selected` 값
- **처리**: selected == Some(index)인 선택지에 NEUTRAL_600 배경 + ACCENT_UNIVERSE 테두리 + 텍스트
- **출력**: 선택된 선택지 시각적 구분

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.choice.selected` | `status` | `"Selected choice"` | Interactive 모드 + 선택됨 |

## 의존
- 선행 implement: `interactive-mode-tab`
